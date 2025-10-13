# Contributing to Turbine LLM

Thank you for your interest in contributing to Turbine LLM! We welcome contributions from the community.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue on GitHub with:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Rust version, etc.)

### Suggesting Features

We love new ideas! Open an issue to discuss:
- What problem does it solve?
- How should it work?
- Any implementation ideas?

### Adding New Providers

Want to add support for a new LLM provider? Great! Here's what you need:

1. Create a new file in `src/providers/your_provider.rs`
2. Implement the `LLMProviderTrait`
3. Add the provider to the `Provider` enum in `src/types.rs`
4. Update the match statement in `src/client.rs`
5. Add documentation and examples
6. Update the README and CHANGELOG

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Add documentation for public APIs
- Include examples in rustdoc comments
- Write clear commit messages

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Ensure all tests pass (`cargo test`)
6. Run `cargo fmt` and `cargo clippy`
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to your branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

### Documentation

- All public APIs must be documented
- Include examples in doc comments
- Update README.md if adding features
- Update CHANGELOG.md

## Code of Conduct

Please note that this project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Questions?

Feel free to open an issue or reach out to the maintainers.

## License

By contributing, you agree that your contributions will be licensed under the same MIT OR Apache-2.0 dual license as the project.
