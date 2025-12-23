use crate::error::TurbineError;
use serde::{Deserialize, Serialize};

/// LLM provider selection.
///
/// Choose which LLM provider to use. Each provider requires its corresponding
/// API key to be set as an environment variable.
///
/// # Environment Variables
///
/// - `OpenAI`: Requires `OPENAI_API_KEY`
/// - `Anthropic`: Requires `ANTHROPIC_API_KEY`
/// - `Gemini`: Requires `GEMINI_API_KEY`
/// - `Groq`: Requires `GROQ_API_KEY`
///
/// # Example
///
/// ```
/// use turbine_llm::Provider;
///
/// let provider = Provider::OpenAI;
/// assert_eq!(provider.env_var(), "OPENAI_API_KEY");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    /// OpenAI (GPT-4, GPT-3.5, etc.)
    OpenAI,
    /// Anthropic (Claude 3.5 Sonnet, etc.)
    Anthropic,
    /// Google Gemini (Gemini 2.0, 1.5, etc.)
    Gemini,
    /// Groq (Llama, Mixtral, etc.)
    Groq,
}

impl Provider {
    pub fn env_var(&self) -> &'static str {
        match self {
            Provider::OpenAI => "OPENAI_API_KEY",
            Provider::Anthropic => "ANTHROPIC_API_KEY",
            Provider::Gemini => "GEMINI_API_KEY",
            Provider::Groq => "GROQ_API_KEY",
        }
    }

    pub fn base_url(&self) -> &'static str {
        match self {
            Provider::OpenAI => "https://api.openai.com/v1",
            Provider::Anthropic => "https://api.anthropic.com/v1",
            Provider::Gemini => "https://generativelanguage.googleapis.com/v1beta",
            Provider::Groq => "https://api.groq.com/openai/v1",
        }
    }

    /// Parses a provider from a model string in format "provider/model-name".
    ///
    /// Supported provider prefixes:
    /// - "openai/" or "gpt" → OpenAI
    /// - "anthropic/" or "claude" → Anthropic
    /// - "google/" or "gemini" → Gemini
    /// - "groq/" or "llama" or "mixtral" → Groq
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::Provider;
    ///
    /// let (provider, model) = Provider::from_model_string("google/gemini-flash").unwrap();
    /// assert_eq!(provider, Provider::Gemini);
    /// assert_eq!(model, "gemini-flash");
    ///
    /// let (provider, model) = Provider::from_model_string("claude-3-5-sonnet").unwrap();
    /// assert_eq!(provider, Provider::Anthropic);
    /// assert_eq!(model, "claude-3-5-sonnet");
    /// ```
    pub fn from_model_string(model: &str) -> Result<(Self, String), TurbineError> {
        // Check for explicit provider prefix (e.g., "openai/gpt-4")
        if let Some((prefix, model_name)) = model.split_once('/') {
            let provider = match prefix.to_lowercase().as_str() {
                "openai" => Provider::OpenAI,
                "anthropic" => Provider::Anthropic,
                "google" | "gemini" => Provider::Gemini,
                "groq" => Provider::Groq,
                _ => {
                    return Err(TurbineError::InvalidResponse(format!(
                        "Unknown provider prefix: {}. Supported: openai, anthropic, google, gemini, groq",
                        prefix
                    )));
                }
            };
            return Ok((provider, model_name.to_string()));
        }

        // Infer provider from model name patterns
        let model_lower = model.to_lowercase();
        let provider = if model_lower.starts_with("gpt") {
            Provider::OpenAI
        } else if model_lower.starts_with("claude") {
            Provider::Anthropic
        } else if model_lower.starts_with("gemini") {
            Provider::Gemini
        } else if model_lower.starts_with("llama") || model_lower.starts_with("mixtral") {
            Provider::Groq
        } else {
            return Err(TurbineError::InvalidResponse(format!(
                "Cannot infer provider from model name: {}. Use format 'provider/model' (e.g., 'openai/gpt-4')",
                model
            )));
        };

        Ok((provider, model.to_string()))
    }
}

/// Output format for LLM responses.
///
/// Specifies whether the response should be plain text or structured JSON.
///
/// # Example
///
/// ```
/// use turbine_llm::{OutputFormat, LLMRequest};
///
/// let request = LLMRequest::new("gpt-4o-mini")
///     .with_output_format(OutputFormat::Json);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    /// Plain text response (default)
    #[default]
    Text,
    /// Structured JSON response
    Json,
}
