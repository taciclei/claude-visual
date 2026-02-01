# Contributing

## Development Setup

```bash
# Clone
git clone https://github.com/your-repo/claude-visual.git
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

### Rust
- `cargo fmt` pour le formatage
- `cargo clippy` pour les lints
- Documenter les modules et fonctions publiques

### GPUI Conventions
```rust
// Composant stateless (RenderOnce)
pub struct Button { ... }
impl RenderOnce for Button { ... }

// Composant stateful (Render + Entity)
pub struct ChatView { ... }
impl Render for ChatView { ... }
```

### Naming
- `snake_case` pour fonctions et variables
- `PascalCase` pour types et traits
- `SCREAMING_SNAKE_CASE` pour constantes

## Adding Features

### 1. Nouveau composant UI

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

### 2. Nouvelle action

```rust
// src/main.rs
actions!(claude_visual, [MyAction]);

fn register_actions(cx: &mut App) {
    cx.on_action(|_: &MyAction, cx| {
        // Handle
    });
}
```

### 3. Nouveau service

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

1. Fork le repo
2. Créer une branche: `git checkout -b feature/my-feature`
3. Committer: `git commit -m "Add my feature"`
4. Pusher: `git push origin feature/my-feature`
5. Ouvrir une PR

### PR Checklist
- [ ] `cargo fmt` passé
- [ ] `cargo clippy` sans warnings
- [ ] `cargo test` passé
- [ ] Documentation mise à jour si nécessaire
- [ ] Changelog mis à jour

## Reporting Issues

### Bug Report
- Version de Claude Visual
- OS et version
- Steps to reproduce
- Expected vs actual behavior
- Logs (`RUST_LOG=debug`)

### Feature Request
- Use case
- Proposed solution
- Alternatives considérées

## License

MIT - Voir [LICENSE](../LICENSE)
