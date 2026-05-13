# Contributing Guidelines

Thank you for considering contributing to **healthwand**!

## How to Contribute

1. **Fork the repository** and clone your fork.
2. **Create a feature branch**:
   ```sh
   git checkout -b <feature-or-bugfix-name>
   ```
3. **Make your changes** following the project's coding style (see `rustfmt.toml` if present).
4. **Run tests** to ensure everything passes:
   ```sh
   cargo test --all
   ```
5. **Commit your changes** with a clear commit message.
6. **Push** to your fork and open a **Pull Request** against the `main` branch.

## Code Style
- Rust code should be formatted with `cargo fmt`.
- Linting is enforced via `cargo clippy`; ensure no warnings remain.
- Keep documentation up‑to‑date; add doc comments for public items.

## Issue Reporting
- Use the GitHub Issues page.
- Provide a clear title, description, and steps to reproduce.
- Include relevant logs or error messages.

## License
- By contributing, you agree that your contributions will be licensed under the project's MIT license.

---
*Generated automatically as part of the repository audit.*
