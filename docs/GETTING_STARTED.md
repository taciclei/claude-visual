# Getting Started

## Prérequis

### Système
- macOS (Metal) ou Linux (Vulkan)
- Rust 1.75+ (via rustup)

### Outils
- [Claude Code CLI](https://github.com/anthropics/claude-code) installé
- Git

### macOS
```bash
# Xcode command line tools
xcode-select --install

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Linux
```bash
# Debian/Ubuntu
sudo apt install build-essential cmake libssl-dev pkg-config

# Vulkan SDK
sudo apt install libvulkan-dev

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

```bash
# Clone
git clone https://github.com/your-repo/claude-visual.git
cd claude-visual

# Build
cargo build --release

# Run
cargo run --release
```

## Premier Lancement

1. L'application crée automatiquement:
   - `~/.config/claude-visual/settings.toml`
   - `~/.local/share/claude-visual/` (database, extensions)

2. Ajouter un projet:
   - Cliquer sur "+" dans la sidebar
   - Ou drag & drop un dossier

3. Commencer une conversation:
   - Taper dans la zone de texte
   - `Cmd+Enter` pour envoyer

## Configuration Rapide

### Changer le thème
```toml
# ~/.config/claude-visual/settings.toml
theme = "light"  # ou "dark"
```

### Changer les fonts
```toml
code_font_family = "Fira Code"
code_font_size = 13.0
```

### Activer vim mode
```toml
vim_mode = true
```

## Raccourcis Essentiels

| Action | Shortcut |
|--------|----------|
| Nouvelle conversation | `Cmd+N` |
| Envoyer message | `Cmd+Enter` |
| Toggle sidebar | `Cmd+B` |
| Command palette | `Cmd+K` |
| Quit | `Cmd+Q` |

## Troubleshooting

### "Claude CLI not found"
```bash
# Vérifier que claude est dans le PATH
which claude

# Sinon, l'installer
npm install -g @anthropic-ai/claude-code
```

### "Failed to create window"
- macOS: Vérifier les permissions System Preferences > Security
- Linux: Vérifier que Vulkan est installé

### "Database error"
```bash
# Reset database
rm ~/.local/share/claude-visual/claude-visual.db
```

## Next Steps

- Lire [FEATURES.md](./FEATURES.md) pour découvrir toutes les fonctionnalités
- Lire [ARCHITECTURE.md](./ARCHITECTURE.md) pour comprendre le code
- Voir [ROADMAP.md](./ROADMAP.md) pour les plans futurs
