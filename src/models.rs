use crate::types::OutputFormat;
use serde::{Deserialize, Serialize};

/// A chat message with a role and content.
///
/// Messages represent individual turns in a conversation. Each message has a role
/// (user, assistant, or system) and textual content.
///
/// # Example
///
/// ```
/// use turbine_llm::Message;
///
/// let user_msg = Message::user("Hello!");
/// let assistant_msg = Message::assistant("Hi there!");
/// let system_msg = Message::system("You are a helpful assistant.");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender (e.g., "user", "assistant", "system")
    pub role: String,
    /// The text content of the message
    pub content: String,
}

impl Message {
    /// Creates a new message with the specified role and content.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::Message;
    ///
    /// let msg = Message::new("user", "Hello!");
    /// assert_eq!(msg.role, "user");
    /// assert_eq!(msg.content, "Hello!");
    /// ```
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }

    /// Creates a user message.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::Message;
    ///
    /// let msg = Message::user("What is Rust?");
    /// assert_eq!(msg.role, "user");
    /// ```
    pub fn user(content: impl Into<String>) -> Self {
        Self::new("user", content)
    }

    /// Creates an assistant message.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::Message;
    ///
    /// let msg = Message::assistant("Rust is a systems programming language.");
    /// assert_eq!(msg.role, "assistant");
    /// ```
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new("assistant", content)
    }

    /// Creates a system message.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::Message;
    ///
    /// let msg = Message::system("You are a helpful assistant.");
    /// assert_eq!(msg.role, "system");
    /// ```
    pub fn system(content: impl Into<String>) -> Self {
        Self::new("system", content)
    }
}

/// A request to send to an LLM provider.
///
/// Use the builder pattern to construct requests with various parameters.
/// Only `model` is required; all other parameters are optional.
///
/// # Example
///
/// ```
/// use turbine_llm::{LLMRequest, Message, OutputFormat};
///
/// let request = LLMRequest::new("gpt-4o-mini")
///     .with_system_prompt("You are a helpful assistant.")
///     .with_message(Message::user("Hello!"))
///     .with_max_tokens(100)
///     .with_temperature(0.7)
///     .with_output_format(OutputFormat::Text);
/// ```
#[derive(Debug, Clone)]
pub struct LLMRequest {
    /// The model identifier (e.g., "gpt-4o-mini", "claude-3-5-sonnet-20241022")
    pub model: String,
    /// The conversation messages
    pub messages: Vec<Message>,
    /// Optional system prompt to guide the model's behavior
    pub system_prompt: Option<String>,
    /// Maximum number of tokens to generate (default: 1024)
    pub max_tokens: Option<u32>,
    /// Sampling temperature from 0.0 to 2.0 (higher = more random)
    pub temperature: Option<f32>,
    /// Nucleus sampling threshold (0.0 to 1.0)
    pub top_p: Option<f32>,
    /// Output format: text or JSON
    pub output_format: OutputFormat,
}

impl LLMRequest {
    /// Creates a new request for the specified model.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::LLMRequest;
    ///
    /// let request = LLMRequest::new("gpt-4o-mini");
    /// ```
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            messages: Vec::new(),
            system_prompt: None,
            max_tokens: Some(1024),
            temperature: None,
            top_p: None,
            output_format: OutputFormat::Text,
        }
    }

    /// Adds a single message to the request.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::{LLMRequest, Message};
    ///
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_message(Message::user("Hello!"));
    /// ```
    pub fn with_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    /// Sets all messages for the request, replacing any existing ones.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::{LLMRequest, Message};
    ///
    /// let messages = vec![
    ///     Message::user("Hello!"),
    ///     Message::assistant("Hi!"),
    ///     Message::user("How are you?"),
    /// ];
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_messages(messages);
    /// ```
    pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    /// Sets the system prompt to guide the model's behavior.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::LLMRequest;
    ///
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_system_prompt("You are a helpful assistant.");
    /// ```
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Sets the maximum number of tokens to generate.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::LLMRequest;
    ///
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_max_tokens(500);
    /// ```
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the sampling temperature (0.0 to 2.0).
    ///
    /// Higher values make output more random, lower values more deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::LLMRequest;
    ///
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_temperature(0.7);
    /// ```
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the nucleus sampling threshold (0.0 to 1.0).
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::LLMRequest;
    ///
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_top_p(0.9);
    /// ```
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Sets the output format (Text or Json).
    ///
    /// # Example
    ///
    /// ```
    /// use turbine_llm::{LLMRequest, OutputFormat};
    ///
    /// let request = LLMRequest::new("gpt-4o-mini")
    ///     .with_output_format(OutputFormat::Json);
    /// ```
    pub fn with_output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    }
}

/// Token usage information for a request/response.
///
/// Tracks the number of tokens consumed by the input prompt and generated output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Number of tokens in the input prompt
    pub input_tokens: u32,
    /// Number of tokens in the generated output
    pub output_tokens: u32,
}

/// Response from an LLM provider.
///
/// Contains the generated content and token usage information.
///
/// # Example
///
/// ```no_run
/// # use turbine_llm::{TurbineClient, LLMRequest, Message, Provider};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = TurbineClient::new(Provider::OpenAI)?;
/// # let request = LLMRequest::new("gpt-4o-mini")
/// #     .with_message(Message::user("Hello!"));
/// let response = client.send_request(&request).await?;
/// println!("Content: {}", response.content);
/// println!("Tokens used: {} input, {} output",
///     response.usage.input_tokens,
///     response.usage.output_tokens);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    /// The generated text content
    pub content: String,
    /// Token usage statistics
    pub usage: Usage,
}

impl LLMResponse {
    /// Creates a new response with content and token usage.
    pub fn new(content: String, input_tokens: u32, output_tokens: u32) -> Self {
        Self {
            content,
            usage: Usage {
                input_tokens,
                output_tokens,
            },
        }
    }
}
