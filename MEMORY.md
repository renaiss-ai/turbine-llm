# Turbine LLM - Project Memory

## Project Overview

**Turbine LLM** is a unified Rust library for calling multiple LLM providers (OpenAI, Anthropic, Gemini, Groq) through a single interface.

## Architecture

### Core Components

1. **TurbineClient** (`src/client.rs`)
   - Main entry point for users
   - Initializes provider based on enum selection
   - Routes requests to appropriate provider implementation

2. **Provider Trait** (`src/providers/mod.rs`)
   - `LLMProviderTrait` - Common interface for all providers
   - Async trait with `send_request()` method

3. **Provider Implementations** (`src/providers/`)
   - **OpenAI** (`openai.rs`) - Uses `/v1/chat/completions` endpoint
   - **Groq** (`groq.rs`) - OpenAI-compatible API
   - **Anthropic** (`anthropic.rs`) - Uses `/v1/messages` endpoint
   - **Gemini** (`gemini.rs`) - Uses `/v1beta/models/{model}:generateContent` endpoint

4. **Models** (`src/models.rs`)
   - `Message` - Chat message (role + content)
   - `LLMRequest` - Unified request structure with builder pattern
   - `LLMResponse` - Unified response with content and usage stats
   - `Usage` - Token usage information

5. **Types** (`src/types.rs`)
   - `Provider` enum - OpenAI, Anthropic, Gemini, Groq
   - `OutputFormat` enum - Text (default), Json

6. **Error Handling** (`src/error.rs`)
   - `TurbineError` - Custom error type using thiserror
   - `Result<T>` type alias

## Key Features

### Request Parameters
- `model` (required) - Model identifier
- `messages` (required) - List of chat messages
- `system_prompt` (optional) - System instructions
- `max_tokens` (optional, default: 1024) - Maximum tokens to generate
- `temperature` (optional) - Randomness (0.0-2.0)
- `top_p` (optional) - Nucleus sampling
- `output_format` - Text or Json

### JSON Output Handling

Each provider handles JSON output differently:

1. **OpenAI/Groq**:
   - Uses `response_format: {"type": "json_object"}`
   - Requires "JSON" in system prompt

2. **Anthropic**:
   - No native parameter
   - Adds JSON instruction to system prompt

3. **Gemini**:
   - Uses `generationConfig.responseMimeType: "application/json"`

## API Keys

Required environment variables:
- `OPENAI_API_KEY`
- `ANTHROPIC_API_KEY`
- `GEMINI_API_KEY`
- `GROQ_API_KEY`

## Dependencies

- `reqwest` - HTTP client
- `serde` / `serde_json` - JSON serialization
- `tokio` - Async runtime
- `thiserror` - Error handling
- `async-trait` - Async trait support

## Usage Pattern

```rust
let client = TurbineClient::new(Provider::OpenAI)?;
let request = LLMRequest::new("model")
    .with_system_prompt("prompt")
    .with_message(Message::user("query"));
let response = client.send_request(&request).await?;
```

## Provider-Specific Notes

### OpenAI
- Base URL: `https://api.openai.com/v1`
- Auth: Bearer token in Authorization header
- Messages support: system, user, assistant

### Anthropic
- Base URL: `https://api.anthropic.com/v1`
- Auth: API key in `x-api-key` header
- Requires `anthropic-version: 2023-06-01` header
- System prompt is separate parameter (not in messages)
- `max_tokens` is required

### Gemini
- Base URL: `https://generativelanguage.googleapis.com/v1beta`
- Auth: API key in `x-goog-api-key` header
- Different structure: `contents` with `parts`
- Role mapping: assistant -> model

### Groq
- Base URL: `https://api.groq.com/openai/v1`
- OpenAI-compatible API
- Same structure as OpenAI

## File Structure

```
turbine-llm/
├── src/
│   ├── lib.rs           # Public API exports
│   ├── client.rs        # TurbineClient
│   ├── models.rs        # Request/Response structs
│   ├── types.rs         # Enums
│   ├── error.rs         # Error types
│   └── providers/       # Provider implementations
│       ├── mod.rs       # Trait definition
│       ├── openai.rs
│       ├── groq.rs
│       ├── anthropic.rs
│       └── gemini.rs
├── examples/
│   └── basic_usage.rs   # Usage examples
├── Cargo.toml
└── README.md
```

## Design Principles

1. **Simple & Clean** - Minimal, straightforward implementations
2. **Unified Interface** - Same API across all providers
3. **Type Safety** - Full Rust type safety
4. **Async First** - Built with async/await
5. **Error Handling** - Proper error types and propagation
