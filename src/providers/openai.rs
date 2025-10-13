use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Result, TurbineError},
    models::{LLMRequest, LLMResponse, Message},
    types::{OutputFormat, Provider},
};

use super::LLMProviderTrait;

pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var(Provider::OpenAI.env_var())?;
        Ok(Self {
            api_key,
            base_url: Provider::OpenAI.base_url().to_string(),
        })
    }
}

#[derive(Serialize)]
struct OpenAIRequestBody {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
    usage: UsageInfo,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct UsageInfo {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[async_trait]
impl LLMProviderTrait for OpenAIProvider {
    async fn send_request(&self, request: &LLMRequest) -> Result<LLMResponse> {
        let mut messages = request.messages.clone();

        // Add system prompt as first message if provided
        if let Some(system_prompt) = &request.system_prompt {
            messages.insert(0, Message::system(system_prompt));
        }

        // If JSON output is requested, add JSON instruction to system prompt
        if request.output_format == OutputFormat::Json {
            let json_instruction = "You must respond with valid JSON only.";
            if let Some(first_msg) = messages.first_mut() {
                if first_msg.role == "system" {
                    first_msg.content = format!("{} {}", first_msg.content, json_instruction);
                }
            } else {
                messages.insert(0, Message::system(json_instruction));
            }
        }

        let response_format = if request.output_format == OutputFormat::Json {
            Some(ResponseFormat {
                format_type: "json_object".to_string(),
            })
        } else {
            None
        };

        let body = OpenAIRequestBody {
            model: request.model.clone(),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            response_format,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(TurbineError::ApiError(error_text));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        let content = openai_response
            .choices
            .first()
            .ok_or_else(|| TurbineError::InvalidResponse("No choices in response".to_string()))?
            .message
            .content
            .clone();

        Ok(LLMResponse::new(
            content,
            openai_response.usage.prompt_tokens,
            openai_response.usage.completion_tokens,
        ))
    }
}
