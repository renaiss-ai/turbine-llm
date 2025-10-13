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
