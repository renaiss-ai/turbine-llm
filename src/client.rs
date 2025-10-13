use crate::{
    error::Result,
    models::{LLMRequest, LLMResponse},
    providers::{
        LLMProviderTrait, anthropic::AnthropicProvider, gemini::GeminiProvider, groq::GroqProvider,
        openai::OpenAIProvider,
    },
    types::Provider,
};

/// The main client for interacting with LLM providers.
///
/// `TurbineClient` provides a unified interface to send requests to different LLM providers
/// including OpenAI, Anthropic, Google Gemini, and Groq.
///
/// # Example
///
/// ```no_run
/// use turbine_llm::{TurbineClient, LLMRequest, Message, Provider};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a client for OpenAI
///     let client = TurbineClient::new(Provider::OpenAI)?;
///
///     // Build and send a request
///     let request = LLMRequest::new("gpt-4o-mini")
///         .with_message(Message::user("What is Rust?"));
///
///     let response = client.send_request(&request).await?;
///     println!("{}", response.content);
///
///     Ok(())
/// }
/// ```
pub struct TurbineClient {
    provider: Box<dyn LLMProviderTrait>,
}

impl TurbineClient {
    /// Creates a new client for the specified provider.
    ///
    /// The appropriate API key must be set as an environment variable before calling this.
    ///
    /// # Errors
    ///
    /// Returns an error if the API key environment variable is not set.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use turbine_llm::{TurbineClient, Provider};
    ///
    /// // Requires OPENAI_API_KEY environment variable
    /// let client = TurbineClient::new(Provider::OpenAI)?;
    /// # Ok::<(), turbine_llm::TurbineError>(())
    /// ```
    pub fn new(provider: Provider) -> Result<Self> {
        let provider_impl: Box<dyn LLMProviderTrait> = match provider {
            Provider::OpenAI => Box::new(OpenAIProvider::new()?),
            Provider::Anthropic => Box::new(AnthropicProvider::new()?),
            Provider::Gemini => Box::new(GeminiProvider::new()?),
            Provider::Groq => Box::new(GroqProvider::new()?),
        };

        Ok(Self {
            provider: provider_impl,
        })
    }

    /// Sends a request to the LLM provider and returns the response.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use turbine_llm::{TurbineClient, LLMRequest, Message, Provider};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = TurbineClient::new(Provider::OpenAI)?;
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_message(Message::user("Hello!"));
    ///
    /// let response = client.send_request(&request).await?;
    /// println!("Response: {}", response.content);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_request(&self, request: &LLMRequest) -> Result<LLMResponse> {
        self.provider.send_request(request).await
    }
}
