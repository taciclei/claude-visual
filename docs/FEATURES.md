# Features

## Interface Claude Code Améliorée

### Blocks (style Warp)
Chaque échange avec Claude est un block collapsible:
- Header avec timestamp et status
- Contenu expandable/collapsible
- Actions rapides (copier, re-run, delete)
- Indicateur de streaming en cours

### Streaming
Affichage en temps réel des réponses Claude:
- Texte apparaît au fur et à mesure
- Code blocks se construisent progressivement
- Indicateur visuel de génération

### Code Blocks
Syntax highlighting avec tree-sitter:
- Coloration par langage détecté
- Numéros de ligne
- Bouton copier
- Bouton exécuter (bash/shell)
- Bouton sauvegarder vers fichier

### Historique
Sessions sauvegardées et recherchables:
- SQLite pour persistence
- Recherche full-text
- Filtrage par projet/date
- Export en Markdown

## Rendu Markdown

### Support CommonMark
- Headers (h1-h6)
- Listes (ordonnées, non-ordonnées, nested)
- Code inline et code blocks
- Emphasis (bold, italic, strikethrough)
- Links et images
- Blockquotes
- Tables
- Horizontal rules

### Code Blocks
```rust
// Syntax highlighting automatique
fn main() {
    println!("Hello, world!");
}
```

### Mermaid (Phase 2)
Support diagrammes:
- Flowcharts
- Sequence diagrams
- Class diagrams

## Gestion de Projets

### Organisation
- Favoris (étoile)
- Récents (auto-tracked)
- Tags personnalisés
- Catégories

### Actions
- Ajouter: drag & drop ou file picker
- Switch: change le cwd de Claude
- Config: `.claude-visual.toml` par projet
- Supprimer: retire de la liste (pas du disque)

### Configuration par Projet
```toml
# .claude-visual.toml
theme = "project-specific-theme"
default_model = "claude-3-opus"
auto_save = true

[shortcuts]
build = "cargo build"
test = "cargo test"
```

## Git Worktree

### Visualisation
- Liste des worktrees actifs
- Branch associée
- Status (clean/dirty)
- Chemin sur disque

### Gestion
- Créer depuis une branche
- Créer nouvelle branche + worktree
- Supprimer worktree
- Switch = change de projet actif

### Status
- Fichiers modifiés
- Fichiers staged
- Commits ahead/behind

## UI Moderne

### Thèmes
- Dark mode (défaut)
- Light mode
- Custom themes (JSON)
- Per-project override

### Fonts
```toml
[fonts]
code = "JetBrains Mono"
ui = "Inter"
code_size = 14.0
ui_size = 14.0
```

### Raccourcis Clavier

| Shortcut | Action |
|----------|--------|
| `Cmd+N` | Nouvelle conversation |
| `Cmd+O` | Ouvrir projet |
| `Cmd+K` | Command palette |
| `Cmd+B` | Toggle sidebar |
| `Cmd+,` | Settings |
| `Cmd+Q` | Quit |
| `Cmd+Enter` | Envoyer message |
| `Escape` | Annuler/fermer |

### Command Palette
Accès rapide à toutes les actions:
- Fuzzy search
- Recent commands
- Keyboard navigation

### Split Views (Phase 4)
- Multiple chats côte à côte
- Tabs pour plusieurs sessions
- Drag & drop pour réorganiser

## Configuration

### Fichier Settings
`~/.config/claude-visual/settings.toml`:

```toml
# Appearance
theme = "dark"
show_sidebar = true
sidebar_width = 250

# Fonts
code_font_family = "JetBrains Mono"
code_font_size = 14.0
ui_font_family = "Inter"
ui_font_size = 14.0

# Behavior
vim_mode = false
auto_save_conversations = true
confirm_before_close = true

# Claude
default_model = "claude-3-sonnet"
stream_responses = true

# Git
show_git_status = true
auto_detect_worktrees = true
```

## Accessibility

- Keyboard navigation complète
- High contrast themes
- Scalable UI (Cmd+/Cmd-)
- Screen reader hints (planned)
