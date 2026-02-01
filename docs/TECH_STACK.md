# Stack Technique

## Core Dependencies

| Composant | Crate | Version | Usage |
|-----------|-------|---------|-------|
| UI Framework | gpui | 0.2 | GPU-accelerated UI (Zed) |
| Async Runtime | tokio | 1.x | Process spawning, async I/O |
| Markdown | pulldown-cmark | 0.12 | CommonMark parsing |
| Syntax | tree-sitter | 0.24 | Code highlighting |
| Git | git2 | 0.19 | Repository operations |
| Database | rusqlite | 0.32 | SQLite storage |
| Serialization | serde + serde_json | 1.x | JSON/config parsing |
| Config | toml | 0.8 | Settings files |
| Errors | anyhow + thiserror | 1.x / 2.x | Error handling |
| Logging | tracing | 0.1 | Structured logging |

## Optional Dependencies

| Composant | Crate | Version | Feature Flag |
|-----------|-------|---------|--------------|
| WASM Runtime | wasmtime | 27 | `plugins` |
| WIT Bindings | wit-bindgen | 0.36 | `plugins` |

## Tree-sitter Grammars

| Language | Crate | Version |
|----------|-------|---------|
| Rust | tree-sitter-rust | 0.23 |
| JavaScript | tree-sitter-javascript | 0.23 |
| TypeScript | tree-sitter-typescript | 0.23 |
| Python | tree-sitter-python | 0.23 |
| JSON | tree-sitter-json | 0.24 |
| TOML | tree-sitter-toml-ng | 0.7 |
| Bash | tree-sitter-bash | 0.23 |
| Markdown | tree-sitter-md | 0.3 |

## GPUI Concepts

### Elements vs Views

```rust
// Element (stateless, RenderOnce)
impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement { ... }
}

// View (stateful, Entity-based)
impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement { ... }
}
```

### Styling (Tailwind-like)

```rust
div()
    .size_full()
    .flex()
    .flex_row()
    .gap_2()
    .p_4()
    .bg(theme.colors.background)
    .border_1()
    .border_color(theme.colors.border)
    .rounded_lg()
```

### Actions

```rust
actions!(app_name, [ActionName, AnotherAction]);

cx.on_action(|_: &ActionName, cx| {
    // Handle action
});
```

## Claude Code Integration

```rust
// Spawn with stream-json output
let mut cmd = Command::new("claude");
cmd.args(["-p", prompt, "--output-format", "stream-json"]);
cmd.current_dir(cwd);
cmd.stdout(Stdio::piped());

// Parse JSON lines
let reader = BufReader::new(stdout);
let mut lines = reader.lines();
while let Some(line) = lines.next_line().await? {
    let event: ClaudeEvent = serde_json::from_str(&line)?;
    // Handle event
}
```

## Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With plugin support
cargo build --features plugins

# Run
cargo run

# Run release
cargo run --release
```

## Platform Support

| Platform | Backend | Status |
|----------|---------|--------|
| macOS | Metal | ✅ Primary |
| Linux | Vulkan | 🔄 Planned |
| Windows | - | ❌ Not supported (GPUI limitation) |
