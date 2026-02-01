# Claude Visual

[![CI](https://github.com/taciclei/claude-visual/actions/workflows/ci.yml/badge.svg)](https://github.com/taciclei/claude-visual/actions/workflows/ci.yml)
[![Security Audit](https://github.com/taciclei/claude-visual/actions/workflows/security.yml/badge.svg)](https://github.com/taciclei/claude-visual/actions/workflows/security.yml)
[![Nightly](https://github.com/taciclei/claude-visual/actions/workflows/nightly.yml/badge.svg)](https://github.com/taciclei/claude-visual/actions/workflows/nightly.yml)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)

A visual client for Claude Code CLI, inspired by Warp terminal. Built with Rust and GPUI for a GPU-accelerated, modern user experience.

## Features

- **Warp-style Blocks**: Each Claude exchange is a collapsible block with copy, share, and action buttons
- **Streaming Responses**: Real-time display of Claude's responses with progress indicators
- **Syntax Highlighting**: Tree-sitter powered code highlighting for 10+ languages
- **Markdown Rendering**: Full CommonMark support with pulldown-cmark
- **Claude Code Skills Integration**: Quick access to all Claude Code skills (/apex, /explore, /debug, /commit, /review, /refactor, /oneshot, /ultrathink, /brainstorm)
- **Command Palette**: Cmd+K opens a searchable command palette with all actions
- **Project Management**: Organize projects with favorites, tags, and recent access
- **Git Worktree Support**: Manage and switch between git worktrees
- **Dark/Light Themes**: Customizable color schemes
- **Conversation History**: SQLite-backed history with full-text search
- **Task Management**: Visual task tracking integrated with Claude Code's TodoWrite

## Screenshots

*Coming soon*

## Installation

### From Releases

Download the latest release for your platform:

- **macOS Apple Silicon**: `claude-visual-macos-arm64.dmg`
- **macOS Intel**: `claude-visual-macos-x64.dmg`
- **Linux**: `claude-visual-linux-x86_64.AppImage` or `.deb`

### From Source

```bash
# Clone the repository
git clone https://github.com/taciclei/claude-visual.git
cd claude-visual

# Build and run
cargo build --release
./target/release/claude-visual
```

## Requirements

- Rust stable (1.75+)
- macOS (GPUI Metal backend) or Linux (Vulkan)
- [Claude Code CLI](https://github.com/anthropics/claude-code) installed and in PATH

### Linux Dependencies

```bash
sudo apt-get install -y \
  libgtk-3-dev \
  libvulkan-dev \
  libxkbcommon-dev \
  libwayland-dev \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev
```

## Building

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Build with plugin support
cargo build --features plugins

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Project Structure

```
claude-visual/
├── src/
│   ├── main.rs               # Entry point
│   ├── app/                   # App state, settings, themes
│   ├── ui/                    # GPUI components
│   │   ├── workspace.rs      # Main layout
│   │   ├── sidebar/          # Project, history, and worktree panels
│   │   ├── chat/             # Chat view, input, command palette
│   │   ├── blocks/           # Code, file, and diff blocks
│   │   └── components/       # Reusable UI components
│   ├── claude/               # Claude CLI integration
│   ├── project/              # Project management
│   ├── git/                  # Git operations
│   ├── markdown/             # Markdown parsing/rendering
│   └── storage/              # SQLite database
├── assets/                   # Icons, fonts, themes
├── packaging/                # Linux packaging (AppImage, Flatpak, Debian)
└── scripts/                  # Build scripts
```

## Configuration

Settings are stored in `~/.config/claude-visual/settings.toml`:

```toml
theme = "dark"
code_font_family = "JetBrains Mono"
code_font_size = 14.0
ui_font_family = "Inter"
ui_font_size = 14.0
show_sidebar = true
vim_mode = false
auto_save_conversations = true
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+N` | New conversation |
| `Cmd+O` | Open project |
| `Cmd+K` | Command palette |
| `Cmd+B` | Toggle sidebar |
| `Cmd+Enter` | Send message |
| `Cmd+Shift+Enter` | Send with @codebase |
| `Escape` | Cancel/Close |
| `Cmd+Q` | Quit |

## Claude Code Skills

Claude Visual provides quick access to all Claude Code skills:

| Skill | Description |
|-------|-------------|
| `/apex` | Systematic implementation with APEX methodology |
| `/explore` | Deep codebase exploration |
| `/debug` | Error analysis and bug resolution |
| `/commit` | Generate conventional commits |
| `/review` | Deep code review |
| `/refactor` | Code refactoring with parallel agents |
| `/oneshot` | Ultra-fast feature implementation |
| `/ultrathink` | Deep thinking mode |
| `/brainstorm` | Iterative research and reflection |

## Tech Stack

| Component | Technology |
|-----------|------------|
| UI Framework | [GPUI](https://gpui.rs/) (Zed's GPU-accelerated UI) |
| Markdown | pulldown-cmark |
| Syntax Highlighting | tree-sitter (Rust, JS, TS, Python, JSON, TOML, Bash, Markdown) |
| Git | git2-rs |
| Storage | SQLite (rusqlite) |
| Config | TOML |
| Plugins | wasmtime (optional) |
| Async Runtime | Tokio |

## Plugin Support

Claude Visual includes optional support for Zed-compatible WASM extensions.

```bash
# Build with plugin support
cargo build --features plugins
```

Plugins can provide:
- Language support (syntax highlighting, LSP)
- Themes
- Slash commands
- Snippets

Extensions are installed to `~/.local/share/claude-visual/extensions/`

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [GPUI](https://gpui.rs/) - Zed's GPU-accelerated UI framework
- [Claude Code](https://github.com/anthropics/claude-code) - Anthropic's CLI for Claude
- [Warp](https://www.warp.dev/) - Design inspiration
