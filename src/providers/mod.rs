pub mod anthropic;
pub mod gemini;
pub mod groq;
pub mod openai;

use crate::{
    error::Result,
    models::{LLMRequest, LLMResponse},
};
use async_trait::async_trait;

#[async_trait]
pub trait LLMProviderTrait: Send + Sync {
    async fn send_request(&self, request: &LLMRequest) -> Result<LLMResponse>;
}
