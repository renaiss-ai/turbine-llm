use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Result, TurbineError},
    models::{LLMRequest, LLMResponse},
    types::{OutputFormat, Provider},
};

use super::LLMProviderTrait;

pub struct GeminiProvider {
    api_key: String,
    base_url: String,
}

impl GeminiProvider {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var(Provider::Gemini.env_var())?;
        Ok(Self {
            api_key,
            base_url: Provider::Gemini.base_url().to_string(),
        })
    }
}

#[derive(Serialize)]
struct GeminiRequestBody {
    contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "systemInstruction")]
    system_instruction: Option<SystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "generationConfig")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Serialize)]
struct SystemInstruction {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "topP")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responseMimeType")]
    response_mime_type: Option<String>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: UsageMetadata,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

#[derive(Deserialize)]
struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: u32,
}

#[async_trait]
impl LLMProviderTrait for GeminiProvider {
    async fn send_request(&self, request: &LLMRequest) -> Result<LLMResponse> {
        // Convert messages to Gemini format
        let mut contents: Vec<Content> = Vec::new();

        for message in &request.messages {
            // Map roles: assistant -> model, user -> user, system -> ignore (handled separately)
            if message.role == "system" {
                continue;
            }

            let role = if message.role == "assistant" {
                "model"
            } else {
                "user"
            };

            contents.push(Content {
                role: role.to_string(),
                parts: vec![Part {
                    text: message.content.clone(),
                }],
            });
        }

        if contents.is_empty() {
            return Err(TurbineError::MissingField(
                "At least one user or assistant message is required".to_string(),
            ));
        }

        // System instruction
        let system_instruction = request
            .system_prompt
            .as_ref()
            .map(|prompt| SystemInstruction {
                parts: vec![Part {
                    text: prompt.clone(),
                }],
            });

        // Generation config
        let response_mime_type = if request.output_format == OutputFormat::Json {
            Some("application/json".to_string())
        } else {
            None
        };

        let generation_config = Some(GenerationConfig {
            temperature: request.temperature,
            top_p: request.top_p,
            max_output_tokens: request.max_tokens,
            response_mime_type,
        });

        let body = GeminiRequestBody {
            contents,
            system_instruction,
            generation_config,
        };

        let client = reqwest::Client::new();
        let url = format!("{}/models/{}:generateContent", self.base_url, request.model);

        let response = client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(TurbineError::ApiError(error_text));
        }

        let gemini_response: GeminiResponse = response.json().await?;

        let content = gemini_response
            .candidates
            .first()
            .ok_or_else(|| TurbineError::InvalidResponse("No candidates in response".to_string()))?
            .content
            .parts
            .first()
            .ok_or_else(|| TurbineError::InvalidResponse("No parts in response".to_string()))?
            .text
            .clone();

        Ok(LLMResponse::new(
            content,
            gemini_response.usage_metadata.prompt_token_count,
            gemini_response.usage_metadata.candidates_token_count,
        ))
    }
}
