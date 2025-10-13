# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/Renaiss-AI/turbine-llm/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Renaiss-AI/turbine-llm/releases/tag/v0.1.0
