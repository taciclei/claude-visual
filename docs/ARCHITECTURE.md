# Architecture

## Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────┐
│                      Claude Visual                          │
├─────────────────────────────────────────────────────────────┤
│  UI Layer (GPUI)                                            │
│  ┌─────────┐ ┌─────────────────────────────────────────┐   │
│  │ Sidebar │ │              Chat View                   │   │
│  │         │ │  ┌─────────────────────────────────┐    │   │
│  │ Projects│ │  │ Message Blocks (Warp-style)     │    │   │
│  │         │ │  │  - Code blocks (tree-sitter)    │    │   │
│  │ Worktree│ │  │  - Markdown (pulldown-cmark)    │    │   │
│  │         │ │  │  - File previews                │    │   │
│  │         │ │  │  - Diff views                   │    │   │
│  │         │ │  └─────────────────────────────────┘    │   │
│  │         │ │  ┌─────────────────────────────────┐    │   │
│  │         │ │  │ Input Area                      │    │   │
│  └─────────┘ └──┴─────────────────────────────────┴────┘   │
├─────────────────────────────────────────────────────────────┤
│  Core Services                                              │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      │
│  │  Claude  │ │  Project │ │   Git    │ │ Storage  │      │
│  │  Client  │ │  Manager │ │  Ops     │ │ (SQLite) │      │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘      │
├─────────────────────────────────────────────────────────────┤
│  Plugin System (optional)                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  WASM Host (wasmtime) - Zed-compatible extensions    │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Structure des modules

```
src/
├── main.rs                 # Entry point, GPUI init
├── app/
│   ├── mod.rs
│   ├── state.rs           # AppState global (Entity-based)
│   ├── settings.rs        # User preferences
│   └── theme.rs           # Theme management
├── ui/
│   ├── mod.rs
│   ├── workspace.rs       # Main layout
│   ├── sidebar/
│   │   ├── projects.rs    # Project list
│   │   └── worktrees.rs   # Git worktree panel
│   ├── chat/
│   │   ├── view.rs        # Chat container
│   │   ├── message.rs     # Message bubbles
│   │   └── input.rs       # Multi-line input
│   ├── blocks/
│   │   ├── code_block.rs  # Syntax highlighted
│   │   ├── file_block.rs  # File preview
│   │   └── diff_block.rs  # Diff view
│   └── components/
│       ├── button.rs
│       ├── input.rs
│       ├── modal.rs
│       └── tooltip.rs
├── claude/
│   ├── mod.rs
│   ├── client.rs          # Process management
│   ├── message.rs         # Event types
│   └── streaming.rs       # Async stream handler
├── project/
│   ├── mod.rs
│   ├── manager.rs         # CRUD projects
│   ├── config.rs          # Per-project config
│   └── recent.rs          # Recent tracking
├── git/
│   ├── mod.rs
│   ├── repository.rs      # Git operations
│   ├── worktree.rs        # Worktree CRUD
│   └── status.rs          # File status
├── markdown/
│   ├── mod.rs
│   ├── parser.rs          # pulldown-cmark
│   └── renderer.rs        # GPUI rendering
├── storage/
│   ├── mod.rs
│   ├── database.rs        # SQLite ops
│   └── models.rs          # DB models
└── plugins/
    ├── mod.rs             # Extension manifest
    ├── host.rs            # WASM host
    ├── loader.rs          # Discovery/install
    └── registry.rs        # Extension registry
```

## Data Flow

```
User Input
    │
    ▼
┌─────────────┐
│ Chat Input  │
└─────────────┘
    │
    ▼
┌─────────────┐     ┌─────────────┐
│ Claude      │────▶│ stream-json │
│ Client      │     │ output      │
└─────────────┘     └─────────────┘
    │                     │
    ▼                     ▼
┌─────────────┐     ┌─────────────┐
│ ClaudeEvent │────▶│ Message     │
│ Stream      │     │ Blocks      │
└─────────────┘     └─────────────┘
                          │
                          ▼
                    ┌─────────────┐
                    │ UI Update   │
                    │ (GPUI)      │
                    └─────────────┘
```

## State Management

GPUI utilise un modèle Entity-based:

```rust
pub struct AppState {
    pub settings: Entity<Settings>,
    pub theme: Entity<Theme>,
    pub project_manager: Entity<ProjectManager>,
    pub database: Arc<Database>,
    pub current_directory: Arc<RwLock<Option<PathBuf>>>,
}
```

- `Entity<T>` = référence partagée avec change tracking
- `cx.new(|cx| ...)` = créer une nouvelle Entity
- `entity.read(cx)` = lire l'état
- `entity.update(cx, |state, cx| ...)` = modifier l'état
