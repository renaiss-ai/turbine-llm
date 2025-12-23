use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Result, TurbineError},
    models::{LLMRequest, LLMResponse, Message},
    types::{OutputFormat, Provider},
};

use super::LLMProviderTrait;

pub struct AnthropicProvider {
    api_key: String,
    base_url: String,
}

impl AnthropicProvider {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var(Provider::Anthropic.env_var())?;
        Ok(Self {
            api_key,
            base_url: Provider::Anthropic.base_url().to_string(),
        })
    }

    pub fn new_with_key(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: Provider::Anthropic.base_url().to_string(),
        }
    }
}

#[derive(Serialize)]
struct AnthropicRequestBody {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    usage: UsageInfo,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

#[derive(Deserialize)]
struct UsageInfo {
    input_tokens: u32,
    output_tokens: u32,
}

#[async_trait]
impl LLMProviderTrait for AnthropicProvider {
    async fn send_request(&self, request: &LLMRequest) -> Result<LLMResponse> {
        // Filter out system messages (Anthropic doesn't support them in messages array)
        let messages: Vec<Message> = request
            .messages
            .iter()
            .filter(|m| m.role != "system")
            .cloned()
            .collect();

        if messages.is_empty() {
            return Err(TurbineError::MissingField(
                "At least one user or assistant message is required".to_string(),
            ));
        }

        // Build system prompt
        let mut system_prompt = request.system_prompt.clone();

        // For JSON output, add instruction to system prompt and use prefilling
        if request.output_format == OutputFormat::Json {
            let json_instruction = "You must respond with valid JSON only. Start your response with an opening brace {.";
            system_prompt = Some(match system_prompt {
                Some(existing) => format!("{} {}", existing, json_instruction),
                None => json_instruction.to_string(),
            });
        }

        let body = AnthropicRequestBody {
            model: request.model.clone(),
            messages,
            max_tokens: request.max_tokens.unwrap_or(1024),
            system: system_prompt,
            temperature: request.temperature,
            top_p: request.top_p,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(TurbineError::ApiError(error_text));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        let content = anthropic_response
            .content
            .first()
            .ok_or_else(|| TurbineError::InvalidResponse("No content in response".to_string()))?
            .text
            .clone();

        Ok(LLMResponse::new(
            content,
            anthropic_response.usage.input_tokens,
            anthropic_response.usage.output_tokens,
        ))
    }
}
