use crate::{
    error::Result,
    models::{LLMRequest, LLMResponse, Message},
    providers::{
        LLMProviderTrait, anthropic::AnthropicProvider, gemini::GeminiProvider, groq::GroqProvider,
        openai::OpenAIProvider,
    },
    types::Provider,
};
use std::io::{self, Write};

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
    default_model: Option<String>,
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
            default_model: None,
        })
    }

    /// Creates a new client with an explicit API key.
    ///
    /// This is useful when you want to pass the API key directly instead of
    /// reading from environment variables.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use turbine_llm::{TurbineClient, Provider};
    ///
    /// let client = TurbineClient::new_with_key(Provider::OpenAI, "sk-xxx");
    /// ```
    pub fn new_with_key(provider: Provider, api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        let provider_impl: Box<dyn LLMProviderTrait> = match provider {
            Provider::OpenAI => Box::new(OpenAIProvider::new_with_key(&api_key)),
            Provider::Anthropic => Box::new(AnthropicProvider::new_with_key(&api_key)),
            Provider::Gemini => Box::new(GeminiProvider::new_with_key(&api_key)),
            Provider::Groq => Box::new(GroqProvider::new_with_key(&api_key)),
        };

        Self {
            provider: provider_impl,
            default_model: None,
        }
    }

    /// Creates a new client from a model string in format "provider/model-name".
    ///
    /// This is a simplified constructor that automatically:
    /// - Parses the provider from the model string
    /// - Extracts the model name
    /// - Checks for API key in environment
    /// - Prompts for API key if not found
    ///
    /// # Example
    ///
    /// ```no_run
    /// use turbine_llm::TurbineClient;
    ///
    /// // Automatically detects provider from model string
    /// let client = TurbineClient::from_model("google/gemini-flash")?;
    /// let client = TurbineClient::from_model("openai/gpt-4o-mini")?;
    /// let client = TurbineClient::from_model("anthropic/claude-3-5-sonnet")?;
    ///
    /// // Can also infer from model name
    /// let client = TurbineClient::from_model("gpt-4o")?;
    /// let client = TurbineClient::from_model("claude-3-5-sonnet")?;
    /// # Ok::<(), turbine_llm::TurbineError>(())
    /// ```
    pub fn from_model(model_str: &str) -> Result<Self> {
        let (provider, model_name) = Provider::from_model_string(model_str)?;

        // Check if API key exists, prompt if not
        let env_var = provider.env_var();
        if std::env::var(env_var).is_err() {
            println!("API key not found in environment.");
            println!("Provider: {:?}", provider);
            println!("Required environment variable: {}", env_var);
            print!("Please enter your API key: ");
            io::stdout().flush()?;

            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key)?;
            let api_key = api_key.trim();

            if api_key.is_empty() {
                return Err(crate::TurbineError::ApiKeyNotFound(env_var.to_string()));
            }

            // Set the environment variable for this session
            // SAFETY: We're setting an environment variable in a single-threaded context
            // before any provider is initialized. This is safe as long as no other threads
            // are reading environment variables concurrently.
            unsafe {
                std::env::set_var(env_var, api_key);
            }
        }

        let provider_impl: Box<dyn LLMProviderTrait> = match provider {
            Provider::OpenAI => Box::new(OpenAIProvider::new()?),
            Provider::Anthropic => Box::new(AnthropicProvider::new()?),
            Provider::Gemini => Box::new(GeminiProvider::new()?),
            Provider::Groq => Box::new(GroqProvider::new()?),
        };

        Ok(Self {
            provider: provider_impl,
            default_model: Some(model_name),
        })
    }

    /// Creates a client from a model string with an explicit API key.
    ///
    /// Combines model string parsing with direct API key passing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use turbine_llm::TurbineClient;
    ///
    /// let client = TurbineClient::from_model_with_key("openai/gpt-4o", "sk-xxx")?;
    /// let response = client.send("Hello").await?;
    /// # Ok::<(), turbine_llm::TurbineError>(())
    /// ```
    pub fn from_model_with_key(model_str: &str, api_key: impl Into<String>) -> Result<Self> {
        let (provider, model_name) = Provider::from_model_string(model_str)?;
        let api_key = api_key.into();

        let provider_impl: Box<dyn LLMProviderTrait> = match provider {
            Provider::OpenAI => Box::new(OpenAIProvider::new_with_key(&api_key)),
            Provider::Anthropic => Box::new(AnthropicProvider::new_with_key(&api_key)),
            Provider::Gemini => Box::new(GeminiProvider::new_with_key(&api_key)),
            Provider::Groq => Box::new(GroqProvider::new_with_key(&api_key)),
        };

        Ok(Self {
            provider: provider_impl,
            default_model: Some(model_name),
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

    /// Simplified method to send a single user message.
    ///
    /// This is a convenience method for quick interactions. It uses the default model
    /// set during `from_model()` construction.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No default model was set (only works with `from_model()`)
    /// - The HTTP request fails
    /// - The API returns an error response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use turbine_llm::TurbineClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = TurbineClient::from_model("openai/gpt-4o-mini")?;
    ///
    /// // Simple one-liner
    /// let response = client.send("What is Rust?").await?;
    /// println!("{}", response.content);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, message: &str) -> Result<LLMResponse> {
        let model = self.default_model.as_ref().ok_or_else(|| {
            crate::TurbineError::MissingField(
                "No default model set. Use from_model() constructor or send_request() directly"
                    .to_string(),
            )
        })?;

        let request = LLMRequest::new(model).with_message(Message::user(message));

        self.send_request(&request).await
    }

    /// Simplified method to send a message with a system prompt.
    ///
    /// Convenience method that combines system prompt and user message.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use turbine_llm::TurbineClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = TurbineClient::from_model("anthropic/claude-3-5-sonnet")?;
    ///
    /// let response = client.send_with_system(
    ///     "You are a Rust expert",
    ///     "Explain ownership in one sentence"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_with_system(
        &self,
        system_prompt: &str,
        message: &str,
    ) -> Result<LLMResponse> {
        let model = self.default_model.as_ref().ok_or_else(|| {
            crate::TurbineError::MissingField(
                "No default model set. Use from_model() constructor or send_request() directly"
                    .to_string(),
            )
        })?;

        let request = LLMRequest::new(model)
            .with_system_prompt(system_prompt)
            .with_message(Message::user(message));

        self.send_request(&request).await
    }
}
