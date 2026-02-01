# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release build with LTO
cargo run                # Run debug build
cargo build --features plugins  # Build with WASM plugin support
```

## Architecture

This is a Rust desktop application using GPUI for GPU-accelerated UI. It provides a visual interface for Claude Code CLI, inspired by Warp terminal.

### Core Modules

- **app/** - Global state management (`AppState` holds settings, theme, project manager, database), user settings, and theming
- **ui/** - GPUI components following a hierarchical structure:
  - `workspace.rs` - Main layout containing sidebar and chat view
  - `sidebar/` - Project list and git worktree panels
  - `chat/` - Chat view, message rendering, input handling
  - `blocks/` - Specialized blocks (code, file, diff) for Claude responses
  - `components/` - Reusable primitives (button, input, modal, tooltip)
- **claude/** - CLI integration via `ClaudeClient`:
  - Spawns `claude` process with `--output-format stream-json`
  - Parses streaming JSON events (content_block_delta, tool_use, tool_result)
  - Returns async Stream of `ClaudeEvent` for UI consumption
- **project/** - Project management with favorites, tags, recent access tracking
- **git/** - Git operations via git2-rs (repository, worktree, status)
- **markdown/** - Markdown parsing (pulldown-cmark) and GPUI rendering
- **storage/** - SQLite database for conversations and project metadata
- **plugins/** - Optional WASM plugin system compatible with Zed extensions (requires `--features plugins`)

### Data Flow

1. User input → `ChatView` → `Workspace.send_message()`
2. `ClaudeClient.send_prompt()` spawns claude CLI process
3. Streaming JSON events parsed into `ClaudeEvent` variants
4. Events flow back to `ChatView.handle_claude_event()` for rendering

### File Locations

- Settings: `~/.config/claude-visual/settings.toml`
- Database: `~/.local/share/claude-visual/` (SQLite)
- Extensions: `~/.local/share/claude-visual/extensions/`

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| gpui | UI framework (Metal/Vulkan) |
| tokio | Async runtime for CLI communication |
| tree-sitter-* | Syntax highlighting (Rust, JS, TS, Python, JSON, TOML, Bash, Markdown) |
| git2 | Git operations |
| rusqlite | Persistent storage |
| pulldown-cmark | Markdown parsing |
| wasmtime | WASM plugin runtime (optional) |

## Platform Requirements

- macOS (Metal backend) or Linux (Vulkan)
- Rust 1.75+
- Claude Code CLI must be installed and available in PATH
