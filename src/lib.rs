//! # Turbine LLM
//!
//! A unified Rust interface for multiple LLM providers with growing model support.
//!
//! Turbine provides a simple, consistent API to interact with various Large Language Model
//! providers including OpenAI, Anthropic, Google Gemini, and Groq. Switch between providers
//! with minimal code changes.
//!
//! ## Features
//!
//! - **Unified API**: Single interface for multiple LLM providers
//! - **Simple & Clean**: Minimal, straightforward code
//! - **Text & JSON Output**: Support for both text and structured JSON responses
//! - **Async/Await**: Built with Tokio for async operations
//! - **Type-Safe**: Full Rust type safety with proper error handling
//!
//! ## Quick Start
//!
//! ```no_run
//! use turbine_llm::{TurbineClient, LLMRequest, Message, Provider};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client for your chosen provider
//!     let client = TurbineClient::new(Provider::OpenAI)?;
//!
//!     // Build a request
//!     let request = LLMRequest::new("gpt-4o-mini")
//!         .with_system_prompt("You are a helpful assistant.")
//!         .with_message(Message::user("What is Rust?"))
//!         .with_max_tokens(100);
//!
//!     // Send the request
//!     let response = client.send_request(&request).await?;
//!     println!("{}", response.content);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Supported Providers
//!
//! Set the appropriate API key as an environment variable:
//!
//! - **OpenAI**: `OPENAI_API_KEY`
//! - **Anthropic**: `ANTHROPIC_API_KEY`
//! - **Gemini**: `GEMINI_API_KEY`
//! - **Groq**: `GROQ_API_KEY`
//!
//! ## JSON Output
//!
//! Request structured JSON responses from any provider:
//!
//! ```no_run
//! use turbine_llm::{TurbineClient, LLMRequest, Message, OutputFormat, Provider};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = TurbineClient::new(Provider::Anthropic)?;
//!
//! let request = LLMRequest::new("claude-3-5-sonnet-20241022")
//!     .with_system_prompt("Return data as JSON.")
//!     .with_message(Message::user("Give me info about Paris"))
//!     .with_output_format(OutputFormat::Json);
//!
//! let response = client.send_request(&request).await?;
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod models;
pub mod providers;
pub mod types;

// Re-export commonly used types for convenience
pub use client::TurbineClient;
pub use error::{Result, TurbineError};
pub use models::{LLMRequest, LLMResponse, Message, Usage};
pub use types::{OutputFormat, Provider};
