# Plugin System

## Overview

Claude Visual supporte les extensions WASM compatibles avec Zed, utilisant wasmtime comme runtime.

## Activation

```bash
# Build with plugin support
cargo build --features plugins
```

## Extension Format

Les extensions suivent le format Zed:

```
my-extension/
├── extension.toml      # Manifest
├── extension.wasm      # Compiled WASM (optional)
├── themes/             # Theme definitions
├── languages/          # Language configs
└── grammars/           # Tree-sitter grammars
```

### extension.toml

```toml
id = "my-extension"
name = "My Extension"
version = "0.1.0"
authors = ["Author Name"]
description = "Extension description"
repository = "https://github.com/user/repo"

[lib]
kind = "rust"
version = "0.2.0"

[[themes]]
path = "themes/dark.json"

[[languages]]
path = "languages/mylang"

[grammars.mylang]
repository = "https://github.com/user/tree-sitter-mylang"
rev = "abc123"
```

## Capabilities

| Capability | Status | Description |
|------------|--------|-------------|
| Themes | ✅ Ready | Custom color schemes |
| Languages | 🔄 Planned | Syntax highlighting, LSP |
| Slash Commands | 🔄 Planned | Custom / commands |
| Snippets | 🔄 Planned | Code snippets |

## API Usage

```rust
use claude_visual::plugins::{ExtensionRegistry, PluginHost};

// Initialize registry
let registry = ExtensionRegistry::new()?;
registry.initialize()?;

// Install extension
registry.install(Path::new("/path/to/extension"))?;

// List installed
for manifest in registry.list_installed() {
    println!("{}: {}", manifest.id, manifest.name);
}

// Enable/disable
registry.enable("extension-id")?;
registry.disable("extension-id")?;
```

## Extension Directories

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/claude-visual/extensions/` |
| Linux | `~/.local/share/claude-visual/extensions/` |

## Zed Compatibility

### Compatible
- Theme files (JSON format)
- Language configurations
- Tree-sitter grammars (.wasm)
- Extension manifest format

### Not Compatible (yet)
- Full WIT interface (partial implementation)
- Worktree resource access
- LSP management
- Slash command registration

## Creating an Extension

### 1. Rust Extension (WASM)

```rust
use zed_extension_api::{self as zed, Extension};

struct MyExtension;

impl Extension for MyExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(MyExtension);
```

### 2. Theme-only Extension

```toml
# extension.toml
id = "my-theme"
name = "My Theme"
version = "1.0.0"
authors = ["Your Name"]

[[themes]]
path = "themes/my-theme.json"
```

```json
// themes/my-theme.json
{
  "name": "My Theme",
  "appearance": "dark",
  "style": {
    "background": "#1e1e2e",
    "foreground": "#cdd6f4",
    ...
  }
}
```

## Architecture

```
┌─────────────────────────────────────────┐
│           ExtensionRegistry             │
│  - Manages installed extensions         │
│  - Enable/disable tracking              │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│           ExtensionLoader               │
│  - Discovers extensions on disk         │
│  - Installs/uninstalls                  │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│             PluginHost                  │
│  - wasmtime Engine                      │
│  - WASM module loading                  │
│  - Host function implementations        │
└─────────────────────────────────────────┘
```

## Future Plans

1. **Full WIT Interface** - Implement complete Zed extension API
2. **Hot Reload** - Reload extensions without restart
3. **Extension Store** - Browse and install from registry
4. **UI Extensions** - Custom panels and views
