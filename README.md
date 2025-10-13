# Turbine LLM

[![Crates.io](https://img.shields.io/crates/v/turbine-llm)](https://crates.io/crates/turbine-llm)
[![Documentation](https://docs.rs/turbine-llm/badge.svg)](https://docs.rs/turbine-llm)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/Renaiss-AI/turbine-llm#license)
[![CI](https://github.com/Renaiss-AI/turbine-llm/workflows/CI/badge.svg)](https://github.com/Renaiss-AI/turbine-llm/actions)

**One interface, all LLMs** - A unified Rust library for calling multiple LLM providers with growing model support.

🚀 Switch between OpenAI, Anthropic, Gemini, and Groq with minimal code changes. Perfect for building AI applications that need provider flexibility.

---

## Sponsored by Renaiss AI

<p align="center">
  <a href="https://renaiss.ai">
    <strong>Renaiss AI</strong> - Enterprise AI Research Lab
  </a>
</p>

Turbine LLM is developed and maintained with support from [Renaiss AI](https://renaiss.ai), bridging the gap between AI potential and business reality.

---

## Features

- **Unified API**: Single interface for multiple LLM providers
- **Simple & Clean**: Minimal, straightforward code - no complexity
- **Text & JSON Output**: Support for both text and structured JSON responses
- **Async/Await**: Built with Tokio for high-performance async operations
- **Type-Safe**: Full Rust type safety with proper error handling
- **Growing Support**: New providers and models added regularly

## Why Turbine?

- **Provider Independence**: Easily switch providers or use multiple simultaneously
- **Consistent Interface**: Same code works across all providers
- **Production Ready**: Proper error handling, async support, comprehensive docs
- **Actively Maintained**: Regular updates with new models and providers

## Supported Providers

Currently integrated:

- ✅ **OpenAI** (GPT-4, GPT-3.5, etc.)
- ✅ **Anthropic** (Claude 3.5 Sonnet, Haiku, etc.)
- ✅ **Google Gemini** (Gemini 2.0, 1.5, etc.)
- ✅ **Groq** (Llama, Mixtral, etc.)

Coming soon:

- 🔜 Cohere
- 🔜 Mistral AI
- 🔜 Perplexity

*New providers and models added regularly. Check [CHANGELOG.md](CHANGELOG.md) for updates.*

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
turbine-llm = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### 1. Set API Keys

```bash
export OPENAI_API_KEY="your-openai-key"
export ANTHROPIC_API_KEY="your-anthropic-key"
export GEMINI_API_KEY="your-gemini-key"
export GROQ_API_KEY="your-groq-key"
```

### 2. Basic Usage

```rust
use turbine_llm::{TurbineClient, LLMRequest, Message, Provider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client for your chosen provider
    let client = TurbineClient::new(Provider::OpenAI)?;

    // Build a request
    let request = LLMRequest::new("gpt-4o-mini")
        .with_system_prompt("You are a helpful assistant.")
        .with_message(Message::user("What is Rust?"))
        .with_max_tokens(100);

    // Send and get response
    let response = client.send_request(&request).await?;
    println!("{}", response.content);

    Ok(())
}
```

### 3. JSON Output

Request structured JSON from any provider:

```rust
use turbine_llm::{TurbineClient, LLMRequest, Message, OutputFormat, Provider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TurbineClient::new(Provider::Anthropic)?;

    let request = LLMRequest::new("claude-3-5-sonnet-20241022")
        .with_system_prompt("Return data as JSON.")
        .with_message(Message::user("Info about Paris with keys: name, country, population"))
        .with_output_format(OutputFormat::Json);

    let response = client.send_request(&request).await?;
    println!("{}", response.content);

    Ok(())
}
```

### 4. Multi-turn Conversations

```rust
let request = LLMRequest::new("gemini-2.0-flash-exp")
    .with_messages(vec![
        Message::user("Hello! My name is Alice."),
        Message::assistant("Hello Alice! Nice to meet you."),
        Message::user("What's my name?"),
    ]);
```

## API Reference

### Provider

Select which LLM provider to use:

```rust
pub enum Provider {
    OpenAI,      // Requires OPENAI_API_KEY
    Anthropic,   // Requires ANTHROPIC_API_KEY
    Gemini,      // Requires GEMINI_API_KEY
    Groq,        // Requires GROQ_API_KEY
}
```

### LLMRequest Builder

Construct requests with optional parameters:

```rust
LLMRequest::new("model-name")
    .with_system_prompt("System prompt")        // Optional
    .with_message(Message::user("Query"))       // Add single message
    .with_messages(vec![...])                   // Add multiple messages
    .with_max_tokens(1000)                      // Optional, default: 1024
    .with_temperature(0.7)                      // Optional, 0.0-2.0
    .with_top_p(0.9)                            // Optional
    .with_output_format(OutputFormat::Json)     // Text (default) or Json
```

### Message Helpers

```rust
Message::user("User message")
Message::assistant("Assistant message")
Message::system("System message")
```

## Model Examples

### OpenAI
- `gpt-4o` - Latest GPT-4 Omni
- `gpt-4o-mini` - Faster, cost-effective
- `gpt-3.5-turbo` - Fast and efficient

### Anthropic
- `claude-3-5-sonnet-20241022` - Most capable
- `claude-3-5-haiku-20241022` - Fast and affordable

### Gemini
- `gemini-2.0-flash-exp` - Latest experimental
- `gemini-1.5-pro` - Production ready

### Groq
- `llama-3.3-70b-versatile` - Powerful Llama model
- `mixtral-8x7b-32768` - Mixtral with large context

## Error Handling

```rust
match client.send_request(&request).await {
    Ok(response) => println!("{}", response.content),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Examples

Run the included examples:

```bash
# Basic text generation
cargo run --example text_generation

# JSON output
cargo run --example json_output

# Multi-turn conversation
cargo run --example conversation
```

## Documentation

- [API Documentation](https://docs.rs/turbine-llm)
- [Examples](./examples/)
- [Changelog](./CHANGELOG.md)
- [Contributing](./CONTRIBUTING.md)

## Troubleshooting

### API Key Not Found

```
Error: API key not found for provider: OpenAI
```

**Solution**: Make sure the environment variable is set:

```bash
export OPENAI_API_KEY="your-key-here"
```

### Model Not Found

Different providers use different model names. Check the [Model Examples](#model-examples) section for correct model identifiers.

### Rate Limiting

If you hit rate limits, implement exponential backoff or switch providers temporarily.

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

Developed with ❤️ by the Rust community and sponsored by [Renaiss AI](https://renaiss.ai).
