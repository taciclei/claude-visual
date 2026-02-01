# Contributing to Claude Visual

Thank you for your interest in contributing to Claude Visual!

## Development Setup

```bash
# Clone the repository
git clone https://github.com/taciclei/claude-visual.git
cd claude-visual

# Build debug
cargo build

# Run with logs
RUST_LOG=debug cargo run

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Lint
cargo clippy
```

## Project Structure

```
src/
├── main.rs          # Entry point
├── app/             # App state, settings, themes
├── ui/              # GPUI components
├── claude/          # Claude CLI integration
├── project/         # Project management
├── git/             # Git operations
├── markdown/        # Markdown rendering
├── storage/         # SQLite database
└── plugins/         # Extension system
```

## Code Style

### Rust Conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for lints
- Document public modules and functions
- Write unit tests for new functionality

### GPUI Conventions

```rust
// Stateless component (RenderOnce)
pub struct Button { ... }
impl RenderOnce for Button { ... }

// Stateful component (Render + Entity)
pub struct ChatView { ... }
impl Render for ChatView { ... }
```

### Naming Conventions
- `snake_case` for functions and variables
- `PascalCase` for types and traits
- `SCREAMING_SNAKE_CASE` for constants

## Adding Features

### 1. New UI Component

```rust
// src/ui/components/my_component.rs
use gpui::*;

pub struct MyComponent {
    // props
}

impl MyComponent {
    pub fn new() -> Self { ... }
}

impl RenderOnce for MyComponent {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            // ...
    }
}
```

### 2. New Action

```rust
// src/main.rs
actions!(claude_visual, [MyAction]);

fn register_actions(cx: &mut App) {
    cx.on_action(|_: &MyAction, cx| {
        // Handle action
    });
}
```

### 3. New Service

```rust
// src/myservice/mod.rs
pub mod client;
pub use client::MyClient;

// src/myservice/client.rs
pub struct MyClient { ... }
impl MyClient {
    pub fn new() -> Self { ... }
}
```

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // ...
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
#[test]
fn test_full_flow() {
    // ...
}
```

## Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes with conventional commits
4. Run checks: `cargo fmt && cargo clippy && cargo test`
5. Push to your fork: `git push origin feature/my-feature`
6. Open a Pull Request

### Commit Message Format

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: add new feature`
- `fix: resolve bug`
- `docs: update documentation`
- `refactor: restructure code`
- `test: add tests`
- `chore: update dependencies`

### PR Checklist

- [ ] `cargo fmt` passes
- [ ] `cargo clippy` has no warnings
- [ ] `cargo test` passes
- [ ] Documentation updated if needed
- [ ] CHANGELOG updated for significant changes

## Reporting Issues

### Bug Reports

Please include:
- Claude Visual version
- OS and version
- Steps to reproduce
- Expected vs actual behavior
- Logs (`RUST_LOG=debug cargo run`)

### Feature Requests

Please include:
- Use case description
- Proposed solution
- Alternative approaches considered

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

See [LICENSE](LICENSE) for details.
