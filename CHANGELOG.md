# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2025-01-23

### Fixed
- Fixed doctest compilation error in `from_model_with_key` example

## [0.2.1] - 2025-01-23

### Added
- `TurbineClient::new_with_key(provider, api_key)` - create client with explicit API key
- `TurbineClient::from_model_with_key(model_str, api_key)` - parse model string with explicit API key
- Provider `new_with_key()` methods for all providers (OpenAI, Anthropic, Gemini, Groq)

## [0.2.0] - 2025-01-17

### Added
- **Simplified API**: New `TurbineClient::from_model()` constructor for easier client creation
  - Automatic provider detection from model strings (e.g., `"openai/gpt-4o-mini"`, `"google/gemini-flash"`)
  - Model name inference support (e.g., `"gpt-4o"`, `"claude-3-5-sonnet"`, `"gemini-flash"`)
  - Interactive API key prompting when not found in environment
- **Convenience Methods**:
  - `client.send()` - Simple one-liner message sending
  - `client.send_with_system()` - Send messages with system prompt
- **Provider Parsing**: New `Provider::from_model_string()` for parsing provider from model strings
- New example: `simple_usage.rs` demonstrating the simplified API

### Changed
- Updated documentation to showcase simplified API as recommended approach
- Traditional API remains fully supported for advanced use cases
- Improved error handling for API key prompts (added IO error support)

### Documentation
- README updated with simplified API examples prominently featured
- API reference restructured to highlight new convenience methods
- Added comprehensive examples for both simplified and traditional APIs

## [0.1.0] - 2025-01-13

### Added
- Initial release of Turbine LLM
- Support for OpenAI (GPT-4, GPT-3.5, etc.)
- Support for Anthropic (Claude 3.5 Sonnet, Haiku, etc.)
- Support for Google Gemini (Gemini 2.0, 1.5, etc.)
- Support for Groq (Llama, Mixtral, etc.)
- Unified `TurbineClient` interface for all providers
- Builder pattern for `LLMRequest` with optional parameters
- Support for text and JSON output formats
- Async/await support with Tokio
- Comprehensive error handling with `TurbineError`
- Full rustdoc documentation
- Example code for basic usage, JSON output, and conversations
- MIT OR Apache-2.0 dual licensing

[Unreleased]: https://github.com/Renaiss-AI/turbine-llm/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/Renaiss-AI/turbine-llm/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/Renaiss-AI/turbine-llm/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Renaiss-AI/turbine-llm/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Renaiss-AI/turbine-llm/releases/tag/v0.1.0
