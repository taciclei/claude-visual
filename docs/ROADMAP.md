# Claude Visual - Roadmap

## Vision

Un client visuel moderne pour Claude Code, inspiré de Warp terminal, avec support GPU via GPUI.

---

## État Actuel - Janvier 2025

### Phase 1: Foundation (MVP) ✅ COMPLETE
- [x] Setup projet Rust + GPUI
- [x] Structure des modules (app, ui, claude, project, git, markdown, storage, plugins)
- [x] Spawner Claude Code en process avec stream-json
- [x] Parser output stream-json (ClaudeEvent variants)
- [x] UI basique: workspace, sidebar, chat
- [x] Composants de base (button, input, modal, tooltip)
- [x] Theme system (dark/light modes)
- [x] Database schema SQLite

### Phase 2: Core Features ✅ COMPLETE
- [x] Text input fonctionnel avec FocusHandle
- [x] Event system (ChatInput → ChatView → Workspace)
- [x] Sidebar projets cliquable avec recherche filtrée
- [x] Code blocks interactifs (Entity-based, collapse/copy)
- [x] Syntax highlighting tree-sitter (8 langages: Rust, JS, TS, TSX, Python, JSON, TOML, Bash)
- [x] Raccourcis clavier (Cmd+N, Cmd+B, Cmd+Q, Cmd+K, Cmd+O)
- [x] Persistence conversations SQLite (auto-save messages)
- [x] History sidebar avec liste conversations
- [x] Sidebar tabs (Projects/History) avec switch dynamique
- [x] Markdown rendering complet (CommonMark)

---

## Phase 3: Enhanced UX ✅ COMPLETE

### 3.1: Collapsible Message Blocks ✅ COMPLETE
**But:** Convertir MessageView en Entity pour permettre collapse/expand

**Fichiers:**
- `src/ui/chat/message.rs` - Convertir RenderOnce → Render + Entity
- `src/ui/chat/view.rs` - Gérer Vec<Entity<MessageView>>

**Tâches:**
1. [x] Ajouter struct MessageViewState avec collapsed: bool
2. [x] Implémenter toggle_collapsed()
3. [x] Header cliquable avec chevron indicator
4. [x] Actions rapides (copy button)
5. [x] Distinguer visuellement user/assistant/tool blocks (colored borders)

### 3.2: Streaming Indicator ✅ COMPLETE
**But:** Feedback visuel pendant la génération Claude

**Fichiers:**
- `src/ui/chat/view.rs` - Ajouter spinner/pulse animation
- `src/ui/chat/input.rs` - Désactivation pendant streaming

**Tâches:**
1. [x] Afficher "Claude is thinking..." pendant is_streaming = true
2. [x] Pulsing dot indicator
3. [x] Streaming message view pendant génération
4. [x] Désactiver input pendant streaming
5. [x] Bouton "Stop" pour cancel le stream

### 3.3: File Picker Native ✅ COMPLETE
**But:** Dialog natif pour ajouter projets

**Fichiers:**
- `src/ui/workspace.rs` - Handler open_project_picker()
- `Cargo.toml` - Added rfd dependency

**Tâches:**
1. [x] Intégrer rfd crate
2. [x] Handler AddProjectRequested → ouvrir picker
3. [x] Filtrer sur directories only
4. [x] Valider que le path existe
5. [x] Auto-save project au database

### 3.4: Enhanced Code Blocks (Priority: MEDIUM) ✅ COMPLETE (Basic)
**But:** Actions avancées sur les blocks de code

**Fichiers:**
- `src/ui/blocks/code_block.rs`
- `src/ui/workspace.rs`
- `src/main.rs`

**Tâches:**
1. [x] Bouton "Run" pour bash/shell (emits CodeBlockEvent::Execute)
2. [x] Bouton "Save" avec event pour file picker (emits CodeBlockEvent::SaveToFile)
3. [x] Handle actions in Workspace (execute command via tokio, show file dialog via rfd)
4. [x] Diff view mode pour les modifications
   - CodeDisplayMode enum (Normal, Diff)
   - LineChangeType enum (Context, Added, Removed, ModifiedOld, ModifiedNew)
   - DiffLine struct with content, change_type, line numbers
   - compute_diff() method for generating diff lines
   - with_diff() constructor for diff code blocks
   - toggle_display_mode() for switching views
   - Diff button in header with +/- stats
   - Two-column line numbers (old/new)
   - Prefix column with +/-/space
   - Color-coded backgrounds (green/red/neutral)
   - Syntax highlighting in diff view
   - Unit tests for diff computation
5. [x] Line highlighting pour les références - COMPLETE
   - HighlightedRange struct with start_line, end_line, style, label
   - HighlightStyle enum (Reference, Error, Warning, Success, Info, Emphasis)
   - with_highlights() constructor for pre-configured highlights
   - Methods: add_highlight(), highlight_line(), highlight_range(), clear_highlights()
   - get_line_highlight() for checking if line has highlight
   - highlight_bg_color() and highlight_border_color() per style
   - Highlight gutter indicator column in render
   - Color-coded backgrounds based on style
   - Unit tests for highlight management
6. [x] Search dans le code block (`src/ui/blocks/code_block.rs`)
   - Search button in code block header
   - SearchMatch struct for tracking match locations
   - Search query handling with case-insensitive matching
   - Match navigation (next/previous)
   - Line highlighting for matching lines
   - Current match highlighting with accent color
   - Match count display

### 3.5: Command Palette (Cmd+K) ✅ COMPLETE
**But:** Fuzzy search pour actions rapides

**Fichiers:**
- `src/ui/components/command_palette.rs` (nouveau)
- `src/ui/workspace.rs`

**Tâches:**
1. [x] Modal overlay avec input
2. [x] Fuzzy search sur toutes actions
3. [x] Keyboard navigation (↑/↓/Enter/Esc)
4. [x] Shortcuts affichés à côté des actions
5. [x] Execute commands (new conversation, toggle sidebar, toggle theme, etc.)

---

## Phase 4: Git Integration 🔄 IN PROGRESS

### 4.1: Repository Detection ✅ COMPLETE
**Fichier:** `src/git/repository.rs`

**Tâches:**
1. [x] Détecter si projet est un git repo (Repository::is_repo)
2. [x] Afficher branch courante (Repository::current_branch)
3. [x] Status indicator (Repository::status)

### 4.2: Worktree Panel UI ✅ COMPLETE
**Fichier:** `src/ui/sidebar/worktrees.rs`, `src/ui/workspace.rs`

**Tâches:**
1. [x] Liste des worktrees avec branch name
2. [x] Indicateur locked par worktree
3. [x] Click pour switch de working directory (emits WorktreePanelEvent)
4. [x] Bouton "Create Worktree" (emits event)
5. [x] Wire panel to workspace (Git tab in sidebar)
6. [x] Context menu (delete, copy branch, copy path)

### 4.3: File Status Display ✅ COMPLETE
**Fichiers:** `src/git/status.rs`, `src/ui/sidebar/worktrees.rs`

**Tâches:**
1. [x] Modified files (M) avec status char
2. [x] Added files (A)
3. [x] Deleted files (D)
4. [x] Untracked files (?)
5. [x] Staged indicator (separate section)
6. [x] Branch display avec modified indicator
7. [x] Clickable file items (emits FileClicked event)
8. [x] Diff preview on click

### 4.4: Diff Block ✅ COMPLETE
**Fichier:** `src/ui/blocks/diff_block.rs`

**Tâches:**
1. [x] Unified diff format (parsing +/- lines with hunk support)
2. [x] Color coding (green added, red removed, gray context)
3. [x] Stats display (+additions/-deletions badges)
4. [x] Collapse/expand toggle (Entity-based, clickable header)
5. [x] Line numbers (old/new) with proper alignment
6. [x] Syntax highlighting dans le diff
7. [x] Side-by-side option

---

## Phase 5: Polish & Productivity

### 5.1: Command Palette (Cmd+K) ✅ MOVED TO PHASE 3
**Note:** Implemented in Phase 3.5

### 5.2: Settings UI ✅ COMPLETE
**Fichier:** `src/ui/settings/mod.rs`

**Tâches:**
1. [x] Visual editor pour settings.toml
2. [x] Categories: Appearance, Editor, Git, Claude
3. [x] Live preview des changements de thème
4. [x] Toggle switches, font selectors, sliders
5. [x] Keyboard shortcut (Cmd+,)
6. [x] Reset to defaults button (`src/ui/settings/mod.rs`)
   - Reset button in footer
   - Confirmation dialog with warning
   - reset_to_defaults() method
7. [x] Import/Export settings

### 5.3: Conversation Search ✅ COMPLETE
**Fichiers:**
- `src/storage/database.rs` - FTS5 virtual table + triggers
- `src/storage/models.rs` - SearchResult struct
- `src/ui/sidebar/history.rs` - Search UI

**Tâches:**
1. [x] Full-text search SQLite FTS5 (virtual table + sync triggers)
2. [x] Search par contenu de messages (search_messages method)
3. [x] Search UI avec results display
4. [x] Highlighting des matches (FTS5 highlight function)
5. [x] Interactive search input with keyboard (on_key_down + on_input handlers)
6. [x] Filtres par date/projet
   - DateRangeFilter enum (AllTime, Today, LastWeek, LastMonth, LastQuarter, LastYear)
   - SearchFilter struct with date_range and project_id
   - search_messages_with_filter() method in database.rs
   - Filter panel toggle in history sidebar
   - Date range chips with visual selection
   - Project filter dropdown with all projects
   - Clear filters button when filters active
   - Automatic re-search on filter change

### 5.4: Export Markdown ✅ COMPLETE
**Fichiers:** `src/ui/chat/view.rs`, `src/ui/workspace.rs`

**Tâches:**
1. [x] Export conversation complète
2. [x] Format Markdown propre avec headers
3. [x] Code blocks préservés avec language tags
4. [x] Timestamps inclus
5. [x] Save dialog avec nom fichier

### 5.5: Drag & Drop ✅ COMPLETE
**Fichiers:**
- `src/ui/sidebar/projects.rs`
- `src/ui/workspace.rs`

**Tâches:**
1. [x] Drop zone dans sidebar (ExternalPaths handler)
2. [x] Accepter dossiers (filter to directories only)
3. [x] Visual feedback pendant drag (overlay with icon and text)
4. [x] Auto-add au project list (via FilesDropped event)

---

## Phase 6: Plugin System ✅ COMPLETE

### 6.1: WASM Host Setup ✅ COMPLETE
**Fichier:** `src/plugins/host.rs`

**Tâches:**
1. [x] Initialize wasmtime Engine (async + component model)
2. [x] Load compiled WASM modules
3. [x] Host state with extension ID and workdir
4. [x] Load/unload extension methods
5. [x] Extension capabilities enum

### 6.2: Extension Loader ✅ COMPLETE
**Fichier:** `src/plugins/loader.rs`

**Tâches:**
1. [x] Scan extension directories (discover method)
2. [x] Parse extension.toml manifests
3. [x] Install from local path
4. [x] Uninstall extensions
5. [x] Check if installed

### 6.3: Theme Support ✅ COMPLETE
**Fichier:** `src/plugins/themes.rs`

**Tâches:**
1. [x] Load Zed theme JSON format (ZedThemeFile parser)
2. [x] Convert to internal ThemeColors (convert_variant)
3. [x] Parse hex and rgba colors
4. [x] Convert syntax highlighting styles
5. [ ] Hot reload on file change (requires external theme files - deferred)
6. [x] Theme picker UI
7. [x] Per-project theme override (enhancement)

### 6.4: Slash Commands API ✅ COMPLETE
**Fichier:** `src/plugins/commands.rs`

**Tâches:**
1. [x] Register custom /commands (CommandRegistry)
2. [x] Built-in commands (/help, /clear, /export, /theme, /project, /model)
3. [x] Command handlers with context (CommandHandler type)
4. [x] CommandResult types (Text, Markdown, Code, Error, Silent)
5. [x] Autocomplete suggestions (list_for_autocomplete)
6. [x] Parse command from input (parse_command)
7. [ ] Execute WASM command handlers (needs PluginHost integration)

---

## Phase 7: Advanced Features ✅ COMPLETE

### 7.1: Multi-Tab Conversations ✅ COMPLETE
**Fichier:** `src/ui/tabs/mod.rs`

**Tâches:**
1. [x] Tab bar component avec tab items (TabBar Entity)
2. [x] Entity<ChatView> pour chaque conversation active
3. [x] Keyboard shortcuts (Cmd+T new, Cmd+W close, Cmd+1-9 switch)
4. [x] Ctrl+Tab / Cmd+Shift+] for next tab
5. [x] Ctrl+Shift+Tab / Cmd+Shift+[ for previous tab
6. [x] Drag & drop pour réorganiser (DraggedTab)
7. [x] Tab title updates with conversation preview
8. [x] Dirty indicator for unsaved changes
9. [x] Tab overflow menu pour trop de tabs
10. [x] Pin tabs (conversations importantes)

### 7.2: Vim Mode ✅ COMPLETE
**Fichier:** `src/ui/vim/mod.rs`

**Tâches:**
1. [x] Modal state (Normal, Insert, Visual, VisualLine, VisualBlock, Command, Search)
2. [x] Normal mode: hjkl navigation, dd delete line, yy yank, word motions (w, b, e)
3. [x] Insert mode: standard input with Esc to exit
4. [x] Visual mode: text selection with d/y/c actions
5. [x] Command mode: Ex commands with Enter to execute
6. [x] Search mode: /pattern with Enter to search
7. [x] VimKeyHandler for key sequences
8. [x] VimState for state management
9. [x] VimStatusLine component with mode indicator
10. [x] Count prefix support (e.g., 5j moves down 5 lines)
11. [x] Operator + motion (d/y/c with movements)
12. [x] Unit tests for keymaps
13. [x] Integration with ChatInput (enhancement)
14. [x] Configurable keymaps (enhancement)

### 7.3: Split Views ✅ COMPLETE
**Fichier:** `src/ui/split/mod.rs`

**Tâches:**
1. [x] SplitView Entity with SplitNode tree structure
2. [x] SplitDirection (Horizontal/Vertical)
3. [x] Pane struct with id, weight, is_focused
4. [x] Recursive node rendering with dividers
5. [x] Focus management (focus_next, focus_prev, focus_pane)
6. [x] Split operations (split_horizontal, split_vertical)
7. [x] Close pane functionality
8. [x] Max 4 panes limit
9. [x] Resize dividers (UI prepared, cursor styles)
10. [x] Unit tests for pane count and focus
11. [x] Keyboard shortcuts (Cmd+\, Cmd+Shift+\, Cmd+Alt+Arrow, Cmd+Shift+W)
12. [x] Actual resize drag handler (enhancement)
13. [x] Integration with Workspace ChatViews (enhancement)

---

## Phase 8: MCP Integration ✅ COMPLETE

### 8.1: MCP Client Setup ✅ COMPLETE
**Fichiers:** `src/mcp/mod.rs`, `src/mcp/client.rs`, `src/mcp/protocol.rs`, `src/mcp/config.rs`

**Tâches:**
1. [x] Implement MCP client protocol (JSON-RPC 2.0 over stdio)
2. [x] Server discovery (mcp.json config with McpConfig)
3. [x] Connection lifecycle management (start, stop, initialize)
4. [x] Capability negotiation (ServerCapabilities, McpCapabilities)
5. [x] McpManager for managing multiple server connections
6. [x] Presets for common servers (filesystem, github, sqlite, brave_search, fetch, memory, puppeteer)

### 8.2: MCP Servers UI ✅ COMPLETE
**Fichiers:** `src/ui/mcp/mod.rs`, `src/ui/mcp/servers.rs`

**Tâches:**
1. [x] Server list panel (McpServersPanel Entity)
2. [x] Connection status indicators (color dots: gray/yellow/green/red)
3. [x] Enable/disable toggle per server
4. [x] Connect/disconnect buttons
5. [x] Server item display (name, command, tool/resource counts)
6. [x] Error display for failed connections
7. [x] Expandable panel with header
8. [x] Add server button (emits AddServer event)
9. [x] Server configuration editor (`src/ui/mcp/server_config.rs`)
   - ServerConfigEditor Entity for editing server configuration
   - EditingServerConfig struct for edit state
   - EditingField enum for tracking focused field
   - Fields: name, command, args, env, description, auto_approve, enabled
   - Multiline text areas for args, env, auto_approve
   - Enabled toggle switch
   - Validation with error display
   - Save/Cancel/Delete actions
   - ServerConfigEditorEvent for action handling
   - Unit tests for config conversion and validation
10. [x] Logs viewer par server (`src/ui/mcp/logs.rs`)
   - McpLogsPanel Entity with real-time log display
   - LogLevel enum (Debug, Info, Warning, Error)
   - LogEntry struct with timestamp, server, level, message, context
   - LogFilter with level filtering, server filtering, text search
   - VecDeque ring buffer with configurable max_logs
   - Auto-scroll toggle for following new logs
   - Clear logs action
   - Level-based color coding with theme integration
   - Timestamp formatting and log entry rendering
   - Unit tests for log level parsing and filtering

### 8.3: Tools Integration ✅ COMPLETE
**Fichier:** `src/ui/mcp/tools.rs`

**Tâches:**
1. [x] List tools from connected servers (McpToolsPanel Entity)
2. [x] Tool approval UI (ToolApprovalStatus: Pending/ApprovedSession/ApprovedPermanent/Denied)
3. [x] Pending tool calls display with approve/deny buttons
4. [x] Tool item display with description and approval badge
5. [x] Tool filter by name/description/server
6. [x] PendingToolCall struct with call_id, arguments, timestamp
7. [x] Event emitters for approval actions
8. [x] Tool execution with progress (`src/ui/mcp/progress.rs`)
   - ToolProgressPanel Entity with active execution tracking
   - ExecutionPhase enum (Preparing, Executing, Processing, Completed, Failed, Cancelled)
   - ActiveExecution with elapsed time, progress percentage, status messages
   - Cancel and dismiss actions
   - Progress bar visualization
9. [x] Tool result display in chat (`src/ui/blocks/tool_result_block.rs`)
   - ToolResultBlock Entity for displaying tool results
   - ToolExecutionStatus enum (Success, Error, Pending, Cancelled)
   - Collapsible arguments section
   - JSON-formatted results with syntax highlighting
   - Copy and retry actions
   - Duration and status badges

### 8.4: Resources & Prompts ✅ COMPLETE
**Fichier:** `src/ui/mcp/resources.rs`

**Tâches:**
1. [x] Resource browser UI (McpResourcesPanel Entity)
2. [x] Resource list with name, URI, description
3. [x] Prompt list with arguments display
4. [x] Tab switching between Resources and Prompts
5. [x] Filter by name/description/server
6. [x] Click to read resource (emits ReadResource event)
7. [x] Click to use prompt (emits UsePrompt event)
8. [x] ResourceItem and PromptItem structs
9. [x] Attach resources to context (`src/ui/mcp/context_attach.rs`)
   - McpContextAttachPanel Entity for resource attachment
   - AttachableResource struct with server, URI, name, description, mime type
   - AttachmentStatus enum (Ready, Loading, Attached, Failed)
   - Integration with ContextManager for adding to conversation
   - Attach/detach workflow with visual feedback
   - MCP types in context.rs (McpResource, McpPrompt)
   - MIME type and URI-based language detection
10. [x] Slash command integration (`src/plugins/commands.rs`)
   - /mcp-servers - List connected MCP servers
   - /mcp-tools [server] - List available tools
   - /mcp-resources [server] - List available resources
   - /mcp-prompts [server] - List available prompts
   - /mcp-tool <server> <tool> [args] - Execute MCP tool
   - /mcp-read <server> <uri> - Read MCP resource
   - /mcp-prompt <server> <prompt> [args] - Use MCP prompt
   - Extended CommandResult enum with MCP operation types
   - Updated /help to show MCP commands
   - Unit tests for MCP commands

---

## Phase 9: Cloud & Collaboration ✅ COMPLETE

### 9.1: Conversation Sync ✅ COMPLETE
**Fichiers:** `src/cloud/mod.rs`, `src/cloud/auth.rs`, `src/cloud/storage.rs`, `src/cloud/sync.rs`

**Tâches:**
1. [x] Account authentication (OAuth)
   - CloudAuth with OAuth flow (GitHub, Google)
   - PKCE support for secure auth
   - Token refresh and session persistence
   - OAuthProvider enum with auth/token URLs
2. [x] Encrypted cloud storage
   - CloudStorage client with API integration
   - AES-256-GCM and ChaCha20-Poly1305 encryption
   - Argon2 key derivation from password
   - StorageMetadata with versioning
3. [x] Sync status indicators
   - SyncStatus enum (Idle, Syncing, Offline, Error, Synced)
   - SyncStatusIndicator component
   - SyncStatusPanel with full controls
4. [x] Conflict resolution UI
   - ConflictResolution enum (KeepLocal, KeepRemote, KeepBoth, Manual)
   - SyncConflict struct with local/remote data
   - Conflict detection and resolution workflow
5. [x] Offline-first with sync queue
   - SyncManager with persistent queue
   - SyncOperation enum (Create, Update, Delete)
   - Automatic sync on network change
   - Queue persistence to disk

### 9.2: Sharing ✅ COMPLETE
**Fichiers:** `src/ui/cloud/sharing.rs`

**Tâches:**
1. [x] Generate shareable links
   - ShareDialog Entity with link generation
   - ShareLink struct with URL, permissions, expiry
2. [x] Permission levels (view/comment/edit)
   - SharePermission enum with three levels
   - Permission selector UI with descriptions
3. [x] Expiring links
   - ExpiryOption enum (Never to 1 month)
   - Expiry display in existing links
4. [x] Password protection option
   - Password toggle and input
   - password_protected flag on links

### 9.3: Team Features ✅ COMPLETE
**Fichiers:** `src/cloud/team.rs`, `src/ui/cloud/team.rs`, `src/ui/cloud/activity.rs`, `src/ui/cloud/analytics.rs`

**Tâches:**
1. [x] Team workspaces
   - TeamManager for team operations
   - Team struct with members, projects, settings
   - TeamRole (Owner, Admin, Member, Viewer) with permissions
   - TeamSettings for configuration
2. [x] Shared projects
   - SharedProject struct with permissions
   - ProjectPermission levels (View, Comment, Edit, Admin)
   - Share/unshare project operations
3. [x] Activity feed
   - ActivityEntry with type, target, timestamp
   - ActivityType enum (Created, Updated, Deleted, Shared, Joined, etc.)
   - ActivityTarget enum (Team, Project, Conversation, Member, etc.)
   - ActivityPanel UI with filters and navigation
4. [x] Usage analytics
   - UsageAnalytics with period selection
   - UserUsage and ProjectUsage tracking
   - DailyUsage breakdown
   - AnalyticsPanel UI with overview, users, projects, timeline views
5. [x] Team invitations
   - TeamInvitation with status tracking
   - InvitationStatus (Pending, Accepted, Declined, Expired, Revoked)
   - Accept/decline invitation workflow
6. [x] Team UI
   - TeamPanel with team list, details, members, projects views
   - Create team dialog
   - Invite member dialog
   - Role selector

### 9.4: Cloud UI ✅ COMPLETE
**Fichiers:** `src/ui/cloud/mod.rs`, `src/ui/cloud/auth.rs`, `src/ui/cloud/sync.rs`, `src/ui/cloud/sharing.rs`

**Tâches:**
1. [x] AuthDialog Entity
   - OAuth provider buttons (GitHub, Google)
   - Loading and error states
   - Sign in/Sign up mode toggle
2. [x] SyncStatusIndicator (compact)
   - Status dot with color coding
   - Pending count badge
   - Conflict indicator
3. [x] SyncStatusPanel (expanded)
   - Last sync time display
   - Upload/download stats
   - Auto-sync toggle
   - Sync Now button
   - Conflict warnings
4. [x] ShareDialog Entity
   - Permission selector
   - Password protection
   - Expiry options
   - Existing links management
   - Copy/Revoke actions

---

## Phase 10: Performance & Optimization ✅ MOSTLY COMPLETE

### 10.1: Rendering Optimization 🔄 IN PROGRESS
**Objectif:** Améliorer les performances de rendu GPUI

**Tâches:**
1. [x] Virtual scrolling component (`src/ui/components/virtual_list/mod.rs`)
   - VirtualListState with scroll tracking
   - Visible range calculation with overscan
   - scroll_to_item and scroll_to_bottom methods
2. [x] Lazy loading des code blocks (`src/ui/blocks/lazy_block.rs`)
   - LazyBlock<T> Entity with deferred rendering
   - LazyState enum (Pending, Loading, Loaded, Error)
   - LazyBlockConfig with presets
   - VisibilityObserver for tracking visibility
   - Skeleton placeholder rendering
3. [x] Message pooling (`src/app/pool.rs`)
   - ObjectPool<T> generic pool with factory and reset
   - PooledItem<T> guard for automatic return
   - StringPool and VecPool for buffer reuse
   - PooledMessageData for message view recycling
4. [x] Debounce des mises à jour UI pendant streaming (`src/app/debounce.rs`)
   - Debouncer with interval-based throttling
   - ThrottledCallback for value updates
   - DebouncedChannel for async operations
   - BatchAccumulator for grouping updates
   - UpdateCoalescer for render coalescing
5. [ ] Profiling avec instruments/tracy

### 10.2: Memory Management ✅ COMPLETE
**Objectif:** Réduire l'empreinte mémoire

**Tâches:**
1. [x] Pagination des messages (`src/storage/pagination.rs`)
   - PageInfo, Cursor, PaginationRequest types
   - PaginatedResult with items and page info
   - MessageWindow for virtualized list windowing
   - PaginationState tracker for loading state
   - Cursor-based and offset-based pagination
2. [x] Cleanup des conversations fermées (`src/storage/cleanup.rs`)
   - CleanupConfig with presets (default, aggressive, conservative)
   - CleanupTarget enum (Conversations, Messages, Attachments, Cache, Logs, Temporary)
   - CleanupItem with should_cleanup() logic based on age
   - CleanupJob with dry_run mode and preview()
   - CleanupStats with space_freed tracking
   - CleanupScheduler for automatic cleanup intervals
3. [x] LRU Cache (`src/app/cache.rs`)
   - Generic LruCache<K, V> with entry/size limits
   - TTL support for automatic expiration
   - SyntaxCache type alias for highlighting cache
4. [x] Compression des messages archivés (`src/storage/compression.rs`)
   - CompressionAlgorithm enum (None, Lz4, Zstd, Deflate)
   - CompressionConfig with presets (fast, best, balanced)
   - Compressor with compress/decompress methods
   - CompressedData with checksum verification (CRC32)
   - CompressionStats for monitoring efficiency
5. [ ] Memory leak detection (valgrind/heaptrack)

### 10.3: Startup Performance ✅ COMPLETE
**Objectif:** Démarrage < 500ms

**Tâches:**
1. [x] Lazy initialization module (`src/app/lazy.rs`)
   - StartupMetrics for timing tracking
   - LazyInit<T> for deferred initialization
   - DeferredTaskQueue for post-startup tasks
2. [x] Database connection pooling (`src/storage/pool.rs`)
   - PoolConfig with presets (default, high_concurrency, low_memory)
   - DatabasePool with min/max connections
   - PooledConnectionGuard for RAII-style management
   - PoolStats with hit rate tracking
   - WAL mode and performance pragmas
   - Health check and idle timeout support
   - SharedPool type alias for thread-safe sharing
3. [x] Precompiled tree-sitter queries (`src/syntax/queries.rs`)
   - QueryCache with lazy compilation and caching
   - CompiledQuery with capture name indexing
   - LanguageConfig for language registration
   - Prewarm functionality for startup optimization
   - Language aliases support (rs -> rust, js -> javascript)
   - Separate .scm query files per language
   - QueryCacheStats for monitoring hit rates
4. [x] Async theme loading (`src/app/theme_loader.rs`)
   - ThemeLoader with async discovery and loading
   - ThemeMetadata for lightweight theme listing
   - Theme caching with Arc<Theme>
   - JSON and TOML format support
   - ThemeBuilder for programmatic theme creation
   - Preload functionality for eager loading
   - Background loading with callbacks or oneshot channels
5. [x] Splash screen (`src/ui/components/splash.rs`)
   - SplashScreen Entity with progress display
   - StartupPhase enum for tracking progress
   - Progress bar with phase indicators
   - SplashManager for timing control
   - Minimum display time to prevent flicker

---

## Phase 11: AI Features Enhancement ✅ COMPLETE

### 11.1: Multi-Model Support ✅ COMPLETE
**Objectif:** Support de différents providers AI

**Fichiers:** `src/ai/mod.rs`, `src/ai/provider.rs`, `src/ai/claude.rs`, `src/ai/openai.rs`, `src/ai/ollama.rs`, `src/ai/context.rs`

**Tâches:**
1. [x] Abstraction AIProvider trait (`src/ai/provider.rs`)
2. [x] Claude API direct (sans CLI) (`src/ai/claude.rs`)
3. [x] OpenAI GPT-4 support (`src/ai/openai.rs`)
4. [x] Ollama local models (`src/ai/ollama.rs`)
5. [x] Model selector dans UI (`src/ui/ai/model_selector.rs`)
6. [x] Configuration par provider (API keys, endpoints) (`ProviderConfig`)

### 11.2: Context Management ✅ COMPLETE
**Objectif:** Gestion intelligente du contexte

**Fichiers:** `src/ai/context.rs`, `src/ui/ai/context_panel.rs`, `src/ai/mention.rs`

**Tâches:**
1. [x] Context manager core (`src/ai/context.rs`)
   - ContextItem with File, Snippet, Diff, Web types
   - Token counting and limits
   - Pinned context support
2. [x] Context panel UI (`src/ui/ai/context_panel.rs`)
   - Context items list with token counts
   - Pin/unpin items
   - Add/remove items
   - Token usage indicator
3. [x] File attachment UI - @file syntax (`src/ai/mention.rs`)
   - Mention parser for @file:path, @snippet:name, @url:, @symbol:
   - Line range support (@file:path.rs:10-20)
   - Partial mention detection for autocomplete
   - Highlighted mentions in chat input
   - File attachment badges
   - FilesAttached event on submit
4. [x] Image input support (`src/ui/ai/image_input.rs`)
   - ImageAttachment struct with path, data, mime type
   - Drag & drop zone for images
   - Image preview grid with thumbnails
   - Remove button per image
   - Size validation (max 20MB per image)
   - Supported formats: PNG, JPG, GIF, WebP, SVG
5. [x] Context window indicator (`src/ui/ai/context_indicator.rs`)
   - ContextUsage struct with token tracking
   - Progress bar with color coding (green/warning/critical)
   - Compact view with expandable details
   - Message and file count display
   - Warning messages for high usage
6. [x] Auto-summarization pour conversations longues (`src/ai/summarizer.rs`)
   - SummarizationConfig with presets for small/large contexts
   - ConversationMessage and ConversationSummary structs
   - Summarizer for managing summarization workflow
   - generate_summary_prompt() for AI-based summarization
   - apply_summary() to replace messages with summary
   - extract_topics() for automatic topic detection
   - SummarizationStats for monitoring

### 11.3: Agent Mode ✅ COMPLETE
**Objectif:** Mode agent autonome

**Fichiers:** `src/agent/mod.rs`, `src/agent/task.rs`, `src/agent/planner.rs`, `src/agent/executor.rs`, `src/agent/rollback.rs`

**Tâches:**
1. [x] Task tree structure (`src/agent/task.rs`)
   - TaskStatus enum (Pending, Running, Completed, Failed, etc.)
   - AgentTask with lifecycle methods
   - TaskTree for hierarchical task management
   - TaskNode with expansion state
2. [x] Plan generation (`src/agent/planner.rs`)
   - PlanStep with dependencies and risk levels
   - Plan with critical path calculation
   - AgentPlanner for parsing AI-generated plans
   - Plan validation and JSON extraction
3. [x] Executor (`src/agent/executor.rs`)
   - ExecutorState (Idle, Running, Paused, etc.)
   - ExecutorEvent for progress tracking
   - AgentExecutor with pause/resume/cancel
   - ToolExecutor trait for tool execution
   - Auto-approval for low-risk steps
4. [x] Agent UI (`src/ui/agent/`)
   - TaskPanel with tree view
   - ExecutorView with controls
   - PlanView with step display
5. [x] Integration with Workspace (`src/ui/agent/workspace.rs`)
   - AgentWorkspace Entity for main workspace integration
   - AgentMode enum for state tracking
   - Approval workflow with request/approve/reject
   - Pause/resume/cancel controls
   - Log entries with levels
   - Progress tracking with step counts
6. [x] Rollback capability (`src/agent/rollback.rs`)
   - RollbackOperation enum (FileCreated, FileModified, FileDeleted, etc.)
   - RollbackCheckpoint with operations list
   - RollbackManager for tracking and executing rollbacks
   - Support for file, directory, git, and database operations
   - Custom handler registration for extensibility
   - Step-based checkpoints for plan execution
   - RollbackResult with partial rollback support

### 11.4: Code Intelligence ✅ COMPLETE
**Objectif:** Intégration avec analyse de code

**Fichiers:** `src/lsp/mod.rs`, `src/lsp/protocol.rs`, `src/lsp/client.rs`, `src/lsp/manager.rs`

**Tâches:**
1. [x] LSP protocol types (`src/lsp/protocol.rs`)
   - Position, Range, Location for text positions
   - Diagnostic, DiagnosticSeverity for errors/warnings
   - CompletionItem, CompletionItemKind for autocomplete
   - Hover, HoverContents for hover information
   - SignatureHelp, ParameterInformation
   - DocumentSymbol, SymbolKind for outline
   - CodeAction, WorkspaceEdit for quick fixes
   - ServerCapabilities for feature negotiation
   - JSON-RPC request/response structures
2. [x] LSP client implementation (`src/lsp/client.rs`)
   - LspClient for single server communication
   - LspClientConfig with presets (rust-analyzer, typescript, pyright)
   - JSON-RPC 2.0 message handling
   - Async request/response with pending tracking
   - Server lifecycle (initialize, shutdown)
   - Document sync (didOpen, didClose, didChange)
   - Code intelligence methods (completion, hover, definition, references)
   - LspEvent enum for server notifications
3. [x] Multi-server manager (`src/lsp/manager.rs`)
   - Language enum (Rust, TypeScript, JavaScript, Python, Go, JSON, TOML, Markdown)
   - Language detection from file extension
   - LspManager for managing multiple server connections
   - Automatic server start on document open
   - Document tracking with version management
   - Unified API for all LSP operations
4. [x] LSP UI components (`src/ui/lsp/`)
   - HoverPanel for displaying hover documentation
   - CompletionDropdown for autocomplete suggestions
   - DiagnosticsPanel for errors/warnings display
   - Keyboard navigation (↑/↓/Enter/Esc)
   - Severity filtering (All/Errors/Warnings)
   - Go to location on diagnostic click
   - Quick fix button integration
5. [x] Code actions UI (`src/ui/lsp/code_actions.rs`)
   - CodeActionsPanel Entity
   - CodeActionItem with kind, title, diagnostics
   - CodeActionKind enum (QuickFix, Refactor, Source, etc.)
   - Preferred action highlighting
   - Keyboard navigation (↑/↓/Enter)
   - CodeActionIndicator for inline lightbulb
6. [x] Integration with code blocks (go to definition) (`src/ui/blocks/code_lsp.rs`)
   - CodeLspIntegration for code block analysis
   - CodeToken with position and type information
   - CodeLspConfig for feature toggles
   - handle_click() for Ctrl+Click go-to-definition
   - handle_hover() for hover information
   - find_symbol_occurrences() for symbol highlighting
   - ClickableToken for interactive rendering
7. [x] Integration with file explorer (inline diagnostics) (`src/ui/sidebar/explorer_diagnostics.rs`)
   - DiagnosticCounts for error/warning/info/hint aggregation
   - ExplorerDiagnosticsConfig for display settings
   - ExplorerDiagnosticsStore with caching
   - Parent directory aggregation for nested diagnostics
   - DiagnosticBadge with multiple styles (Dot, Count, IconCount)
   - IconDecoration for file icon overlays
   - FileEntryWithDiagnostics for tree integration
8. [x] Integration with chat input (auto-completion) (`src/ui/chat/input_completion.rs`)
   - ChatCompletionKind enum (File, Folder, Command, Mention, Symbol, etc.)
   - ChatCompletionItem with fuzzy matching
   - CompletionConfig for behavior settings
   - InputCompletionManager for state management
   - Fuzzy matching with simple_fuzzy_match()
   - Command, mention, and file completion sources
   - Keyboard navigation (↑/↓/Tab/Enter/Esc)

---

## Phase 12: Developer Tools Integration ✅ COMPLETE

### 12.1: Terminal Integration ✅ COMPLETE
**Objectif:** Terminal embarqué

**Fichiers:** `src/terminal/mod.rs`, `src/terminal/pty.rs`, `src/terminal/parser.rs`, `src/ui/terminal/`

**Tâches:**
1. [x] PTY support (`src/terminal/pty.rs`)
   - Pty struct with async process management
   - PtyConfig for shell, cwd, env, size, history
   - PtyEvent enum (Output, Exit, Error, TitleChanged, Bell)
   - TerminalKey enum for special key sequences
   - Command history tracking
   - Output buffer with configurable size
2. [x] ANSI rendering (`src/terminal/parser.rs`)
   - AnsiParser with CSI, OSC, SGR parsing
   - AnsiColor enum (16 colors, 256 palette, RGB)
   - TextStyle with bold, italic, underline, colors
   - AnsiEvent enum (Text, Style, CursorMove, Clear, etc.)
   - 256-color and true color support
3. [x] Terminal view UI (`src/ui/terminal/terminal_view.rs`)
   - TerminalView Entity with styled line rendering
   - Input handling with history navigation
   - Scroll and selection support
   - Header with status indicator
4. [x] Command history shared with AI
   - command_history Vec in TerminalView
   - capture_output() for AI context
   - TerminalViewEvent::OutputCaptured
5. [x] Output capture for context
   - recent_output() method
   - all_output() method

### 12.2: File Explorer ✅ COMPLETE
**Objectif:** Navigateur de fichiers intégré

**Fichiers:** `src/ui/explorer/mod.rs`, `src/ui/explorer/file_item.rs`, `src/ui/explorer/tree.rs`

**Tâches:**
1. [x] Tree view avec icônes (`src/ui/explorer/tree.rs`)
   - FileTree Entity with hierarchical tree structure
   - Expand/collapse directories
   - Depth-based indentation
   - File type icons (emojis for languages, folders)
2. [x] File entry types (`src/ui/explorer/file_item.rs`)
   - FileType enum (Directory, File, Symlink, GitIgnored, GitSubmodule)
   - FileEntry struct with tree management
   - Icon detection by extension (50+ file types)
   - Formatted file sizes (B, KB, MB, GB)
3. [x] Git status integration
   - GitStatus enum (Clean, Modified, Added, Deleted, Renamed, Untracked, Ignored, Conflicted)
   - Color coding per status
   - Status character display (M, A, D, R, ?, !, U)
4. [x] Tree navigation
   - Keyboard navigation (↑/↓/Enter/Esc)
   - Click to select
   - Double-click to open
   - Search/filter by filename
5. [x] Context menu actions
   - FileTreeEvent enum (FileSelected, FileOpened, FileAddedToContext, DirectoryExpanded, etc.)
   - Add to context, open file, create, rename, delete events
6. [x] File preview on hover (`src/ui/explorer/preview.rs`)
   - FilePreviewPanel Entity with async file loading
   - PreviewState enum (Empty, Loading, Loaded, Binary, TooLarge, Error)
   - Language detection for 25+ file types
   - Line count and file size display
   - Binary file detection
   - Max preview size limit (1MB)
   - Actions: Open file, Add to context
7. [x] Drag to attach to context (`src/ui/explorer/tree.rs`, `src/ui/chat/input.rs`)
   - DraggedFile struct for single file drag data
   - DraggedFiles struct for multiple file drag data
   - on_drag handler on file tree entries
   - Drag preview showing file icon and name
   - Drop zone on chat input with visual feedback
   - handle_file_drop() and handle_files_drop() methods
   - Automatic @file mention insertion
   - FilesAttached event emission

### 12.3: Diff Viewer Enhanced ✅ COMPLETE
**Objectif:** Revue de code avancée

**Fichiers:** `src/ui/diff/mod.rs`, `src/ui/diff/side_by_side.rs`, `src/ui/diff/hunk.rs`, `src/ui/diff/comments.rs`

**Tâches:**
1. [x] Side-by-side diff view (`src/ui/diff/side_by_side.rs`)
   - SideBySideDiffView Entity
   - DiffDisplayMode enum (SideBySide, Unified)
   - Two-pane layout with old/new sides
   - Line alignment for changes
   - Color-coded additions/deletions
2. [x] Inline comments (`src/ui/diff/comments.rs`)
   - InlineComment struct with author, content, timestamp
   - CommentThread for grouped comments
   - DiffComments manager with location tracking
   - Reply support with threading
   - Resolve/unresolve workflow
   - Export to JSON
3. [x] Apply/reject hunks (`src/ui/diff/hunk.rs`)
   - HunkStatus enum (Pending, Applied, Rejected, Modified, Conflicted)
   - HunkAction enum (Apply, Reject, Reset, Edit, Split, Combine)
   - ManagedHunk with line selection
   - DiffHunkManager for bulk operations
   - Generate patch from selected lines
   - Navigate between hunks
4. [x] Hunk management UI
   - Per-hunk apply/reject buttons
   - Bulk apply all / reject all
   - Reset functionality
   - Status indicators with colors
   - Progress tracking (X/Y hunks reviewed)
5. [ ] Three-way merge view (future enhancement)
6. [ ] Syntax highlighting dans diff (future enhancement)

### 12.4: Debugging Integration ✅ COMPLETE
**Objectif:** Support debugging

**Fichiers:** `src/debug/mod.rs`, `src/debug/protocol.rs`, `src/debug/client.rs`, `src/debug/session.rs`, `src/ui/debug/`

**Tâches:**
1. [x] DAP protocol types (`src/debug/protocol.rs`)
   - DapMessage, DapRequest, DapResponse, DapEvent
   - InitializeArguments, LaunchArguments, AttachArguments
   - Breakpoint, BreakpointLocation, Source
   - StackFrame, Scope, Variable, Thread
   - StoppedEventBody, OutputEventBody, TerminatedEventBody
   - Capabilities and ExceptionBreakpointsFilter
2. [x] DAP client (`src/debug/client.rs`)
   - DapClient for adapter communication
   - DapClientConfig with presets (rust-analyzer, codelldb, debugpy, delve)
   - JSON-RPC message handling over stdio
   - Request/response tracking with async channels
   - Full DAP lifecycle methods
   - Breakpoint management
   - Execution control (continue, step, pause)
   - Variable inspection
3. [x] Debug session management (`src/debug/session.rs`)
   - DebugSession with high-level API
   - DebugState enum (Idle, Initializing, Running, Stopped, etc.)
   - SessionEvent for UI notifications
   - UserBreakpoint with conditions and hit counts
   - Thread and frame tracking
   - Variables caching
4. [x] Breakpoint UI (`src/ui/debug/breakpoints.rs`)
   - BreakpointsList Entity
   - BreakpointItem with file, line, condition
   - Enable/disable toggle
   - Add/remove breakpoints
   - Verified status indicator
5. [x] Variable inspection (`src/ui/debug/variables.rs`)
   - VariablesView Entity
   - VariableItem with expandable children
   - ScopeItem for grouping
   - Tree-based navigation
   - Value display with truncation
6. [x] Stack trace display (`src/ui/debug/call_stack.rs`)
   - CallStackView Entity
   - ThreadItem with frames
   - StackFrameItem with location
   - Frame selection for context switch
7. [x] Debug panel (`src/ui/debug/debug_panel.rs`)
   - DebugPanel Entity with toolbar
   - Start/Stop/Restart controls
   - Continue/Pause/Step controls
   - Tab-based views (Console, Variables, Call Stack, Breakpoints, Watch)
   - Session output display
   - State indicators with colors
8. [x] Watch expressions (`src/ui/debug/watch.rs`)
   - WatchView Entity with expression management
   - WatchExpression with expandable children
   - Add/edit/remove expressions
   - Refresh and evaluate actions
   - Error display for failed evaluations
   - WatchChild for nested values
9. [x] AI-assisted debugging prompts
   - DebugPromptType enum with 8 prompt types
   - DebugContext struct for gathering debug state
   - Context-aware prompt generation
   - AI Help button in debug toolbar
   - Dropdown menu with prompt options
   - Prompt types: AnalyzeError, ExplainState, SuggestBreakpoints, AnalyzeStackTrace, SuggestFix, ExplainVariables, PerformanceAnalysis, MemoryAnalysis
   - Each prompt includes relevant context (console output, variables, location, etc.)
   - AskAI event for integration with chat

---

## Phase 13: Distribution & Packaging ✅ COMPLETE

### 13.1: macOS Distribution ✅ COMPLETE
**Objectif:** App bundle signée

**Tâches:**
1. [x] .app bundle avec Info.plist (`assets/Info.plist`)
2. [ ] Code signing (Developer ID)
3. [ ] Notarization Apple
4. [x] DMG installer (`make dmg`, release workflow)
5. [x] Cargo.toml bundle metadata

**Fichiers créés:**
- `assets/Info.plist` - macOS app bundle metadata
- `Makefile` - Build commands (bundle, dmg, install)
- `Cargo.toml` - [package.metadata.bundle] section

### 13.2: Linux Distribution ✅ COMPLETE
**Objectif:** Packages Linux

**Tâches:**
1. [x] AppImage (`packaging/appimage/`)
   - AppImageBuilder.yml configuration
   - claude-visual.desktop entry
   - build-appimage.sh script
2. [x] Flatpak (`packaging/flatpak/`)
   - com.claude-visual.app.yml manifest
   - Desktop and metainfo files
   - build-flatpak.sh script
3. [x] .deb package (`packaging/debian/`)
   - control, rules, changelog files
   - build-deb.sh script
4. [ ] .rpm package (Fedora/RHEL) - Future
5. [ ] AUR package (Arch) - Future

**Fichiers créés:**
- `packaging/appimage/` - AppImage configuration
- `packaging/flatpak/` - Flatpak manifest and metadata
- `packaging/debian/` - Debian package files
- `scripts/build-*.sh` - Build scripts
- Updated CI/CD workflows for Linux builds

### 13.3: CI/CD Pipeline ✅ COMPLETE
**Objectif:** Build automatisé

**Tâches:**
1. [x] GitHub Actions workflow (`.github/workflows/ci.yml`)
2. [x] Cross-platform builds (macOS x64/arm64, Linux x86_64)
3. [x] Automated tests (cargo test) on macOS and Linux
4. [x] Release automation (`.github/workflows/release.yml`)
5. [x] Changelog generation (from git commits)
6. [x] Linux artifacts (AppImage, .deb) in releases

**Fichiers créés:**
- `.github/workflows/ci.yml` - CI pipeline (check, test, build for macOS + Linux)
- `.github/workflows/release.yml` - Release pipeline (builds, DMG, AppImage, .deb)

### 13.4: Auto-Update System ✅ COMPLETE
**Objectif:** Mises à jour automatiques

**Tâches:**
1. [x] Update checker service (`src/update/checker.rs`)
2. [ ] Delta updates (bsdiff) - Future
3. [x] Update notification UI (`src/ui/update/notification.rs`)
   - UpdateNotification component with banner display
   - UpdateNotificationEvent for user actions
   - Progress display during download/install
   - "Update Now", "Remind Later", "Skip Version" buttons
   - Release notes expansion
   - Error state display
4. [x] Version comparison
5. [ ] Beta channel option - Future

**Fichiers créés:**
- `src/update/mod.rs` - Module exports
- `src/update/checker.rs` - GitHub release checker, version comparison
- `src/update/installer.rs` - DMG mount/install, tar extraction
- `src/ui/update/mod.rs` - UI module exports
- `src/ui/update/notification.rs` - Update notification banner UI

---

## Phase 14: Accessibility & i18n ✅ COMPLETE

### 14.1: Accessibility ✅ COMPLETE
**Objectif:** WCAG 2.1 AA compliance

**Tâches:**
1. [x] Screen reader support (VoiceOver/Orca) (`src/ui/accessibility/announcements.rs`)
   - AnnouncementManager for screen reader notifications
   - AnnouncementPriority (Polite, Assertive)
   - LiveRegion configuration
   - CommonAnnouncements for standard messages
2. [x] High contrast themes (`src/app/theme.rs`)
   - ThemeVariant enum (Dark, Light, HighContrastDark, HighContrastLight)
   - AccessibilitySettings with WCAG compliance options
   - Contrast ratio calculation (WCAG formula)
   - meets_wcag_aa() and meets_wcag_aaa() helpers
   - High contrast dark theme (black bg, white text, yellow focus)
   - High contrast light theme (white bg, black text)
3. [x] Keyboard-only navigation (`src/ui/accessibility/focus.rs`)
   - FocusManager for tracking focus across application
   - FocusableElement with tab index and zone
   - FocusZone enum for logical grouping
   - FocusTrap for modal dialogs
   - Focus ring styles (standard, high contrast, subtle, inset)
4. [x] Focus indicators (`src/ui/accessibility/focus.rs`)
   - FocusRingStyle configuration
   - High contrast yellow focus rings
   - Customizable width, offset, color, radius
   - Keyboard mode detection
5. [x] Reduced motion option (`src/app/theme.rs`)
   - AccessibilitySettings.reduce_motion flag
   - should_reduce_motion() helper
   - high_accessibility() preset
6. [x] Skip links (`src/ui/accessibility/skip_links.rs`)
   - SkipLinkManager for keyboard navigation
   - Default skip links (main, chat, nav, sidebar)
   - Keyboard shortcuts (Alt+1-4)

**Fichiers créés:**
- `src/ui/accessibility/mod.rs` - Module exports
- `src/ui/accessibility/focus.rs` - Focus management
- `src/ui/accessibility/skip_links.rs` - Skip links
- `src/ui/accessibility/announcements.rs` - Screen reader announcements

### 14.2: Internationalization ✅ COMPLETE
**Objectif:** Support multilingue

**Tâches:**
1. [x] Translation system (`src/i18n/translations.rs`)
   - TranslationBundle with message storage
   - I18n manager with locale switching
   - t() and tf() convenience functions
   - Argument substitution ({key} syntax)
   - Fallback to English for missing translations
2. [x] Locale management (`src/i18n/locale.rs`)
   - Locale enum (8 languages)
   - BCP 47 language tag support
   - Native and English names
   - System locale detection
   - Text direction support (LTR/RTL)
3. [x] French translation (fr-FR) - 100+ strings
4. [x] English translation (en-US) - 100+ strings (default)
5. [x] Spanish translation (es-ES) - 100+ strings
6. [x] German translation (de-DE) - 100+ strings
7. [x] Japanese translation (ja-JP) - 100+ strings
8. [x] Portuguese translation (pt-BR) - 100+ strings
9. [x] Chinese translation (zh-CN) - 100+ strings
10. [x] Language selector in settings
    - Auto-detect option with system locale display
    - Language list with flag emojis
    - LanguageSetting enum (Auto/Specific)

**Fichiers créés:**
- `src/i18n/mod.rs` - Module exports
- `src/i18n/locale.rs` - Locale and language management
- `src/i18n/translations.rs` - Translation system

---

## Phase 15: Advanced Plugin System ✅ COMPLETE

### 15.1: Extension API Bindings ✅ COMPLETE
**Objectif:** API extension complète

**Fichiers:** `src/plugins/api.rs`, `src/plugins/host.rs`

**Tâches:**
1. [x] Extension context and API framework
   - ExtensionContext with working directory and project path
   - ApiResult for standardized return values
   - API_VERSION constant for compatibility
2. [x] UI API (`src/plugins/api.rs`)
   - UiApi for notifications and status bar items
   - NotificationLevel enum (Info, Success, Warning, Error)
3. [x] File system operations
   - FileSystemApi with permission-based access
   - read_file(), write_file(), list_directory()
   - Path access checking
4. [x] Key-value storage
   - SettingsApi for extension configuration
   - Per-extension namespaced storage
5. [x] Event system
   - EventApi for subscribing to application events
   - Predefined events (conversation, project, theme, settings)
6. [x] WASM host functions (`src/plugins/host.rs`)
   - show_notification, log, api_version
   - settings_get, settings_set
   - Extension init/cleanup lifecycle

### 15.2: Extension Marketplace UI ✅ COMPLETE
**Objectif:** Store d'extensions

**Fichiers:** `src/ui/extensions/panel.rs`, `src/ui/extensions/mod.rs`

**Tâches:**
1. [x] Extensions panel UI
   - ExtensionsTab enum (Installed, Available, Updates)
   - ExtensionItem with metadata
2. [x] Browse/search UI
   - Search input with filtering
   - Extension list with icons and descriptions
3. [x] Extension management
   - Enable/disable toggle
   - Uninstall button
   - Details panel with features and authors
4. [ ] One-click install from registry - Future
5. [ ] Update notifications - Future
6. [ ] Ratings/reviews - Future

### 15.3: Extension Theme Loading ✅ COMPLETE
**Objectif:** Thèmes chargés depuis extensions

**Fichiers:** `src/plugins/themes.rs`, `src/app/state.rs`, `src/ui/settings/mod.rs`

**Tâches:**
1. [x] Theme loader with extension support
   - load_extension() to load themes from extension directories
   - ThemeMetadata for author and extension tracking
   - list_by_extension() and unload_extension()
2. [x] AppState integration
   - ThemeLoader in AppState
   - load_extension_themes() method
   - list_all_themes() combining built-in and extension themes
3. [x] Theme selector UI update
   - Built-in themes section
   - Extension themes section
   - Author and extension info display
   - Save handler for extension themes

### 15.4: Custom Themes Deep ✅ COMPLETE
**Objectif:** Thèmes avancés

**Fichiers:** `src/ui/extensions/theme_editor.rs`, `src/plugins/icons.rs`

**Tâches:**
1. [x] Theme editor UI (`src/ui/extensions/theme_editor.rs`)
   - EditingColor enum for all theme and syntax colors
   - ThemeEditorTab (Colors, Syntax, Preview, Export)
   - Color picker with hex input
   - Variant selector (Dark/Light)
   - Code preview with sample syntax
2. [x] Live preview
   - Real-time color updates
   - Sample UI elements preview
   - Code block preview
3. [x] Export/share themes
   - JSON export in Zed-compatible format
   - Theme metadata with name and variant
4. [ ] Semantic token support - Future
5. [x] Icon themes (`src/plugins/icons.rs`)
   - IconLoader for loading icon themes from extensions
   - IconTheme with file and folder icon mappings
   - UiIconKind enum for 50+ UI icons
   - FileIcons with extension, filename, and language mappings
   - IconThemeManifest for VS Code-compatible format

---

## Milestones

| Milestone | Target | Status | Description |
|-----------|--------|--------|-------------|
| Compilable MVP | Phase 1 | ✅ | Structure, Claude CLI, basic UI |
| Interactive UI | Phase 2 | ✅ | Input, events, syntax highlighting |
| Enhanced UX | Phase 3 | ✅ | Collapsible blocks, streaming, file picker |
| Git Ready | Phase 4 | ✅ | Repository, worktrees, status, diff |
| Production Ready | Phase 5 | ✅ | Settings, search, export, drag & drop |
| Extensible | Phase 6 | ✅ | WASM host, themes, slash commands |
| Advanced UI | Phase 7 | ✅ | Tabs, vim mode, split views |
| MCP Support | Phase 8 | ✅ | Client, servers UI, tools, resources |
| Cloud/Collab | Phase 9 | ✅ | Sync, sharing, team features |
| Performance | Phase 10 | ✅ | Virtual scroll ✅, lazy loading ✅, pooling ✅, debounce ✅, cleanup ✅, compression ✅, db pool ✅, queries ✅, themes ✅ |
| AI Features | Phase 11 | ✅ | Multi-model ✅, context ✅, agent ✅, LSP ✅ |
| Dev Tools | Phase 12 | ✅ | Terminal ✅, explorer ✅, diff ✅, debugging ✅ |
| Distribution | Phase 13 | ✅ | Packaging ✅, CI/CD ✅, auto-update ✅, notification UI ✅ |
| Accessibility | Phase 14 | ✅ | Screen reader ✅, i18n ✅, high contrast ✅ |
| Plugins++ | Phase 15 | ✅ | Extension API ✅, marketplace UI ✅, theme loading ✅, theme editor ✅, icon themes ✅ |

---

## Prochaines Étapes - Roadmap 2025

### ✅ PHASES 1-9 COMPLETE (Janvier 2025)

| Phase | Status | Highlights |
|-------|--------|------------|
| 1. Foundation | ✅ | GPUI, Claude CLI, basic UI |
| 2. Core Features | ✅ | Input, events, tree-sitter |
| 3. Enhanced UX | ✅ | Collapsible blocks, streaming |
| 4. Git Integration | ✅ | Repository, worktrees, diff |
| 5. Polish | ✅ | Settings, search, export |
| 6. Plugins | ✅ | WASM, themes, commands |
| 7. Advanced UI | ✅ | Tabs, vim, splits |
| 8. MCP | ✅ | Client, servers, tools |
| 9. Cloud | ✅ | OAuth, encrypted sync, offline-first |

---

### 📋 PRIORITÉS RECOMMANDÉES (Ordre suggéré)

#### Sprint 1: Distribution First (Phase 13)
**Pourquoi:** Rendre l'app utilisable par d'autres devs avant d'ajouter des features

1. **CI/CD** (13.3) - GitHub Actions, tests automatisés
2. **macOS Distribution** (13.1) - .app bundle, DMG
3. **Auto-Update** (13.4) - Mises à jour silencieuses

#### Sprint 2: Performance (Phase 10)
**Pourquoi:** Stabiliser avant d'ajouter de la complexité

1. **Virtual Scrolling** (10.1) - Longues conversations
2. **Memory Management** (10.2) - Cleanup, pagination
3. **Startup** (10.3) - < 500ms cold start

#### Sprint 3: AI Features (Phase 11)
**Pourquoi:** Core value proposition

1. **Multi-Model** (11.1) - Claude API direct, OpenAI, Ollama
2. **Context Management** (11.2) - @file, images, pinned
3. **Agent Mode** (11.3) - Task tree, pause/resume

#### Sprint 4: Developer Tools (Phase 12)
**Pourquoi:** Différenciation vs autres clients

1. **Terminal** (12.1) - PTY embarqué
2. **File Explorer** (12.2) - Tree view, git status
3. **Diff Viewer++** (12.3) - Side-by-side, apply hunks

#### Sprint 5: Cloud (Phase 9) ✅ COMPLETE
**Status:** Implémenté

1. **Auth** (9.1) ✅ - OAuth GitHub/Google, PKCE flow
2. **Sync** (9.1) ✅ - AES-256-GCM/ChaCha20 encrypted storage
3. **Offline-first** ✅ - Queue, conflict resolution
3. **Sharing** (9.2) - Links, permissions

#### Sprint 6: Polish (Phases 14-15)
**Pourquoi:** Nice to have, community growth

1. **i18n** (14.2) - Translations
2. **Accessibility** (14.1) - Screen readers
3. **Marketplace** (15.2) - Extension store

---

## Sprint Actuel: Distribution (Phase 13)

**Objectif:** Créer une app distribuable et installable.

### Fichiers à créer:
```
.github/
├── workflows/
│   ├── build.yml        # Build on push
│   ├── test.yml         # Run tests
│   └── release.yml      # Create releases

scripts/
├── package-macos.sh     # Create .app bundle
├── create-dmg.sh        # Create DMG
└── notarize.sh          # Apple notarization

src/update/
├── mod.rs               # Update module
├── checker.rs           # Check for updates
└── installer.rs         # Apply updates
```

### Dépendances à ajouter:
```toml
[dependencies]
self_update = "0.40"     # Auto-update framework
keyring = "2"            # Secure credential storage

[build-dependencies]
cargo-bundle = "0.6"     # macOS .app bundle
```

### Actions immédiates:
1. [ ] Setup GitHub Actions pour builds automatisés
2. [ ] Créer .app bundle avec cargo-bundle
3. [ ] Tester sur macOS clean (sans Rust installé)
4. [ ] Documenter process d'installation

---

## Alternative: Sprint AI Features First

Si les features AI sont prioritaires:

### Fichiers à créer:
```
src/ai/
├── mod.rs               # AI module exports
├── provider.rs          # AIProvider trait
├── claude_api.rs        # Claude API direct
├── openai.rs            # OpenAI GPT-4
├── ollama.rs            # Local models
└── context.rs           # Context management

src/ui/ai/
├── mod.rs               # AI UI module
├── model_selector.rs    # Model picker
├── context_panel.rs     # Context attachments
└── agent_panel.rs       # Agent mode UI
├── sync.rs          # Sync status panel
└── sharing.rs       # Share dialog
```

### Architecture WASM Host:
```rust
// Plugin host with wasmtime
pub struct PluginHost {
    engine: wasmtime::Engine,
    linker: wasmtime::Linker<PluginState>,
    loaded_plugins: HashMap<String, Plugin>,
}

impl PluginHost {
    pub fn new() -> Result<Self> {
        let engine = wasmtime::Engine::new(&wasmtime::Config::new())?;
        let mut linker = wasmtime::Linker::new(&engine);

        // Register host functions
        linker.func_wrap("env", "log", |msg: &str| {
            tracing::info!("[plugin] {}", msg);
        })?;

        Ok(Self { engine, linker, loaded_plugins: HashMap::new() })
    }

    pub fn load_extension(&mut self, path: &Path) -> Result<()> {
        // Parse extension.toml manifest
        // Load WASM module
        // Register plugin
    }
}
```

---

## Notes Techniques

### GPUI Entity Pattern
```rust
// Stateless → Stateful conversion
// Before:
impl RenderOnce for MessageView { ... }

// After:
impl Render for MessageView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement { ... }
}
```

### Settings UI Architecture
```rust
// Settings modal as Entity
pub struct SettingsModal {
    active_tab: SettingsTab,
    pending_changes: HashMap<String, SettingsValue>,
    live_preview: bool,
}

pub enum SettingsTab {
    Appearance,
    Editor,
    Git,
    Claude,
}

impl SettingsModal {
    fn apply_changes(&mut self, cx: &mut Context<Self>) {
        // Write to settings.toml
        // Emit SettingsChanged event
    }
}
```

### FTS5 Search Integration
```rust
impl Database {
    pub fn search_messages(&self, query: &str) -> Result<Vec<SearchResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT m.id, m.conversation_id, m.content,
                    highlight(messages_fts, 0, '<mark>', '</mark>') as highlighted
             FROM messages_fts
             JOIN messages m ON messages_fts.rowid = m.rowid
             WHERE messages_fts MATCH ?1
             ORDER BY rank"
        )?;
        // ...
    }
}
```

### MCP Protocol Basics
```rust
// MCP uses JSON-RPC 2.0 over stdio
pub struct McpClient {
    process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    pending_requests: HashMap<u64, oneshot::Sender<JsonValue>>,
}

impl McpClient {
    pub async fn initialize(&mut self) -> Result<ServerCapabilities> {
        self.send_request("initialize", json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "claude-visual", "version": "0.1.0" }
        })).await
    }

    pub async fn list_tools(&mut self) -> Result<Vec<Tool>> {
        self.send_request("tools/list", json!({})).await
    }
}
```

### Tree-sitter Languages Supported
- Rust, JavaScript, TypeScript, TSX
- Python, JSON, TOML, Bash, Markdown

### Platform Support
- macOS (Metal) - Primary
- Linux (Vulkan) - Planned

---

## Changelog

### 2026-01-26 (Session 19 - Continued x21)
- UI Components Library Expansion - 89 modules total
  - Started with 78 modules, added 11 new component files

- **Countdown Component** (`src/ui/components/countdown.rs`)
  - Timer and clock display components
  - Countdown, Timer, Clock, PomodoroTimer
  - TimeRemaining struct with days/hours/minutes/seconds
  - CountdownSize: Sm, Md, Lg, Xl
  - CountdownVariant: Default, Minimal, Detailed, Circular
  - show_days, show_labels options

- **OTP Input Component** (`src/ui/components/otp_input.rs`)
  - OTP/PIN verification inputs
  - OtpInput, PinInput, VerificationInput, CvvInput
  - OtpSize: Sm, Md, Lg
  - OtpVariant: Default, Underline, Filled
  - OtpState: Default, Valid, Invalid, Loading

- **Sparkline Component** (`src/ui/components/sparkline.rs`)
  - Inline mini charts for data trends
  - Sparkline, TrendIndicator, MiniBarChart, ProgressSparkline, ComparisonSparkline
  - SparklineVariant: Line, Area, Bar, Dots
  - TrendDirection: Up, Down, Flat
  - show_min_max, show_last_value options

- **Floating Action Component** (`src/ui/components/floating_action.rs`)
  - Floating action buttons (FAB)
  - Fab, SpeedDial, FabContainer, MiniFab
  - FabSize: Sm, Md, Lg, Extended
  - FabVariant: Primary, Secondary, Tertiary, Surface
  - FabPosition: BottomRight, BottomLeft, BottomCenter, TopRight, TopLeft
  - SpeedDialDirection: Up, Down, Left, Right

- **Segmented Control Component** (`src/ui/components/segmented_control.rs`)
  - iOS-style segmented controls
  - SegmentedControl, IconSegmentedControl, ButtonGroup, ViewSwitcher
  - SegmentedSize: Xs, Sm, Md, Lg
  - SegmentedVariant: Filled, Outline, Ghost, Pills
  - Segment item with icon, badge, disabled states

- **Mention Component** (`src/ui/components/mention.rs`)
  - @mention input functionality
  - Mention, MentionDropdown, MentionInput, ChannelMention
  - MentionVariant: User, Channel, Team, Document, Link
  - MentionableUser with avatar, status, online state
  - is_self highlight for self-mentions

- **Emoji Picker Component** (`src/ui/components/emoji_picker.rs`)
  - Emoji selection interface
  - EmojiPicker, EmojiButton, ReactionPicker, EmojiReaction
  - EmojiCategory: Recent, Smileys, People, Animals, Food, Travel, Activities, Objects, Symbols, Flags
  - EmojiPickerSize: Sm, Md, Lg
  - Built-in default emoji set
  - show_search, show_preview, show_categories options

- **Sortable List Component** (`src/ui/components/sortable_list.rs`)
  - Drag and drop reorderable lists
  - SortableList, KanbanColumn, SortableGrid, NestedSortableList
  - SortableItem with icon, disabled, locked states
  - SortableVariant: Default, Cards, Compact, Bordered
  - DragState: Idle, Dragging, DragOver
  - show_handle, show_numbers options

- **Audio Player Component** (`src/ui/components/audio_player.rs`)
  - Audio playback controls
  - AudioPlayer, VoiceMessage, AudioRecordButton, PodcastPlayer
  - PlaybackState: Stopped, Playing, Paused, Loading, Error
  - AudioPlayerSize: Sm, Md, Lg
  - AudioPlayerVariant: Default, Minimal, Full, Waveform
  - Volume control, mute, time formatting

- **Video Player Component** (`src/ui/components/video_player.rs`)
  - Video playback controls
  - VideoPlayer, VideoThumbnail, VideoCard, MiniPlayer
  - VideoPlayerSize: Sm, Md, Lg, Full
  - VideoAspectRatio: Widescreen, Standard, Cinematic, Square, Portrait
  - VideoQuality: Auto, 360p-4K options
  - Progress bar with buffered indicator
  - Picture-in-picture mini player

- **Code Editor Component** (`src/ui/components/code_editor.rs`)
  - Code editor with line numbers
  - CodeEditor, LineNumbers, DiffEditor, InlineCode
  - EditorTheme: Dark, Light, Monokai, Dracula, Nord
  - EditorFontSize: Xs, Sm, Md, Lg, Xl
  - EditorLine with modified, error, warning, breakpoint states
  - Selection with cursor position
  - show_line_numbers, show_gutter, show_minimap, word_wrap options
  - Side-by-side diff comparison

### 2026-01-26 (Session 19 - Continued x17)
- UI Components Library Expansion - Advanced Components
  - Created 7 new complex UI components for data display and layout

- **Popover Component** (`src/ui/components/popover.rs`)
  - Floating content container for tooltips and menus
  - PopoverPlacement: Top, Bottom, Left, Right (with Start/End variants)
  - PopoverTrigger: Click, Hover, Manual
  - PopoverEvent: Opened, Closed
  - show_arrow, offset, close_on_outside_click options
  - PopoverContent for simple text popovers
  - MenuPopover for action lists with icons and shortcuts
  - MenuPopoverItem builder with danger, disabled states

- **Accordion Component** (`src/ui/components/accordion.rs`)
  - Collapsible content sections
  - AccordionMode: Single (one open at a time), Multiple
  - AccordionStyle: Default, Separated, Minimal, Flush
  - AccordionItem with title, subtitle, icon, disabled
  - expand_all(), collapse_all() methods
  - AccordionEvent: Expanded(index), Collapsed(index)
  - SimpleAccordion stateless variant
  - FaqAccordion for Q&A format

- **Table Component** (`src/ui/components/table.rs`)
  - Full-featured data table
  - TableColumn with key, header, width constraints, alignment
  - ColumnAlign: Left, Center, Right
  - TableSize: Compact, Default, Comfortable
  - TableStyle: Default, Striped, Bordered, Minimal
  - TableRow with selectable, disabled states
  - TableSort with column and direction
  - TableEvent: RowClick, RowDoubleClick, SelectionChanged, SortChanged
  - Multi-select support
  - DataTable stateless variant for quick rendering
  - KeyValueTable for property display

- **Pagination Component** (`src/ui/components/pagination.rs`)
  - Page navigation for lists/tables
  - PaginationSize: Small, Medium, Large
  - PaginationStyle: Default, Pill, Simple, Outlined
  - visible_pages calculation with ellipsis
  - first(), last(), prev(), next() methods
  - show_first_last, show_prev_next options
  - PaginationEvent: PageChanged, PageSizeChanged
  - SimplePagination stateless variant
  - PageSizeSelector for items per page

- **Tree Component** (`src/ui/components/tree.rs`)
  - Hierarchical data display
  - TreeNode with id, label, icon, children, data
  - TreeStyle: Default, NoLines, Indented
  - expand(), collapse(), toggle() methods
  - expand_all(), collapse_all() for batch operations
  - TreeEvent: Selected, Expanded, Collapsed, DoubleClick
  - Multi-select support
  - FileTree stateless variant for file browsers
  - FileTreeItem with file/dir variants
  - DirectoryListing with size/modified columns

- **NumberInput Component** (`src/ui/components/number_input.rs`)
  - Numeric input with stepper buttons
  - NumberInputSize: Small, Medium, Large
  - min, max, step, precision configuration
  - prefix and suffix support (e.g., "$", "px")
  - increment(), decrement() methods
  - NumberInputEvent: Changed, Focus, Blur
  - SimpleStepper stateless variant
  - QuantitySelector with presets
  - SpinButton compact variant

- **SplitPane Component** (`src/ui/components/split_pane.rs`)
  - Resizable panel layout
  - SplitOrientation: Horizontal, Vertical
  - min_first, min_second size constraints
  - collapse_first(), collapse_second(), expand() methods
  - Draggable divider with handle
  - SplitPaneEvent: Resized, Collapsed, Expanded
  - HorizontalSplit, VerticalSplit stateless variants
  - CollapsibleSidebar with left/right positioning

- **ColorPicker Component** (`src/ui/components/color_picker.rs`)
  - Color selection with visual swatch
  - Preset color palette
  - show_alpha option for transparency
  - ColorPickerEvent: Changed, Opened, Closed
  - Color to hex conversion
  - ColorSwatch stateless variant
  - ColorPalette for displaying multiple colors
  - GradientBar for gradient display

- **Rating Component** (`src/ui/components/rating.rs`)
  - Star rating input/display
  - RatingSize: Small, Medium, Large
  - allow_half for half-star ratings
  - Custom icons (filled, empty, half)
  - RatingEvent: Changed, Hover, HoverEnd
  - readonly and show_value options
  - StarRating stateless variant
  - ThumbsFeedback for up/down voting
  - ReactionPicker for emoji reactions

- **Timeline Component** (`src/ui/components/timeline.rs`)
  - Chronological event display
  - TimelineOrientation: Vertical, Horizontal
  - TimelineItemStatus: Pending, InProgress, Completed, Error
  - TimelineItem with title, description, timestamp, icon
  - Connector lines between items
  - Alternate sides option for vertical layout
  - SimpleTimeline stateless variant
  - ActivityItem and ActivityFeed for social feeds

- **Stepper Component** (`src/ui/components/stepper.rs`)
  - Multi-step process indicator
  - StepperOrientation: Horizontal, Vertical
  - StepStatus: Pending, Active, Completed, Error, Skipped
  - Step with label, description, icon, optional flag
  - next(), prev(), go_to(), reset() navigation
  - complete_current(), error_current() status methods
  - StepperEvent: StepClicked, StepChanged, Completed
  - ProgressSteps stateless variant
  - BreadcrumbSteps for breadcrumb-style navigation

- **Module Registry Update**
  - Updated `src/ui/components/mod.rs` with 38 total modules
  - All components compile successfully
  - Unit tests for core component logic

### 2026-01-26 (Session 19 - Continued x16)
- NEW UI Components Library Expansion
  - Created 10+ new reusable UI components following GPUI patterns

- **Badge Component** (`src/ui/components/badge.rs`)
  - Badge and BadgeWrapper for notification counts and status indicators
  - BadgeVariant: Default, Primary, Success, Warning, Error, Outline
  - BadgeSize: XSmall (dot), Small, Medium, Large
  - with_text(), with_count(), dot() factory methods
  - BadgePosition for wrapper positioning (TopRight, TopLeft, etc.)
  - max_count support with "99+" display

- **Divider Component** (`src/ui/components/divider.rs`)
  - Horizontal and vertical dividers
  - DividerOrientation, DividerStyle (Solid, Dashed, Dotted)
  - DividerThickness (Thin, Normal, Thick)
  - Label support for dividers with centered text
  - HorizontalRule and VerticalRule stateless variants

- **Tag Component** (`src/ui/components/tag.rs`)
  - Tag/Chip for labels and categories
  - TagColor: Default, Primary, Success, Warning, Error, Info, Purple, Pink
  - TagSize: Small, Medium, Large
  - Closable and clickable/selectable modes
  - TagEvent: Clicked, Closed
  - TagGroup for rendering multiple tags
  - TagGroupItem builder pattern

- **Skeleton Component** (`src/ui/components/skeleton.rs`)
  - Loading placeholder components
  - SkeletonShape: Rectangle, Circle, Rounded, Text
  - Skeleton, SkeletonLine (stateless), SkeletonText, SkeletonCard
  - Configurable lines, heights, widths
  - last_line_width for natural text appearance

- **Alert Component** (`src/ui/components/alert.rs`)
  - Notification and warning banners
  - AlertType: Info, Success, Warning, Error
  - AlertStyle: Filled, Outline, Subtle, LeftAccent
  - Dismissible with AlertEvent::Dismissed
  - Action buttons with AlertEvent::ActionClicked
  - InlineAlert stateless variant
  - Custom icons support

- **Card Component** (`src/ui/components/card.rs`)
  - Content containers
  - CardVariant: Default, Elevated, Outlined, Ghost, Interactive
  - CardPadding: None, Small, Medium, Large
  - Header with title, subtitle, icon
  - Footer support
  - Selected and disabled states
  - CollapsibleCard with toggle
  - SimpleCard stateless variant

- **Dropdown Component** (`src/ui/components/dropdown.rs`)
  - Selection menus
  - DropdownSize: Small, Medium, Large
  - DropdownOption with label, description, icon, disabled
  - Searchable mode
  - DropdownEvent: Changed, Opened, Closed
  - Error state display
  - OptionList for inline radio-style selection

- **Tabs Component** (`src/ui/components/tabs.rs`)
  - Tabbed interfaces
  - TabsStyle: Underline, Pill, Boxed, Minimal
  - TabsSize: Small, Medium, Large
  - TabItem with icon, badge, disabled, closable
  - TabsEvent: Changed, CloseRequested
  - Full-width mode
  - TabBar stateless variant

- **Switch Component** (`src/ui/components/switch.rs`)
  - Toggle switches, checkboxes, radio buttons
  - SwitchSize: Small, Medium, Large
  - Switch with label and description
  - Checkbox with indeterminate state
  - RadioButton and RadioGroup
  - RadioGroupOption builder
  - All emit Changed events

- Module Registration
  - Added all new modules to `src/ui/components/mod.rs`
  - All components compile successfully

### 2026-01-26 (Session 19 - Continued x15)
- StatusBar Component COMPLETE
  - Created `src/ui/components/status_bar.rs`
  - StatusBar struct with workspace state display
  - Shows: project name, message count, streaming indicator, filter, vim mode, word wrap, line numbers, font size, theme
  - StatusItem enum for flexible item types (Text, Separator, IconText, Clickable)
  - StatusBarConfig for left/center/right sections
  - Setter methods: set_project(), set_message_count(), set_streaming(), set_vim_mode(), set_filter(), set_word_wrap(), set_line_numbers()
- StatusBar Integration in Workspace
  - Added StatusBar Entity to Workspace struct
  - Initialized in Workspace::new()
  - Added to render function (hidden in focus mode)
  - Added update_status_bar() helper method
  - Status bar updates on:
    - Project selection
    - Streaming start/end
    - Tab switching
    - New tab/conversation creation
    - Vim mode toggle
    - Word wrap toggle
    - Line numbers toggle
    - Filter cycle
- ChatView Enhancement
  - Added messages_len() method for status bar message count

### 2026-01-26 (Session 19 - Continued x12)
- Phase 3.5 (ChatView) - Message Navigation Indicator COMPLETE
  - Navigation indicator displays when a message is selected
  - Shows current position (e.g., "3/12") in toolbar
  - Role label for selected message (You/Claude/Tool/Result)
  - First/Last jump buttons (⏮/⏭)
  - Previous/Next navigation buttons (◀/▶)
  - Button states reflect navigation boundaries
  - Clear selection button (✕)
  - Navigation hint when no selection ("Navigate: ⌥↑↓ · Jump: ⌥Home/End")
  - Hint only shows when multiple messages exist
- Phase 3.5 (ChatView) - Copy Selected Message COMPLETE
  - CopySelectedMessage action added to main.rs
  - Alt+C keyboard shortcut
  - copy_selected_message() method in ChatView
  - Copies message with role prefix (e.g., "**You:**\n...")
  - has_selected_message() and selected_message_position() helpers
  - Handler in Workspace (handle_copy_selected_message)
  - Copy button with shortcut hint (⌥C) in navigation indicator
- Phase 3.5 (ChatView) - Bookmark Selected Message COMPLETE
  - BookmarkSelectedMessage action added to main.rs
  - Alt+B keyboard shortcut
  - bookmark_selected_message() method in ChatView
  - is_selected_bookmarked() helper method
  - Handler in Workspace (handle_bookmark_selected_message)
  - Bookmark toggle button (★/☆) in navigation indicator
  - Shows current bookmark state
  - Shortcut hint (⌥B) displayed
  - Golden color when bookmarked
- Phase 3.5 (ChatView) - Bookmarked Messages Filter COMPLETE
  - show_bookmarked_only field in ChatView
  - toggle_bookmarked_filter() method
  - is_bookmarked_filter_active() accessor
  - bookmarked_message_count() method
  - Updated visible_message_count() to respect bookmark filter
  - Updated filtered_message_views() for bookmark filtering
  - Bookmarked filter toggle in toolbar (★ icon)
  - Shows count of bookmarked messages
  - Golden/warning color when active
  - Combines with role filters (AND logic)
  - Filter count display updated for bookmark filtering
- Phase 3.5 (ChatView) - Search Result Navigation COMPLETE
  - NextSearchResult and PrevSearchResult actions
  - Cmd+G: Next search result
  - Cmd+Shift+G: Previous search result
  - Handlers in Workspace (handle_next_search_result, handle_prev_search_result)
  - Keyboard hints in search bar (⌘G, ⇧⌘G)
  - Updated search bar navigation buttons with shortcut hints
- Phase 3.5 (ChatView) - Stats Bar Toggle COMPLETE
  - ToggleStats action added to main.rs
  - Cmd+I keyboard shortcut
  - Handler in Workspace (handle_toggle_stats)
  - "Stats" toggle button in toolbar
  - Active state highlighting when stats bar visible
  - Keyboard hint (⌘I) displayed

### 2026-01-26 (Session 19 - Continued x11)
- Phase 3.2 (ChatView) - Conversation Title COMPLETE
  - conversation_title and editing_title fields in ChatView
  - title_edit_buffer for temporary storage during editing
  - title_focus FocusHandle for keyboard capture
  - display_title() with auto-generation from first user message
  - start_editing_title() with focus and buffer initialization
  - save_edited_title() and cancel_editing_title() methods
  - handle_title_key_down() for Enter/Escape/Backspace
  - handle_title_input() for text input capture
  - Title display in messages toolbar with edit icon (✎)
  - Click to edit with inline input field
  - Keyboard hints in edit mode (Enter to save · Esc to cancel)
  - Save/Cancel buttons for mouse users
  - Visual cursor indicator (_) in edit mode
  - Custom vs auto-generated title styling
  - Title reset on clear_conversation()
- Phase 3.2 (ChatView) - Export Enhancement COMPLETE
  - Updated export_to_markdown() to use conversation title
  - Added Export button to messages toolbar
  - Request export via request_export() method
- Phase 3.2 (ChatView) - Auto-Scroll Toggle COMPLETE
  - auto_scroll field in ChatView (default true)
  - toggle_auto_scroll() and set_auto_scroll() methods
  - is_auto_scroll() accessor
  - "Auto↓" toggle button in toolbar
  - Active state highlighting when enabled
- Phase 3.5 (Keyboard Shortcuts) - Chat Actions COMPLETE
  - New actions: ToggleChatSearch, ExportConversation, CopyConversation, ClearConversation, CollapseAllMessages, ExpandAllMessages
  - Keybindings:
    - Cmd+F: Toggle chat search
    - Cmd+E: Export conversation
    - Cmd+Shift+C: Copy conversation
    - Cmd+Shift+Backspace: Clear conversation
    - Cmd+[: Collapse all messages
    - Cmd+]: Expand all messages
  - Handler methods in Workspace:
    - handle_toggle_chat_search()
    - handle_export_conversation()
    - handle_copy_conversation()
    - handle_clear_conversation()
    - handle_collapse_all()
    - handle_expand_all()
  - Actions registered on Workspace render
  - Global action handlers in main.rs
- Phase 3.5 (UX) - Keyboard Shortcut Hints COMPLETE
  - Added shortcut hints to toolbar buttons:
    - Collapse: ⌘[
    - Expand: ⌘]
    - Copy: ⇧⌘C
    - Export: ⌘E
    - Search: ⌘F
  - Muted style for shortcut text (50% opacity)
  - Shortened button labels (Collapse All → Collapse)
- Phase 3.2 (ChatView) - Filter Counts COMPLETE
  - message_count_for_filter() method
  - Pre-computed filter counts in toolbar render
  - Count display next to filter chip labels (e.g., "You 5", "Claude 3")
  - Counts hidden for "All" filter
  - Counts hidden when zero
  - Styled with reduced opacity for subtlety

### 2026-01-26 (Session 19 - Continued x10)
- Phase 3.4 (Enhanced Code Blocks) - Line Highlighting COMPLETE
  - HighlightedRange struct with start_line, end_line, style, label
  - HighlightStyle enum (Reference, Error, Warning, Success, Info, Emphasis)
  - with_highlights() constructor for pre-configured highlights
  - Methods: add_highlight(), highlight_line(), highlight_range(), clear_highlights()
  - get_line_highlight() for checking if line has highlight
  - highlight_bg_color() and highlight_border_color() per style
  - Highlight gutter indicator column in render
  - Color-coded backgrounds based on style
  - Unit tests for highlight management
- Phase 3.1 (MessageView) - Context Menu Actions COMPLETE
  - MessageAction enum (Copy, CopyAsMarkdown, Regenerate, Edit, Quote, RetryFromHere, Delete)
  - Role-based action availability (Edit for User, Regenerate for Assistant)
  - Keyboard shortcut hints (⌘C for Copy, ⌫ for Delete)
  - MessageViewEvent extended with CopyAsMarkdown, RegenerateResponse, Edit, Quote, RetryFromHere
  - show_context_menu() and hide_context_menu() methods
  - execute_action() for action handling
  - copy_as_markdown() with role prefix formatting
  - More actions button (•••) in message footer
  - Context menu with visual separator before Delete
  - Destructive action styling (red for Delete)
  - Unit tests for action properties
- Phase 5.4 (ChatView) - Conversation Statistics COMPLETE
  - ConversationStats struct with comprehensive metrics
  - Message counts (total, user, assistant, tool)
  - Word counts (total, user, assistant)
  - Character count
  - Estimated token count (words * 1.3 approximation)
  - Conversation duration calculation
  - Formatting helpers: format_duration(), format_tokens(), format_words()
  - K/M suffixes for large numbers
  - Stats bar UI showing all metrics
  - Conditional display (hidden when empty)
  - toggle_stats() and set_show_stats() methods
  - Unit tests for formatting functions
- Phase 3.1 (MessageView) - Relative Timestamps COMPLETE
  - format_relative_time() function for human-readable timestamps
  - "just now" for < 1 minute
  - "X min ago" for < 1 hour
  - "X hours ago" for < 24 hours
  - "yesterday" or "X days ago" for < 7 days
  - Month/day format for older in same year
  - Full date format for different year
  - full_timestamp() method for absolute time
  - Automatic timezone conversion to local
  - Unit tests for all time ranges
- Phase 5.3 (ChatView) - In-Conversation Search COMPLETE
  - ConversationSearchResult struct with message location details
  - Snippet generation with context (30 chars before/after)
  - Case-insensitive searching
  - Multi-match support per line
  - Search bar UI with input, results counter, navigation
  - toggle_search(), set_search_query() methods
  - next_search_result(), prev_search_result() navigation
  - current_result(), search_result_count(), current_result_index()
  - render_search_bar() with close button
  - Unit tests for search results
- Phase 3.1 (MessageView) - Message Reactions COMPLETE
  - MessageReaction enum (ThumbsUp, ThumbsDown)
  - emoji() and label() methods for reaction display
  - Reacted event in MessageViewEvent
  - set_reaction() with toggle support
  - can_react() check (only for assistant messages)
  - Reaction buttons in message footer
  - Visual feedback with colored backgrounds
  - Unit tests for reaction properties
- Phase 3.2 (ChatView) - Scroll to Bottom Button COMPLETE
  - show_scroll_to_bottom state tracking
  - unread_count for messages while scrolled away
  - on_scroll_away() to show button
  - scroll_to_bottom() to hide and reset
  - increment_unread() for new message notification
  - Floating button with arrow icon
  - Unread badge with count (9+ for many)
  - Conditional display (only when 3+ messages)
- Phase 3.1 (MessageView) - Message Bookmarks COMPLETE
  - bookmarked: bool and bookmark_note: Option<String> fields
  - BookmarkToggled event in MessageViewEvent
  - toggle_bookmark() and set_bookmark() methods
  - is_bookmarked() and bookmark_note() accessors
  - Bookmark action (MessageAction::Bookmark) in context menu
  - ⌘B keyboard shortcut for toggle
  - Bookmark indicator (★) in message header
  - Bookmark toggle button (☆/★) in message footer
  - Visual feedback with warning color for bookmarked state
  - Unit tests for bookmark action properties
- Phase 3.2 (ChatView) - Collapse/Expand All Messages COMPLETE
  - set_collapsed() and is_collapsed() methods on MessageView
  - collapse_all() and expand_all() methods on ChatView
  - are_all_collapsed() and are_all_expanded() checks
  - Messages toolbar with view controls
  - Collapse All button (▶)
  - Expand All button (▼)
  - Search toggle button
  - Active state highlighting on toolbar buttons
- Phase 3.2 (ChatView) - Message Filtering by Role COMPLETE
  - MessageFilter enum (All, UserOnly, AssistantOnly, ToolsOnly)
  - label() and includes_role() methods for filter display and logic
  - all_options() for getting all available filters
  - set_message_filter() and next_filter() for changing filters
  - filtered_message_views() for getting filtered messages
  - visible_message_count() for counting visible messages
  - Filter chips in messages toolbar with active highlighting
  - Filter count display (X/Y) when filtered
  - Unit tests for filter labels, includes_role, and all_options
  - MessageFilter exported from chat module
- Phase 3.2 (ChatView) - Copy Conversation COMPLETE
  - copy_conversation_to_clipboard() for plain text copy
  - copy_conversation_as_markdown() for formatted copy
  - "Copy All" button in messages toolbar
  - Role labels ([You], [Claude], [Tool], etc.) in plain text format
  - Tool name display for tool messages
- Phase 3.2 (ChatView) - Clear Conversation COMPLETE
  - clear_conversation() method resets all state
  - clear_messages() method for keeping conversation ID
  - Stops streaming if in progress
  - Clears messages, search, scroll, and filter states
  - "Clear" button in messages toolbar
  - Hover effect with error color warning
  - Logging for cleared events
- Phase 3.2 (ChatInput) - Character and Word Count COMPLETE
  - Character count display in input footer
  - Word count display in input footer
  - Estimated token count (~1.3 tokens/word)
  - Keyboard shortcut hints (Enter to send, Shift+Enter for newline)
  - Conditional display (only when text present)
  - Left side hints, right side counts
- Phase 3.2 (ChatView) - View Options COMPLETE
  - show_timestamps toggle for timestamp visibility
  - compact_mode toggle for reduced padding
  - toggle_timestamps() and set_show_timestamps() methods
  - toggle_compact_mode() and set_compact_mode() methods
  - timestamps_visible() and is_compact_mode() accessors
  - "Time" toggle button in toolbar
  - "Compact" toggle button in toolbar
  - Separator before view options
  - Active state highlighting on toggle buttons

### 2026-01-26 (Session 19 - Continued x9)
- Phase 5.3 (Conversation Search) - Filtres par date/projet COMPLETE
  - Added DateRangeFilter enum (AllTime, Today, LastWeek, LastMonth, LastQuarter, LastYear)
  - Added SearchFilter struct with date_range and project_id
  - Implemented search_messages_with_filter() in database.rs
  - Added filter panel toggle in history sidebar
  - Date range chips with visual selection
  - Project filter dropdown with all projects
  - Clear filters button when filters active
  - Automatic re-search on filter change
- Phase 8.2.10 (MCP Integration) - Logs viewer exports COMPLETE
  - Added exports for McpLogsPanel, LogEntry, LogFilter, LogLevel
- Phase 12.2.9 (Developer Tools) - AI-assisted debugging prompts COMPLETE
  - DebugPromptType enum with 8 prompt types
  - DebugContext struct for gathering debug state
  - Context-aware prompt generation for each prompt type
  - AI Help button in debug toolbar with dropdown menu
  - Prompt types: AnalyzeError, ExplainState, SuggestBreakpoints, AnalyzeStackTrace, SuggestFix, ExplainVariables, PerformanceAnalysis, MemoryAnalysis
  - AskAI event for integration with chat
- Phase 3.4 (Code Block) - Diff view mode COMPLETE
  - CodeDisplayMode enum (Normal, Diff)
  - LineChangeType enum (Context, Added, Removed, ModifiedOld, ModifiedNew)
  - DiffLine struct with content, change_type, line numbers
  - compute_diff() method for generating diff lines
  - with_diff() constructor for diff code blocks
  - toggle_display_mode() for switching views
  - Diff button in header with +/- stats
  - Two-column line numbers (old/new)
  - Prefix column with +/-/space
  - Color-coded backgrounds (green/red/neutral)
  - Syntax highlighting in diff view
  - Unit tests for diff computation

### 2025-01-26 (Session 19 - Continued x8)
- Phase 8.4 (MCP Integration) - Attach Resources to Context COMPLETE
  - Created `src/ui/mcp/context_attach.rs`:
    - McpContextAttachPanel Entity for MCP resource attachment
    - AttachableResource struct with server, URI, name, description, mime type, size
    - AttachmentStatus enum (Ready, Loading, Attached, Failed)
    - McpContextAttachEvent enum for read/attach/detach workflow
    - Panel header with title, resource count, close button
    - Resource list with status indicators
    - Attach/detach buttons with loading states
    - Integration with ContextManager for adding to conversation
    - Error display for failed attachments
    - Unit tests for attachment status and resource creation
  - Updated `src/ai/context.rs`:
    - Added ContextItemType::McpResource for MCP resources
    - Added ContextItemType::McpPrompt for MCP prompts
    - Added ContextItem::mcp_resource() constructor method
    - Added ContextItem::mcp_prompt() constructor method
    - Added mime_to_language() helper for MIME type mapping
    - Added uri_to_language() helper for URI extension detection
    - Updated format_for_prompt() for MCP content types
  - Updated `src/ui/mcp/mod.rs` to export context_attach module
- Phase 8.4 (MCP Integration) - Slash Command Integration COMPLETE
  - Updated `src/plugins/commands.rs`:
    - Extended CommandResult enum with MCP operation types:
      - McpToolCall { server, tool, arguments } for tool execution
      - McpResourceRead { server, uri } for resource reading
      - McpPromptGet { server, prompt, arguments } for prompts
      - McpListServers, McpListTools, McpListResources, McpListPrompts
    - Added /mcp-servers command to list connected servers
    - Added /mcp-tools [server] command to list available tools
    - Added /mcp-resources [server] command to list resources
    - Added /mcp-prompts [server] command to list prompts
    - Added /mcp-tool <server> <tool> [args] command for execution
    - Added /mcp-read <server> <uri> command for resource reading
    - Added /mcp-prompt <server> <prompt> [args] command
    - Updated /help command to show MCP commands section
    - Added unit tests for all MCP commands
- Phase 12.2 (File Explorer) - Drag to Attach to Context COMPLETE
  - Updated `src/ui/explorer/tree.rs`:
    - Added DraggedFile struct for single file drag data
    - Added DraggedFiles struct for multiple file drag data
    - Added on_drag handler to file tree entries
    - Drag preview showing file icon and name with shadow
    - Added FileTreeEvent::FileDragStarted and FilesDragStarted variants
  - Updated `src/ui/explorer/mod.rs` to export DraggedFile and DraggedFiles
  - Updated `src/ui/chat/input.rs`:
    - Added is_drag_over field to track drag state
    - Added handle_file_drop() method for single file
    - Added handle_files_drop() method for multiple files
    - Added drag_over visual feedback on chat input
    - Added on_drop handler to receive dropped files
    - Automatic @file mention insertion on drop
    - FilesAttached event emission
- Phase 8.2 (MCP Integration) - Server Configuration Editor COMPLETE
  - Created `src/ui/mcp/server_config.rs`:
    - ServerConfigEditor Entity for editing server configuration
    - EditingServerConfig struct with conversion methods
    - EditingField enum for tracking focused field
    - Text fields: name, command, description
    - Multiline text areas: args, env, auto_approve
    - Enabled toggle switch with visual feedback
    - Validation with error display
    - Save/Cancel/Delete actions
    - ServerConfigEditorEvent for action handling
    - Render methods for text fields and toggle
    - Unit tests for config conversion and validation
  - Updated `src/ui/mcp/mod.rs` to export server_config module
- Updated `docs/ROADMAP.md`:
  - Marked Attach resources to context as complete
  - Marked Slash command integration as complete
  - Marked Drag to attach to context as complete
  - Marked Server configuration editor as complete

### 2025-01-26 (Session 19 - Continued x7)
- Phase 9.3 (Cloud & Collaboration) - Team UI Integration COMPLETE
  - Updated `src/ui/cloud/team.rs`:
    - Made `set_view_mode` public for command palette integration
    - Made `open_create_dialog` public for external access
    - Made `open_invite_dialog` public for external access
  - Updated `src/ui/workspace.rs`:
    - Fixed team_create command to open create dialog
    - Fixed team_invite command to open invite dialog
    - Added team_activity and team_analytics commands
- Phase 8.3 (MCP Integration) - Tool Execution & Display COMPLETE
  - Created `src/ui/blocks/tool_result_block.rs`:
    - ToolResultBlock Entity for displaying tool results in chat
    - ToolExecutionStatus enum (Success, Error, Pending, Cancelled)
    - ToolResult with tool name, server, arguments, content, error
    - Collapsible arguments section
    - JSON-formatted results with proper indentation
    - Duration display (ms/s formatting)
    - Status badges with icons and colors
    - Copy result to clipboard action
    - Retry action for failed tools
    - Unit tests for result types and status
  - Created `src/ui/mcp/progress.rs`:
    - ToolProgressPanel Entity for active execution tracking
    - ExecutionPhase enum (Preparing, Executing, Processing, Completed, Failed, Cancelled)
    - ActiveExecution with elapsed time, progress, status messages
    - Start/update/complete/fail execution methods
    - Progress bar visualization
    - Cancel and dismiss actions
    - Clear all completed button
    - Collapsible panel header
    - Unit tests for execution phases
  - Updated `src/ui/blocks/mod.rs` to export tool_result_block
  - Updated `src/ui/mcp/mod.rs` to export progress module
- Phase 12.2 (File Explorer) - File Preview COMPLETE
  - Created `src/ui/explorer/preview.rs`:
    - FilePreviewPanel Entity with async file loading
    - PreviewState enum (Empty, Loading, Loaded, Binary, TooLarge, Error)
    - Async file content loading via spawn
    - Binary file detection (null bytes, non-text ratio)
    - Language detection for 25+ file types
    - Line count and file size display
    - Max preview size limit (1MB)
    - Max preview lines (50)
    - Header with filename, language badge, size, close button
    - Stats row with line count and size
    - Content area with monospace font
    - Footer with Open and Add to Context actions
    - Proper error states (binary, too large, load error)
    - Unit tests for binary detection and language detection
  - Updated `src/ui/explorer/mod.rs` to export preview module
- Updated `docs/ROADMAP.md`:
  - Marked Tool execution with progress as complete
  - Marked Tool result display in chat as complete
  - Marked File preview on hover as complete

### 2025-01-26 (Session 19 - Continued x6)
- Phase 9.3 (Cloud & Collaboration) - Team Features COMPLETE
  - Created `src/cloud/team.rs`:
    - TeamManager for team CRUD operations
    - Team struct with members, projects, settings
    - TeamMember with role, email, join date
    - TeamRole enum (Owner, Admin, Member, Viewer) with permissions
    - TeamSettings for configuration (invites, projects, notifications)
    - SharedProject with permissions (View, Comment, Edit, Admin)
    - ActivityEntry with type, target, timestamp
    - ActivityType enum (Created, Updated, Deleted, Shared, Joined, etc.)
    - ActivityTarget enum (Team, Project, Conversation, Member, etc.)
    - UsageAnalytics with period selection and breakdowns
    - UserUsage and ProjectUsage tracking
    - DailyUsage for timeline data
    - TeamInvitation with status tracking
    - Unit tests for team operations
  - Created `src/ui/cloud/team.rs`:
    - TeamPanel Entity for team workspace UI
    - TeamViewMode (TeamList, TeamDetails, Members, Projects)
    - Team list with create button
    - Team details with member/project tabs
    - Create team dialog
    - Invite member dialog with role selector
    - Accept/decline invitation UI
    - TeamPanelEvent for all actions
  - Created `src/ui/cloud/activity.rs`:
    - ActivityPanel Entity for activity feed
    - Filter by activity type
    - Relative time formatting
    - Activity entry rendering with icons and colors
    - Navigate to target on click
    - Empty/loading states
  - Created `src/ui/cloud/analytics.rs`:
    - AnalyticsPanel Entity for usage dashboard
    - AnalyticsViewMode (Overview, Users, Projects, Timeline)
    - Period selector (Week, Month, Quarter, Year)
    - Stat cards for conversations, messages, tokens, users
    - Users table with usage breakdown
    - Projects table with contributors
    - Daily timeline breakdown
    - Chart placeholder for future visualization
  - Updated `src/cloud/mod.rs` to export team module
  - Updated `src/ui/cloud/mod.rs` to export team, activity, analytics
  - Updated `src/ui/workspace.rs`:
    - Added Team tab to SidebarTab enum
    - Added team_panel field (Entity<TeamPanel>)
    - Added TeamPanel import from cloud::team
    - Initialized team_panel in Workspace::new()
    - Subscribed to TeamPanelEvent for team actions
    - Added Team tab button in sidebar tabs
    - Added TeamPanel rendering in sidebar content

### 2025-01-26 (Session 19 - Continued x5)
- Phase 6.2 (Themes) - Per-Project Theme Override COMPLETE
  - Updated `src/project/manager.rs`:
    - Added theme_override: Option<ThemeVariant> to Project struct
    - Updated Project::new() to initialize theme_override as None
    - Added set_theme_override() method to ProjectManager
    - Added get_theme_override() method to ProjectManager
    - Imported ThemeVariant from crate::app::theme
  - Updated `src/ui/sidebar/projects.rs`:
    - Modified ProjectsSidebarEvent::ProjectSelected to include project ID
    - Updated select_project() to emit both ID and path
  - Updated `src/ui/workspace.rs`:
    - Updated ProjectSelected handler to check for theme override
    - Applies project theme variant when switching projects
    - Logs theme override application for debugging
- Phase 7.3 (Split Views) - Integration with Workspace ChatViews COMPLETE
  - Updated `src/ui/workspace.rs`:
    - Added split_container field (Option<Entity<SplitContainer>>)
    - Added split_mode flag and split_pane_views HashMap
    - Added toggle_split_mode() to enter/exit split view mode
    - Added split_horizontal() and split_vertical() methods
    - Added close_split_pane() with auto-exit when last pane closes
    - Added import for SplitContainer, SplitContainerEvent, SplitDirection
    - Subscribed to SplitContainerEvent for pane focus/close/split events
    - Updated render() with split mode header showing:
      - "Split View" label with pane count
      - Split horizontal (─) and vertical (│) buttons
      - Close pane and Exit Split buttons
    - Conditional rendering: TabBar hidden in split mode, SplitContainer shown instead
- Phase 7.3 (Split Views) - Resize Drag Handler COMPLETE
  - Updated `src/ui/split/mod.rs`:
    - Added start_resize_drag() method to initiate drag state
    - Added update_resize_drag() method with delta calculation and weight normalization
    - Added end_resize_drag() method to clear drag state
    - Added get_weights_at_divider() and set_weights_at_divider() helpers
    - Added get_pane_weight() and set_node_weight() recursive helpers
    - Updated render_node() with:
      - Pane flex_grow(FlexGrow(weight)) for proportional sizing
      - Divider mouse event handlers (mouse_down, mouse_up, mouse_move)
      - Visual feedback: divider highlights when dragging
      - Pane shows weight percentage in placeholder text
    - Minimum pane weight constraint (0.2) prevents collapsing
- Phase 7.2 (Vim Mode) - Integration with ChatInput COMPLETE
  - Updated `src/ui/chat/input.rs`:
    - Added vim_state field (Option<Entity<VimState>>)
    - Added vim_mode() method to check current vim mode
    - Added toggle_vim_mode() method
    - Modified handle_key_down() to route keys through VimState when vim enabled
    - Added apply_vim_action() method supporting:
      - Mode transitions (i, a, I, A, o, O)
      - Cursor movement (h, j, k, l, w, b, e, 0, ^, $, gg, G)
      - Text manipulation (x, X, dd, delete)
    - Added cursor manipulation methods:
      - move_cursor_left(), move_cursor_right()
      - insert_char(), delete_char_at_cursor(), delete_char_before(), delete_range()
      - find_line_start(), find_line_end(), find_first_non_blank()
      - find_next_word_start(), find_prev_word_start(), find_word_end()
    - Modified handle_input() to only accept text in Insert mode
    - Added vim mode indicator in render():
      - Shows mode badge (NORMAL/INSERT/VISUAL etc.) with color coding
      - Shows help hint ("Press 'i' to edit", "Esc to return")
    - Vim mode auto-enabled if settings.vim_mode is true

### 2025-01-26 (Session 19 - Continued x4)
- Phase 7.3 (UX Enhancements) - Configurable Keymaps COMPLETE
  - Updated `src/app/settings.rs`:
    - Added Keybindings struct with 10 configurable shortcuts
    - Default keybindings: toggle_sidebar, new_conversation, open_settings, command_palette, next_tab, prev_tab, close_tab, new_tab, send_message, focus_input
    - Added all_bindings() method returning action-keybinding pairs
    - Added set_binding() method for updating individual keybindings
  - Updated `src/ui/settings/mod.rs`:
    - Added Keybindings tab to SettingsTab enum
    - Added render_keybindings_tab() with visual keybinding list
    - Added format_keybinding() for symbol display (cmd→⌘, ctrl→⌃, etc.)
    - Added start_keybinding_edit() placeholder for edit functionality
    - Each keybinding shows action name and formatted shortcut with Edit button
- Phase 4.4 (Git Integration) - Syntax Highlighting in Diff COMPLETE
  - Updated `src/ui/workspace.rs`:
    - Added syntax_highlighter field (Arc<RwLock<Highlighter>>)
    - Added detect_language_from_path() for file extension detection
    - Added highlight_diff_line() method that:
      - Strips diff prefix (+/-/space)
      - Applies tree-sitter syntax highlighting
      - Preserves prefix in output spans
    - Supports languages: rust, js, ts, tsx, python, json, toml, bash, etc.
- Phase 6.2 (Themes) - Theme Picker UI COMPLETE
  - Updated `src/ui/settings/mod.rs`:
    - Enhanced render_theme_option() with visual color previews:
      - Background, surface, text, and accent color swatches
      - Mini UI preview (sidebar + content area mockup)
      - Selected indicator checkmark
      - Fixed width cards (140px) for consistent grid layout
    - Added get_theme_preview_colors() helper function
    - Preview colors for: dark, light, high-contrast-dark, high-contrast-light
- Phase 4.4 (Git Integration) - Side-by-Side Diff View COMPLETE
  - Updated `src/ui/workspace.rs`:
    - Added diff_side_by_side state field for mode toggle
    - Added toggle_diff_mode() method
    - Added prepare_side_by_side_lines() method for parsing diff into two columns
    - Enhanced render_diff_preview() with:
      - Mode toggle button (Split/Unified)
      - Wider popup width for side-by-side mode (1000px vs 700px)
      - Two-pane layout with left (old) and right (new) sides
      - Proper handling of additions, deletions, and context lines
      - Empty placeholders for unmatched lines
  - Note: Also leveraged existing `src/ui/diff/side_by_side.rs` component for advanced use cases
- Phase 4.3 (Git Integration) - Diff Preview on File Click COMPLETE
  - Updated `src/git/repository.rs`:
    - Added file_diff() method for getting diff of a specific file
    - Added file_diff_stats() method for getting additions/deletions count
    - Handles both unstaged (index to workdir) and staged (tree to index) changes
  - Updated `src/ui/workspace.rs`:
    - Added diff_preview state field (Option<(String, String)>)
    - Added show_diff_preview() method with repository integration
    - Added hide_diff_preview() method
    - Updated FileClicked event handler to call show_diff_preview()
    - Added DeleteWorktreeRequested event handler (placeholder)
    - Added render_diff_preview() method with full popup UI:
      - Header with file path and +/- stats
      - Close button
      - Scrollable diff content with syntax coloring
      - Green background for additions, red for deletions
      - Click outside or Escape to close
- Phase 4.2 (Git Integration) - Worktree Context Menu COMPLETE
  - Updated `src/ui/sidebar/worktrees.rs`:
    - Added DeleteWorktreeRequested event
    - Added context_menu_worktree state field
    - Added show_context_menu(), hide_context_menu() methods
    - Added copy_branch_name(), copy_worktree_path() methods
    - Added request_delete_worktree() method with main worktree protection
    - Added right-click handler on worktree items
    - Added context menu popup with:
      - "Copy branch name" option
      - "Copy path" option
      - Divider
      - "Delete worktree" option (disabled for main worktree)
      - Proper click handling and menu dismiss
- Phase 7.1 (Tab System) - Tab Overflow Menu COMPLETE
  - Updated `src/ui/tabs/mod.rs`:
    - Added show_overflow_menu state field
    - Added toggle_overflow_menu(), hide_overflow_menu() methods
    - Added select_from_overflow(), close_from_overflow() methods
    - Added overflow menu button (▼) shown when tab_count > 1
    - Added render_overflow_menu() method with full dropdown:
      - Header showing tab count and "Close All" button
      - Scrollable list of all tabs
      - Pin indicator (📌) for pinned tabs
      - Dirty indicator for modified tabs
      - Active tab highlighting with accent color
      - Click to select tab and close menu
      - Close button (×) for unpinned tabs
      - Integrated with close_all_unpinned()
- Phase 5.1 (Settings UI) - Import/Export Settings COMPLETE
  - Updated `src/app/settings.rs`:
    - Added export_json() method for JSON serialization
    - Added export_to_file() method for file export
    - Added import_json() static method for parsing JSON
    - Added import_from_file() static method for file import
    - Added default_export_filename() for timestamped filenames
  - Updated `src/ui/settings/mod.rs`:
    - Added import/export state fields (show_import_export, import_mode, etc.)
    - Added show_export(), show_import(), hide_import_export() methods
    - Added apply_import() method with validation
    - Added copy_to_clipboard() and paste_from_clipboard() methods
    - Added Export and Import buttons in footer
    - Added render_import_export_dialog() with full UI:
      - Modal dialog with title and description
      - JSON text display area
      - Copy to Clipboard button for export
      - Paste from Clipboard button for import
      - Error message display
      - Apply Import button with validation
- Phase 7.1 (Tab System) - Pin Tabs COMPLETE
  - Updated `src/ui/tabs/mod.rs`:
    - Added `is_pinned` field to Tab struct
    - Added pin/unpin methods to Tab
    - Added TabPinEvent enum for pin state changes
    - Added pin_tab(), unpin_tab(), toggle_pin() methods to TabBar
    - Added sort_tabs_by_pinned() - pinned tabs always appear first
    - Added pinned_count() method
    - Added close_all_unpinned() method
    - Updated close_tab() to prevent closing pinned tabs
    - Double-click on tab toggles pin state
    - Pin indicator (📌) shown for pinned tabs
    - Pinned tabs have accent left border
    - Pinned tabs show compact view (80px vs 120px width)
    - Close button shows ⊗ for pinned (unpins instead of closes)
    - Drag-to-reorder disabled for pinned tabs

### 2025-01-26 (Session 19 - Continued x3)
- Phase 5.1 (Settings UI) - Reset to Defaults COMPLETE
  - Updated `src/ui/settings/mod.rs`:
    - Added show_reset_confirmation state field
    - Added show_reset_confirmation(), hide_reset_confirmation() methods
    - Added reset_to_defaults() method
    - Added "Reset to Defaults" button in footer
    - Added render_reset_confirmation_dialog() with warning message
    - Confirmation dialog with warning icon and message
    - Cancel and Reset buttons
- Phase 3.4 (Enhanced Code Blocks) - Search functionality COMPLETE
  - Updated `src/ui/blocks/code_block.rs` with search features:
    - SearchMatch struct for tracking match positions (line, start, end)
    - Search state fields (search_visible, search_query, search_matches, current_match_index)
    - toggle_search(), show_search(), hide_search() methods
    - set_search_query() with find_matches() for case-insensitive matching
    - next_match() and prev_match() for navigation
    - get_line_matches() and is_current_match() helpers
    - Search button in code block header
    - Search bar with query display, match count, navigation buttons
    - Line highlighting for matched lines (warning color)
    - Current match line highlighting (accent color)
    - Unit tests for SearchMatch
- Phase 13.4 (Auto-Update System) - Update Notification UI COMPLETE
  - Created `src/ui/update/mod.rs` - Update UI module exports
  - Created `src/ui/update/notification.rs` - Update notification component:
    - UpdateNotificationEvent enum (UpdateNow, RemindLater, SkipVersion, etc.)
    - UpdateNotification component with banner display
    - Version info with release type indicator
    - Action buttons: "Update Now", "Later", "Skip"
    - Expandable release notes view
    - Download/install progress display with progress bar
    - Error state display
    - Unit tests for UpdateInfo and UpdateStatus
  - Updated `src/ui/mod.rs` with update module export
- Phase 15.4 (Custom Themes Deep) - COMPLETE
  - Created `src/ui/extensions/theme_editor.rs` - Complete theme editor:
    - EditingColor enum for all theme and syntax colors
    - ThemeEditorTab enum (Colors, Syntax, Preview, Export)
    - ThemeEditor component with full visual editing
    - Color picker with hex input and live preview
    - Variant selector (Dark/Light)
    - Code preview with sample syntax highlighting
    - JSON export in Zed-compatible format
    - Unit tests for hex color conversion
  - Updated `src/ui/extensions/mod.rs` - Exported ThemeEditor and ThemeEditorEvent
  - Created `src/plugins/icons.rs` - Icon theme support:
    - IconDefinition for icon paths with optional colors
    - FileIcons with extension, filename, folder, and language mappings
    - UiIconKind enum for 50+ UI icons (navigation, actions, status, git, files, UI, chat)
    - IconThemeManifest for VS Code-compatible icon theme format
    - IconThemeMetadata for extension tracking
    - IconTheme with resolution methods for files, folders, and UI elements
    - IconLoader with extension loading, current theme management
    - parse_hex_color() for HSLA conversion
    - default_icon_theme() for fallback icons
    - Unit tests for color parsing and icon kinds
  - Updated `src/plugins/mod.rs` - Exported icons module and types
  - Updated `src/app/state.rs` - IconLoader integration:
    - Added icon_loader field (Arc<RwLock<IconLoader>>)
    - Added icon_loader() getter
    - Added load_extension_icon_themes() method
    - Added current_icon_theme() and set_icon_theme() methods
    - Added list_icon_themes() for listing available icon themes
- Phase 15 (Advanced Plugin System) - COMPLETE ✅

### 2025-01-26 (Session 19 - Continued x2)
- Phase 15.2 (Extension Marketplace UI) - COMPLETE
  - Created `src/ui/extensions/panel.rs` - Extensions panel component:
    - ExtensionsTab enum (Installed, Available, Updates)
    - ExtensionItem struct with metadata (id, name, version, author, etc.)
    - Tab switching with colored active indicators
    - Search input with filtering
    - Extension list with icons and descriptions
    - Selection and details panel
    - Enable/disable toggle per extension
    - Uninstall button with confirmation
    - Mock data for testing (3 sample extensions)
  - Created `src/ui/extensions/mod.rs` - Module exports
  - Updated `src/ui/mod.rs` with extensions module export
- Phase 15.3 (Extension Theme Loading) - COMPLETE
  - Enhanced `src/plugins/themes.rs` - Theme loader with extension support:
    - ThemeMetadata struct for author and extension tracking
    - load_extension() method to load themes from extension directories
    - load_file_with_metadata() and load_json_with_metadata() methods
    - list_by_extension() for filtering themes by extension
    - unload_extension() for cleanup when extension is removed
    - Fixed Theme creation with all required fields (variant, focus_ring, selection, accessibility)
    - Unit tests for loading and unloading
  - Enhanced `src/app/state.rs` - ThemeLoader integration:
    - Added theme_loader field (Arc<RwLock<ThemeLoader>>)
    - Added theme_loader() getter
    - Added load_extension_themes() method
    - Added get_theme() for fetching themes by name
    - Added list_all_themes() combining built-in and extension themes
  - Enhanced `src/ui/settings/mod.rs` - Extension themes in settings:
    - Updated render_appearance_tab() with extension themes section
    - Added render_extension_theme_option() for extension theme buttons
    - Author and extension ID display on theme options
    - Updated save() to handle extension theme selection
    - Support for high contrast themes in built-in section

### 2025-01-26 (Session 19 - Continued)
- Phase 14.2 (i18n) - COMPLETE
  - Added Spanish translations (`src/i18n/translations.rs`):
    - 100+ UI strings translated for Spanish (es-ES)
    - All common, sidebar, chat, settings, code, git, agent, MCP strings
  - Added German translations:
    - 100+ UI strings translated for German (de-DE)
  - Added Japanese translations:
    - 100+ UI strings translated for Japanese (ja-JP)
  - Added unit tests for all new translations
- Phase 14.2 (Language Selector UI) - COMPLETE
  - Added LanguageSetting enum to `src/app/settings.rs`:
    - Auto mode with system locale detection
    - Specific locale selection
    - effective_locale() and display_name() methods
  - Updated Settings struct with language field
  - Created language selector in settings modal:
    - Auto-detect option with current system locale display
    - List of available languages with flags
    - Visual selection indicators
    - Integration with i18n system on save
- Phase 15.1 & 15.2 (Advanced Plugin System) - COMPLETE
  - Created `src/plugins/api.rs` - Extension API bindings:
    - API_VERSION constant for compatibility checking
    - ExtensionContext with working directory and project path
    - ApiResult type for standardized return values
    - UiApi for notifications and status bar items
    - FileSystemApi with permission-based path access
    - SettingsApi for extension configuration storage
    - EventApi for subscribing to application events
    - ExtensionApi combining all sub-APIs
    - Predefined event constants (conversation, project, theme, settings)
    - Unit tests for all API components
  - Enhanced `src/plugins/host.rs` - PluginHost with API integration:
    - Integrated ExtensionApi into PluginHost
    - Created linker with host functions for WASM
    - Added show_notification host function
    - Added log host function with levels
    - Added api_version host function
    - Added settings_get and settings_set host functions
    - Extension init function calling on load
    - Proper cleanup on extension unload
    - call_extension_function() for calling WASM exports
    - get_manifest() for extension info retrieval

### 2025-01-26 (Session 19)
- Phase 13.2 (Linux Distribution Packages) - COMPLETE
  - Created `packaging/appimage/AppImageBuilder.yml` - AppImage build configuration
  - Created `packaging/appimage/claude-visual.desktop` - Desktop entry file
  - Created `packaging/flatpak/com.claude-visual.app.yml` - Flatpak manifest
  - Created `packaging/flatpak/com.claude-visual.app.metainfo.xml` - AppStream metadata
  - Created `packaging/debian/control` - Debian package control file
  - Created `packaging/debian/rules` - Debian build rules
  - Created `scripts/build-appimage.sh` - AppImage build script
  - Created `scripts/build-deb.sh` - Debian package build script
  - Created `scripts/build-flatpak.sh` - Flatpak build script
  - Updated `Makefile` with Linux packaging targets (appimage, deb, flatpak, linux-all)
  - Updated `.github/workflows/ci.yml` with Linux test and build jobs
  - Updated `.github/workflows/release.yml` with Linux artifact generation
- Phase 14.1 (Accessibility) - COMPLETE
  - Enhanced `src/app/theme.rs` - High contrast themes:
    - ThemeVariant enum (Dark, Light, HighContrastDark, HighContrastLight)
    - AccessibilitySettings struct with reduce_motion, enhanced_focus, min_contrast_ratio
    - high_contrast_dark() and high_contrast_light() theme methods
    - WCAG contrast ratio calculation (relative_luminance, contrast_ratio)
    - wcag_aa_compliant() and wcag_aaa_compliant() helpers
    - toggle_high_contrast() and set_variant() methods
    - Unit tests for theme variants and contrast checking
  - Created `src/ui/accessibility/mod.rs` - Module exports
  - Created `src/ui/accessibility/focus.rs` - Focus management:
    - FocusRingStyle configuration for focus indicators
    - FocusZone enum (Main, Sidebar, Modal, Dialog, Menu, Toolbar, StatusBar, Custom)
    - FocusTrap for modal focus management
    - FocusableElement for tracking focusable items
    - FocusManager for application-wide focus handling
    - move_focus(), set_focus(), clear_focus() methods
    - Focus trap support for modal dialogs
  - Created `src/ui/accessibility/skip_links.rs` - Skip link support:
    - SkipLinkTarget struct for skip link definitions
    - SkipLinkManager for managing skip links
    - Default skip links (main-content, chat-input, navigation, sidebar)
  - Created `src/ui/accessibility/announcements.rs` - Screen reader support:
    - AnnouncementPriority enum (Polite, Assertive)
    - Announcement struct for screen reader messages
    - LiveRegion enum for ARIA configuration
    - AnnouncementManager for queuing announcements
    - CommonAnnouncements helper with standard messages
  - Updated `src/ui/mod.rs` with accessibility module export
- Phase 14.2 (Internationalization) - COMPLETE
  - Created `src/i18n/mod.rs` - Module exports
  - Created `src/i18n/locale.rs` - Locale management:
    - Locale enum supporting 8 languages (EnUs, FrFr, EsEs, DeDe, JaJp, PtBr, ZhCn, KoKr)
    - BCP 47 language tag support (language_tag(), language_code())
    - Native and English name display
    - System locale detection (LANG, LC_ALL, LC_MESSAGES, macOS AppleLocale)
    - TextDirection enum (Ltr, Rtl) for future RTL support
    - Flag emoji for locale display
    - Unit tests for locale parsing
  - Created `src/i18n/translations.rs` - Translation system:
    - TranslationBundle for storing translations per locale
    - I18n manager with locale switching
    - 100+ English translations covering all UI strings
    - 100+ French translations for complete localization
    - t() and tf() convenience functions for translation lookup
    - t!() macro for translation with argument substitution
    - add_custom_translations() for extension translations
    - Unit tests for translation lookup and formatting
  - Updated `src/main.rs` with i18n module

### 2025-01-25 (Session 18 - Continued)
- Phase 11.4 (Code Intelligence) - COMPLETE
  - Added module exports for LSP integration files:
    - `src/ui/blocks/mod.rs` - Added code_lsp module
    - `src/ui/sidebar/mod.rs` - Added explorer_diagnostics module
    - `src/ui/chat/mod.rs` - Added input_completion module
  - Phase 11.4.6 (`src/ui/blocks/code_lsp.rs`) - Code block LSP integration:
    - CodeLspEvent enum for navigation events
    - CodeToken with position and type information
    - TokenType enum for LSP interactions
    - CodeLspConfig for feature toggles
    - CodeLspIntegration for tokenization and interaction handling
    - handle_click() for Ctrl+Click go-to-definition
    - handle_hover() for hover information
    - find_symbol_occurrences() for symbol highlighting
    - Simple tokenizer (placeholder for tree-sitter)
    - Unit tests for tokenization and symbol finding
  - Phase 11.4.7 (`src/ui/sidebar/explorer_diagnostics.rs`) - File explorer diagnostics:
    - DiagnosticCounts for error/warning/info/hint aggregation
    - ExplorerDiagnosticsConfig for display settings
    - ExplorerDiagnosticsStore with caching and parent aggregation
    - DiagnosticBadge with multiple styles (Dot, Count, IconCount)
    - IconDecoration enum for file icon overlays
    - FileEntryWithDiagnostics for tree integration
    - Unit tests for counts and aggregation
  - Phase 11.4.8 (`src/ui/chat/input_completion.rs`) - Chat input completion:
    - ChatCompletionKind enum (File, Folder, Command, Mention, Symbol, etc.)
    - ChatCompletionItem with icon, label, detail, insert text
    - CompletionConfig for behavior settings
    - CompletionTrigger enum (Slash, At, Path, Keyword, Manual)
    - InputCompletionManager for full state management
    - Fuzzy matching with simple_fuzzy_match()
    - Command, mention, and file completion sources
    - Keyboard navigation (↑/↓/Tab/Enter/Esc)
    - Unit tests for fuzzy matching and trigger detection
- Phase 11 (AI Features Enhancement) marked COMPLETE
- Updated milestones table

### 2025-01-25 (Session 18)
- Phase 10.2 (Memory Management) - COMPLETE
  - Created `src/storage/cleanup.rs` - Storage cleanup utilities:
    - CleanupConfig with presets (default, aggressive, conservative)
    - CleanupTarget enum (Conversations, Messages, Attachments, Cache, Logs, Temporary)
    - CleanupItem with should_cleanup() logic based on age and protection status
    - CleanupJob with dry_run mode and preview()
    - CleanupStats with format_space_freed() display
    - CleanupScheduler for automatic cleanup intervals
    - get_available_space_mb() for disk space checking
    - Unit tests for cleanup logic
  - Created `src/storage/compression.rs` - Message compression:
    - CompressionAlgorithm enum (None, Lz4, Zstd, Deflate)
    - CompressionConfig with presets (fast, best, balanced)
    - Compressor with compress/decompress methods
    - CompressedData with ratio(), savings(), verify() methods
    - CRC32 checksum verification
    - CompressionStats for monitoring efficiency
    - Simple RLE implementation for LZ4 fallback
    - Unit tests for compression roundtrip
  - Added cleanup and compression modules to storage/mod.rs
- Phase 10.3.2 (Database Connection Pooling) - COMPLETE
  - Created `src/storage/pool.rs` - SQLite connection pooling:
    - PoolConfig with presets (default, high_concurrency, low_memory)
    - DatabasePool with min/max connections management
    - PooledConnectionGuard for RAII-style resource management
    - PoolStats with hit rate, timeouts, health check tracking
    - WAL mode and performance pragmas (synchronous, cache_size, temp_store)
    - Health check and idle timeout support
    - Automatic cleanup of stale connections
    - SharedPool type alias for thread-safe sharing
    - create_shared_pool() factory function
    - Unit tests for pool operations
  - Added pool module to storage/mod.rs
- Phase 11.3.6 (Rollback Capability) - COMPLETE
  - Created `src/agent/rollback.rs` - Agent rollback system:
    - RollbackOperation enum for tracking changes:
      - FileCreated, FileModified, FileDeleted, FileRenamed
      - DirectoryCreated, DirectoryDeleted
      - CommandExecuted with optional rollback command
      - GitCommit, GitBranchCreated
      - DatabaseInsert, DatabaseUpdate, DatabaseDelete
      - Custom operations with handler registration
    - RollbackCheckpoint with operations list and step tracking
    - RollbackManager for full rollback workflow:
      - begin_checkpoint(), commit_checkpoint(), discard_checkpoint()
      - record_file_created(), record_file_modified(), record_file_deleted()
      - rollback_to(), rollback_last(), rollback_step()
      - Custom handler registration for extensibility
    - RollbackResult with partial rollback support
    - File content and permissions preservation on Unix
    - Git revert integration
    - Unit tests for rollback operations
  - Added rollback module to agent/mod.rs
- Phase 10.3.3 (Precompiled Tree-sitter Queries) - COMPLETE
  - Created `src/syntax/queries.rs` - Query caching system:
    - QueryCache with lazy compilation and caching
    - CompiledQuery struct with capture name indexing
    - LanguageConfig for declarative language registration
    - prewarm_queries() for startup optimization
    - Language aliases support (rs -> rust, js -> javascript)
    - QueryCacheStats for monitoring hit rates
    - Global singleton with OnceLock
    - Unit tests for caching behavior
  - Created `src/syntax/queries/*.scm` - External query files:
    - rust.scm, javascript.scm, typescript.scm
    - python.scm, json.scm, toml.scm, bash.scm
    - Loaded via include_str! for compile-time embedding
  - Updated syntax/mod.rs with queries module exports
- Phase 10.3.4 (Async Theme Loading) - COMPLETE
  - Created `src/app/theme_loader.rs` - Async theme loading:
    - ThemeLoader with async discovery and loading
    - ThemeMetadata for lightweight theme listing
    - ThemeLoadState enum (NotStarted, Loading, Loaded, Failed)
    - Theme caching with Arc<Theme>
    - JSON and TOML format support (ThemeFormat enum)
    - ThemeBuilder for programmatic theme creation
    - preload_all() for eager loading at startup
    - load_theme_async() with callback notification
    - load_theme_with_channel() with oneshot channel
    - Unit tests with tempfile
  - Updated app/mod.rs with theme_loader module
- Updated ROADMAP:
  - Phase 10.2 marked COMPLETE (Memory Management)
  - Phase 10.3 marked COMPLETE (Startup Performance)
  - Phase 10 marked MOSTLY COMPLETE
  - Phase 11.3 marked COMPLETE (Agent Mode)
  - Updated milestones table

### 2025-01-25 (Session 17)
- Phase 10.1 (Rendering Optimization) - CONTINUED
  - Created `src/ui/blocks/lazy_block.rs` - Lazy loading for code blocks:
    - LazyBlock<T> Entity with deferred rendering
    - LazyState enum (Pending, Loading, Loaded, Error)
    - LazyBlockConfig with presets for code blocks, diffs, images
    - VisibilityObserver for tracking block visibility
    - Skeleton placeholder rendering
    - Preload margin support for smoother scrolling
    - Unit tests for visibility calculations
  - Added lazy_block to blocks/mod.rs exports
- Phase 10.1.3 (Message Pooling) - COMPLETE
  - Created `src/app/pool.rs` - Entity and resource pooling:
    - ObjectPool<T> generic pool with factory and reset functions
    - PooledItem<T> guard for automatic return to pool on drop
    - StringPool for text buffer reuse
    - VecPool<T> for list buffer reuse
    - PoolStats for monitoring pool efficiency
    - prewarm() for pre-allocating pool items
    - SharedPool type alias for thread-safe sharing
    - PooledMessageData for message view recycling
    - Unit tests for pool operations
  - Added pool module to app/mod.rs
- Phase 11.2.6 (Auto-summarization) - COMPLETE
  - Created `src/ai/summarizer.rs` - Conversation summarization:
    - SummarizationConfig with presets for small/large contexts
    - ConversationMessage with role, content, token count
    - ConversationSummary with compression tracking
    - Summarizer for managing summarization workflow
    - generate_summary_prompt() for AI-based summarization
    - apply_summary() to replace messages with summary
    - extract_topics() for automatic topic detection
    - SummarizationStats for monitoring
    - SummarizationRequest for async processing
    - Unit tests for summarization logic
  - Added summarizer module to ai/mod.rs
- Phase 11.3.5 (Agent Workspace Integration) - COMPLETE
  - Created `src/ui/agent/workspace.rs` - Agent workspace panel:
    - AgentWorkspace Entity for main workspace integration
    - AgentMode enum (Disabled, Idle, Planning, Executing, Paused, Completed, Failed)
    - AgentLayout enum (Compact, Normal, Expanded)
    - AgentSettings for configurable behavior
    - start_task(), set_plan(), complete_step() workflow
    - Approval workflow with request_approval(), approve(), reject()
    - Pause/resume/cancel controls
    - Log entries with levels
    - Progress tracking with step counts
    - Full UI with header, progress bar, approval prompts, logs
    - Unit tests for mode transitions
  - Added workspace module to ui/agent/mod.rs

### 2025-01-25 (Session 16)
- Phase 12.4 (Debugging Integration) - ENHANCED
  - Created `src/ui/debug/watch.rs` - Watch expressions view:
    - WatchView Entity with expression management
    - WatchExpression with id, value, error state
    - WatchChild for nested/expanded values
    - Add/edit/remove expressions
    - Refresh single or all watches
    - Expandable children for complex values
    - Error display for failed evaluations
  - Updated `src/ui/debug/mod.rs` - Added watch module exports
- Phase 10.1 (Rendering Optimization) - CONTINUED
  - Created `src/app/debounce.rs` - Debounce utilities:
    - Debouncer with interval-based throttling
    - ThrottledCallback for value updates with trailing call
    - DebouncedChannel for async event debouncing
    - BatchAccumulator for grouping rapid updates
    - UpdateCoalescer for render pass coalescing
    - Presets for streaming (60fps), search (150ms), resize (50ms)
    - Unit tests for all components
  - Added debounce module to app/mod.rs
- Phase 10.2 (Memory Management) - CONTINUED
  - Created `src/storage/pagination.rs` - Message pagination:
    - PageInfo, Cursor (After/Before/Offset) types
    - PaginationRequest for first/last/after/before queries
    - PaginatedResult with items and page info
    - MessageWindow for virtualized list windowing
    - PaginationState tracker for loading state
    - Support for prepending older and appending newer messages
    - Unit tests for cursor parsing and window management
  - Added pagination module to storage/mod.rs
- Phase 10.3 (Startup Performance) - CONTINUED
  - Created `src/ui/components/splash.rs` - Splash screen:
    - SplashScreen Entity with progress display
    - StartupPhase enum (Initializing through Ready)
    - Progress bar with phase-based percentage
    - Spinner animation with tick_animation()
    - SplashManager for timing control
    - Minimum display time to prevent flicker
    - SplashEvent for completion/error notification
  - Added splash module to components/mod.rs
- Phase 11.4 (Code Intelligence) - CONTINUED
  - Created `src/ui/lsp/code_actions.rs` - Code actions picker:
    - CodeActionsPanel Entity
    - CodeActionItem with kind, title, diagnostics
    - CodeActionKind enum (QuickFix, Refactor variants, Source variants)
    - Preferred action highlighting
    - Keyboard navigation (↑/↓/Enter/Esc)
    - Filter by title
    - CodeActionIndicator for inline lightbulb display
  - Updated `src/ui/lsp/mod.rs` - Added code_actions exports
- Phase 12 (Developer Tools Integration) marked COMPLETE
- Updated ROADMAP with all progress

### 2025-01-25 (Session 15)
- Phase 11.4 (Code Intelligence) - IN PROGRESS
  - Created `src/lsp/mod.rs` - LSP module exports:
    - LspClient, LspClientConfig from client
    - Protocol types (Position, Range, Location, etc.)
    - LspManager, LanguageServer from manager
  - Created `src/lsp/protocol.rs` - Full LSP protocol types:
    - Position, Range, Location, TextDocumentIdentifier
    - Diagnostic, DiagnosticSeverity, DiagnosticRelatedInformation
    - CompletionItem, CompletionItemKind, CompletionItemLabelDetails
    - Hover, HoverContents (String, MarkupContent, MarkedString)
    - SignatureHelp, SignatureInformation, ParameterInformation
    - DocumentSymbol, SymbolKind, SymbolTag
    - CodeAction, CodeActionKind, WorkspaceEdit, TextEdit
    - ServerCapabilities, TextDocumentSyncKind, TextDocumentSyncOptions
    - JSON-RPC request/response structures
  - Created `src/lsp/client.rs` - LSP client implementation:
    - LspClient for language server communication
    - LspClientConfig with presets:
      - rust_analyzer() for Rust
      - typescript() for TypeScript/JavaScript
      - pyright() for Python
    - JSON-RPC 2.0 message handling over stdio
    - PendingRequest tracking with oneshot channels
    - LspEvent enum (ServerReady, Diagnostics, Progress, Error, Exited)
    - Full LSP methods: initialize, didOpen, didClose, didChange
    - Code intelligence: completion, hover, definition, references
    - Document symbols, signature help, code actions
    - Shutdown and exit handling
  - Created `src/lsp/manager.rs` - Multi-server manager:
    - Language enum (Rust, TypeScript, JavaScript, Python, Go, JSON, TOML, Markdown)
    - Language::from_extension() for file type detection
    - Language::language_id() for LSP protocol
    - Language::server_command() for default servers
    - LspManager for managing multiple server connections
    - Automatic server start on document open
    - Document version tracking (OpenDocument)
    - LspManagerEvent for unified event handling
    - Unified API: completions, hover, definition, references, document_symbols, code_actions
  - Added `mod lsp;` to main.rs
  - Created `src/ui/lsp/mod.rs` - LSP UI module exports
  - Created `src/ui/lsp/hover.rs` - HoverPanel Entity:
    - Display hover documentation from LSP
    - Parse MarkupContent and MarkedStrings
    - Code block rendering with syntax highlighting
    - Positioned at cursor location
    - Click to close
  - Created `src/ui/lsp/completion.rs` - CompletionDropdown Entity:
    - Autocomplete suggestions display
    - Keyboard navigation (↑/↓/Enter/Esc)
    - Icon and color coding by CompletionItemKind
    - Filter text highlighting
    - Scroll with max visible items
    - Click to select
  - Created `src/ui/lsp/diagnostics.rs` - DiagnosticsPanel Entity:
    - File-grouped diagnostics display
    - Severity filtering (All/Errors/Warnings)
    - Expandable/collapsible file groups
    - Error/warning counts in header
    - Go to location on click
    - Quick fix button per diagnostic
    - Color-coded severity icons
  - Created `src/ui/lsp/status.rs` - LspStatusBar Entity:
    - Server status display per language
    - Total error/warning counts
    - Clickable diagnostics toggle
    - Running server indicator
    - Language-specific icons
  - Added `pub mod lsp;` to ui/mod.rs
- Phase 11.2 (Context Management) - CONTINUED
  - Created `src/ui/ai/context_indicator.rs` - ContextIndicator Entity:
    - ContextUsage struct with token/message/file tracking
    - Progress bar with color coding (success/warning/error)
    - Compact view with expandable details panel
    - Token formatting (K/M suffixes)
    - Warning messages for high context usage
    - Click to show context panel event
  - Created `src/ui/ai/image_input.rs` - ImageInput Entity:
    - ImageAttachment struct with path, data, mime type, size
    - Drag & drop zone with visual feedback
    - Image preview grid with thumbnails
    - Remove button per image
    - Size validation (max 20MB per image, 50MB total)
    - Supported formats: PNG, JPG, GIF, WebP, SVG

### 2025-01-25 (Session 14)
- Phase 11.2 (Context Management) - CONTINUED
  - Created `src/ai/mention.rs` - @file syntax parser:
    - Mention struct with kind, position, raw text
    - MentionKind enum (File, Snippet, Url, Symbol, FileRange)
    - parse_mentions() for extracting all mentions from text
    - get_mention_at_cursor() for autocomplete support
    - PartialMention and PartialMentionKind for typing detection
    - Line range parsing (e.g., @file:src/main.rs:10-20)
    - Unit tests for mention parsing
  - Enhanced `src/ui/chat/input.rs`:
    - Added mentions tracking (parsed mentions, partial mention)
    - Added cursor position tracking
    - render_text_with_mentions() for highlighted display
    - File mention badges above input
    - FilesAttached event on submit
    - MentionPartial event for autocomplete
    - Placeholder updated to show @file syntax
  - Added mention module exports to src/ai/mod.rs
- Phase 11.3 (Agent Mode) - IN PROGRESS
  - Created `src/agent/mod.rs` - Agent module exports:
    - AgentTask, TaskStatus, TaskTree, TaskNode
    - AgentExecutor, ExecutorState, ExecutorEvent
    - AgentPlanner, Plan, PlanStep
  - Created `src/agent/task.rs` - Task management:
    - TaskStatus enum (Pending, Running, Completed, Failed, Skipped, Paused, WaitingApproval, Cancelled)
    - TaskPriority enum (Low, Normal, High, Critical)
    - ToolCall and ToolResult for tool execution
    - AgentTask with full lifecycle (start, complete, fail, pause, resume, cancel, retry)
    - TaskNode for tree structure
    - TaskTree with DFS traversal, completion stats, next pending
  - Created `src/agent/planner.rs` - Plan generation:
    - PlanStep with dependencies, risk levels, approval requirements
    - Plan with step management and task tree conversion
    - runnable_steps() for dependency resolution
    - critical_path() for longest dependency chain
    - AgentPlanner for parsing AI-generated plans
    - JSON extraction and plan validation
    - PlanError and PlanValidationError types
  - Created `src/agent/executor.rs` - Plan execution:
    - ExecutorState enum (Idle, Running, Paused, WaitingApproval, Completed, Failed, Cancelled)
    - ExecutorEvent for progress and status updates
    - PlanResult with execution stats
    - ToolExecutor trait for tool execution
    - AgentExecutor with full control flow:
      - start(), pause(), resume(), cancel()
      - approve(), reject() for approval workflow
      - Auto-approval for low-risk steps
      - Progress tracking and event emission
  - Created `src/ui/agent/mod.rs` - Agent UI module
  - Created `src/ui/agent/task_panel.rs` - TaskPanel Entity:
    - Hierarchical task tree display
    - Status indicators with colors
    - Expand/collapse toggle
    - Progress header with completion bar
    - Task selection and action events
  - Created `src/ui/agent/executor_view.rs` - ExecutorView Entity:
    - State display with icons
    - Progress bar with percentage
    - Control buttons (Start/Pause/Cancel)
    - Approval prompt with Approve/Reject
    - Duration display
  - Created `src/ui/agent/plan_view.rs` - PlanView Entity:
    - Plan header with title and progress
    - Step list with expand/collapse
    - Risk level badges (color-coded)
    - Approval requirement indicators
    - Dependency display
    - Tool badges
  - Added `mod agent;` to main.rs
  - Added `pub mod agent;` to ui/mod.rs

### 2025-01-25 (Session 13)
- Phase 11 (AI Features Enhancement) - IN PROGRESS
  - Phase 11.1 (Multi-Model Support) - COMPLETE
    - Created `src/ai/mod.rs` - AI module exports
    - Created `src/ai/provider.rs` - AIProvider trait:
      - AIProvider async trait with complete() and stream() methods
      - AIRequest/AIResponse structs
      - Message, MessageRole for conversation handling
      - ToolDefinition, ToolCall for function calling
      - StreamChunk enum for streaming responses
      - Usage, StopReason, ModelInfo structs
      - ProviderConfig for API configuration
    - Created `src/ai/claude.rs` - Claude API provider:
      - ClaudeProvider implementing AIProvider
      - Direct Claude API integration (no CLI)
      - SSE streaming support
      - Tool use support
      - Models: Opus, Sonnet, Haiku, 3.5 Sonnet
    - Created `src/ai/openai.rs` - OpenAI provider:
      - OpenAIProvider implementing AIProvider
      - Chat completions API integration
      - SSE streaming support
      - Models: GPT-4o, GPT-4o Mini, GPT-4 Turbo, o1, o1-mini
    - Created `src/ai/ollama.rs` - Ollama provider:
      - OllamaProvider for local models
      - Chat API integration
      - Dynamic model discovery
      - Models: Llama 3.2, Mistral, Code Llama, Qwen
    - Created `src/ai/context.rs` - Context management:
      - ContextItem with multiple types (File, Snippet, Diff, Web, etc.)
      - ContextManager with token limits and pinning
      - Token estimation
      - Format for AI prompt generation
    - Added `mod ai;` to main.rs
  - Phase 11.2 (Context Management) - IN PROGRESS
    - Created `src/ui/ai/mod.rs` - AI UI module
    - Created `src/ui/ai/model_selector.rs` - Model selector:
      - ModelSelector Entity with dropdown
      - Provider grouping (Cloud/Local)
      - Model search/filter
      - Cost indicators
      - Configuration status indicators
    - Created `src/ui/ai/context_panel.rs` - Context panel:
      - ContextPanel Entity
      - Context items list with icons
      - Token usage indicator with progress bar
      - Pin/unpin and remove actions
      - Attach file/snippet buttons
    - Added `pub mod ai;` to ui/mod.rs
- Updated ROADMAP with Phase 11 progress

### 2025-01-25 (Session 12)
- Phase 9 (Cloud & Collaboration) - COMPLETE
  - Phase 9.1 (Conversation Sync) - COMPLETE
    - Created `src/cloud/mod.rs` - Cloud module exports
    - Created `src/cloud/auth.rs` - OAuth authentication:
      - OAuthProvider enum (GitHub, Google, Email)
      - CloudAuth manager with PKCE support
      - Token refresh and session persistence
      - UserProfile and OAuthTokens structs
      - AuthState enum (SignedOut, Authenticating, SignedIn, Failed)
    - Created `src/cloud/storage.rs` - Encrypted cloud storage:
      - CloudStorage client for API communication
      - EncryptedData with AES-256-GCM and ChaCha20-Poly1305
      - Argon2 key derivation from password
      - StorageMetadata with versioning and conflict detection
      - StorageQuota tracking
    - Created `src/cloud/sync.rs` - Offline-first sync:
      - SyncManager with persistent queue
      - SyncOperation enum (Create, Update, Delete)
      - SyncStatus enum (Idle, Syncing, Offline, Error, Synced)
      - ConflictResolution strategies
      - SyncConflict detection and resolution
      - Queue persistence to disk
  - Phase 9.2 (Sharing) - COMPLETE
    - Created sharing UI in `src/ui/cloud/sharing.rs`
    - SharePermission enum (View, Comment, Edit)
    - ShareLink struct with URL, permissions, expiry
    - ExpiryOption enum (Never to 1 month)
    - Password protection option
  - Phase 9.4 (Cloud UI) - COMPLETE
    - Created `src/ui/cloud/mod.rs` - UI module exports
    - Created `src/ui/cloud/auth.rs` - AuthDialog Entity:
      - OAuth provider buttons
      - Loading and error states
      - Sign in/Sign up mode toggle
    - Created `src/ui/cloud/sync.rs` - Sync status UI:
      - SyncStatusIndicator (compact toolbar widget)
      - SyncStatusPanel (expanded panel with controls)
      - Auto-sync toggle and manual sync button
      - Conflict warnings and resolution
    - Created `src/ui/cloud/sharing.rs` - ShareDialog Entity:
      - Permission selector with descriptions
      - Password protection toggle
      - Expiry options selector
      - Existing links management (copy/revoke)
  - Added cloud module to main.rs
  - Added cloud UI to ui/mod.rs
  - Added dependencies to Cargo.toml:
    - reqwest (HTTP client)
    - argon2, aes-gcm, chacha20poly1305, sha2 (cryptography)
    - rand, hex, base64, urlencoding (utilities)
- Phase 9 marked COMPLETE in milestones

### 2025-01-25 (Session 11)
- Phase 13 (Distribution & Packaging) - IN PROGRESS
  - Phase 13.1 (macOS Distribution) - COMPLETE
    - Created `assets/Info.plist` - macOS app bundle configuration
    - Added [package.metadata.bundle] to Cargo.toml
    - Created `Makefile` with build commands:
      - `make build` / `make build-release`
      - `make bundle` - Creates .app bundle
      - `make dmg` - Creates DMG installer
      - `make install` - Installs to /Applications
  - Phase 13.3 (CI/CD Pipeline) - COMPLETE
    - Created `.github/workflows/ci.yml`:
      - Check job (fmt, clippy, cargo check)
      - Test job (cargo test)
      - Build job (macOS x64/arm64 artifacts)
    - Created `.github/workflows/release.yml`:
      - Triggered on version tags (v*)
      - Builds for macOS x64/arm64
      - Creates app bundles and DMGs
      - Auto-generates changelog from commits
      - Creates GitHub release with assets
  - Phase 13.4 (Auto-Update System) - COMPLETE
    - Created `src/update/mod.rs` - Module exports
    - Created `src/update/checker.rs`:
      - UpdateChecker struct for GitHub API
      - UpdateStatus enum
      - UpdateInfo struct
      - Version comparison logic
    - Created `src/update/installer.rs`:
      - UpdateInstaller struct
      - DMG mounting and installation (macOS)
      - Archive extraction (Linux)
      - Application restart handling
    - Added `mod update;` to main.rs
    - Added CheckForUpdates action
- Updated .gitignore for app bundles and DMGs

- Phase 10 (Performance & Optimization) - IN PROGRESS
  - Phase 10.1 (Rendering Optimization) - IN PROGRESS
    - Created `src/ui/components/virtual_list/mod.rs`:
      - VirtualListConfig for configurable item heights and overscan
      - VirtualListState for scroll position tracking
      - VirtualList<T> generic Entity component
      - visible_range() calculation for efficient rendering
      - scroll_to_item() and scroll_to_bottom() methods
    - Added virtual_list to components/mod.rs
  - Phase 10.2 (Memory Management) - PARTIAL
    - Created `src/app/cache.rs`:
      - LruCache<K, V> generic cache with size/entry limits
      - TTL support for automatic expiration
      - evict_oldest() for LRU eviction
      - SyntaxCache type alias (10MB limit, 5min TTL)
    - Added cache module to app/mod.rs
  - Phase 10.3 (Startup Performance) - PARTIAL
    - Created `src/app/lazy.rs`:
      - StartupMetrics for performance tracking
      - ComponentTimer for measuring init times
      - LazyInit<T> for deferred initialization
      - DeferredTaskQueue for post-startup tasks
    - Added lazy module to app/mod.rs

### 2025-01-25 (Session 10)
- MCP Settings Tab Integration
  - Added SettingsTab::Mcp to settings modal
  - render_mcp_tab() shows:
    - Server count and enabled count stats
    - Server list with status indicators
    - Config file locations info
  - MCP module exports verified and working
- Phase 8 fully verified complete

### 2025-01-25 (Session 9)
- Phase 8 (MCP Integration) - COMPLETE
  - Phase 8.1 (MCP Client Setup) - COMPLETE
    - Created `src/mcp/mod.rs` - MCP module declaration and exports
    - Created `src/mcp/protocol.rs` - JSON-RPC 2.0 and MCP protocol types:
      - JsonRpcRequest, JsonRpcResponse, JsonRpcNotification
      - McpError, McpErrorCode for error handling
      - ClientInfo, ServerInfo, InitializeParams, InitializeResult
      - McpCapabilities, ServerCapabilities (Tools, Resources, Prompts, Logging)
      - McpTool, ToolInputSchema, CallToolParams, CallToolResult, ToolContent
      - McpResource, ResourceContents, ResourceReference
      - McpPrompt, PromptArgument, PromptMessage, PromptContent
      - List/Get result types for tools, resources, prompts
    - Created `src/mcp/config.rs` - MCP server configuration:
      - McpConfig for loading/saving mcp.json
      - McpServerConfig with command, args, env, enabled, auto_approve
      - Config search paths (cwd, project root, user config dir)
      - Server management methods (enable, disable, add, remove)
      - Presets for common MCP servers:
        - filesystem, github, sqlite, brave_search
        - fetch, memory, puppeteer
    - Created `src/mcp/client.rs` - MCP client implementation:
      - McpClient for single server communication:
        - Process spawning and stdio communication
        - JSON-RPC request/response handling
        - Initialize handshake and capability negotiation
        - Tool listing and calling methods
        - Resource and prompt retrieval
      - McpManager for managing multiple server connections:
        - Connect/disconnect servers
        - Aggregate tools, resources, prompts from all servers
        - Route tool calls to appropriate server
    - Added `mod mcp;` to main.rs
  - Phase 8.2 (MCP Servers UI) - COMPLETE
    - Created `src/ui/mcp/mod.rs` - MCP UI module
    - Created `src/ui/mcp/servers.rs` - McpServersPanel Entity:
      - ServerConnectionStatus enum (Disconnected, Connecting, Connected, Failed)
      - ServerItem struct with name, config, status, tool/resource counts
      - Server list with color-coded status indicators
      - Enable/disable toggle per server
      - Connect/disconnect buttons with state management
      - Expandable panel with header showing connection count
      - Add server button
  - Phase 8.3 (Tools Integration) - COMPLETE
    - Created `src/ui/mcp/tools.rs` - McpToolsPanel Entity:
      - ToolApprovalStatus enum (Pending, ApprovedSession, ApprovedPermanent, Denied)
      - ToolItem struct with server, tool, approval status
      - PendingToolCall struct for awaiting approvals
      - Pending calls section with approve/deny buttons
      - Tool list with description and approval badges
      - Filter by name/description/server
  - Phase 8.4 (Resources & Prompts) - COMPLETE
    - Created `src/ui/mcp/resources.rs` - McpResourcesPanel Entity:
      - ResourceItem and PromptItem structs
      - ResourcesTab enum (Resources, Prompts)
      - Tab switching UI
      - Resource list with name, URI, description
      - Prompt list with arguments display
      - Click handlers for reading resources and using prompts
      - Filter support
  - Added `pub mod mcp;` to ui/mod.rs
- Phase 8 marked COMPLETE

### 2025-01-25 (Session 7)
- Phase 7.1 (Multi-Tab Conversations) - COMPLETE
  - Created `src/ui/tabs/mod.rs` - TabBar component
  - Tab struct with id, title, conversation_id, is_dirty
  - TabBar Entity with tab management
  - Keyboard shortcuts:
    - Cmd+T: New tab
    - Cmd+W: Close tab
    - Ctrl+Tab / Cmd+Shift+]: Next tab
    - Ctrl+Shift+Tab / Cmd+Shift+[: Previous tab
    - Cmd+1-9: Select tab by number
  - Drag & drop tab reordering (DraggedTab)
  - Tab title updates with conversation preview
  - Dirty indicator for unsaved changes
  - Updated Workspace to manage multiple ChatView entities
  - Added all tab-related actions to main.rs
- Phase 7.2 (Vim Mode) - COMPLETE
  - Created `src/ui/vim/mod.rs` - Vim mode module
  - Created `src/ui/vim/mode.rs` - VimState and VimMode enum
  - Created `src/ui/vim/keymaps.rs` - VimKeyHandler and VimAction
  - VimMode enum: Normal, Insert, Visual, VisualLine, VisualBlock, Command, Search
  - VimAction enum with 50+ actions:
    - Mode transitions (i, a, I, A, o, O, v, V, Ctrl+V, :, /)
    - Cursor movement (hjkl, w, b, e, 0, ^, $, G, gg, Ctrl+D/U/F/B)
    - Text manipulation (d, dd, x, X, y, yy, c, cc, C, p, P, u, Ctrl+R)
    - Search (/, n, N, f, F, t, T)
    - Operators with motions
  - VimKeyHandler for parsing key sequences
  - VimState for managing modal state, cursor, selection
  - VimStatusLine component with mode indicator and help hints
  - Count prefix support (e.g., 5j)
  - Unit tests for keymaps
- Phase 7.3 (Split Views) - COMPLETE
  - Created `src/ui/split/mod.rs` - Split view module
  - Created `src/ui/split/pane.rs` - Pane struct
  - Created `src/ui/split/split_view.rs` - SplitView Entity
  - SplitNode tree structure (Leaf/Split variants)
  - SplitDirection enum (Horizontal/Vertical)
  - Focus management with next/prev navigation
  - Split operations with max 4 panes limit
  - Recursive node rendering with resize dividers
  - Unit tests for pane count and focus
  - Keyboard shortcuts:
    - Cmd+\: Split horizontal
    - Cmd+Shift+\: Split vertical
    - Cmd+Alt+Right/Left: Focus next/prev pane
    - Cmd+Shift+W: Close pane
    - Ctrl+Shift+V: Toggle vim mode
- Phase 7 (Advanced UI) marked COMPLETE

### 2025-01-25 (Session 8)
- Phase 7.3 (Split Views) - COMPLETE
  - Created `src/ui/split/mod.rs` - Split view system
  - SplitDirection enum (Horizontal, Vertical)
  - Pane struct with id, weight, is_focused
  - SplitNode enum (Pane or Split with children)
  - SplitContainer Entity for managing split views
  - Features implemented:
    - Split tree structure with recursive rendering
    - Focus management (focus_next, focus_prev, focus_pane)
    - Split operations (split_horizontal, split_vertical)
    - Close pane functionality
    - Max 4 panes limit
    - Resize dividers with cursor styles
  - Unit tests for pane count and focus management
  - Added split module export to ui/mod.rs
- Phase 7 marked COMPLETE

### 2025-01-25 (Session 6)
- Phase 6.4 (Slash Commands API) - COMPLETE
  - Created `src/plugins/commands.rs` - Command registry and execution
  - SlashCommand struct with name, description, args, extension_id
  - CommandArg for argument specification with completions
  - CommandResult enum (Text, Markdown, Code, Error, Silent)
  - CommandContext with cwd, project, user
  - CommandRegistry with built-in commands:
    - /help - Show available commands
    - /clear - Clear conversation
    - /export - Export conversation
    - /theme - Change theme
    - /project - Switch project
    - /model - Switch Claude model
  - parse_command() for detecting /commands in input
  - list_for_autocomplete() for command suggestions
  - Unit tests for parsing and execution
- Phase 6 (Plugin System) marked COMPLETE

### 2025-01-25 (Session 5)
- Phase 5.5 (Drag & Drop) - COMPLETE
  - Added `FilesDropped` event to ProjectsSidebar
  - Added `is_drag_over` state for visual feedback
  - Implemented `on_drop` handler for ExternalPaths
  - Added drop zone overlay with icon and text
  - Added `handle_dropped_folders` in Workspace
  - Auto-add dropped folders as projects
- Phase 6.3 (Theme Loader) - COMPLETE
  - Created `src/plugins/themes.rs` - Zed theme parser
  - Parse Zed theme JSON format (ZedThemeFile, ZedThemeVariant)
  - Convert to internal ThemeColors and SyntaxColors
  - Parse hex colors (#RGB, #RRGGBB, #RRGGBBAA)
  - Parse rgba() colors
  - RGB to HSL conversion
  - Syntax highlighting style extraction

### 2025-01-25 (Session 4)
- Phase 5.3 (Conversation Search) - COMPLETE
  - Added `search_focus_handle` for interactive search input
  - Added `handle_search_key_down()` and `handle_search_input()` methods
  - Added on_key_down/on_input handlers to search field
  - Focus indicator (accent border) when search input is active
  - Escape key to clear search and return to Recent view
  - Backspace key support for editing search query

### 2025-01-25 (Session 3)
- Phase 5.2 (Settings UI) - COMPLETE
  - Created `src/ui/settings/mod.rs` - Entity-based settings modal
  - 4 tabs: Appearance, Editor, Git, Claude
  - Theme selector (dark/light) with live preview
  - Font selectors (code and UI fonts)
  - Toggle switches (sidebar, vim mode, auto-save)
  - Slider for sidebar width
  - Keyboard shortcut Cmd+, to open
  - Added `OpenSettings` action in main.rs
  - Integrated settings modal into Workspace
  - Added theme switching methods to Theme struct

### 2025-01-25 (Session 2)
- Phase 5.4 (Export Markdown) - COMPLETE
  - Added `export_to_markdown()` method to ChatView
  - Added `ExportRequested` event and handler
  - Native save dialog with default filename
  - Proper markdown formatting with headers, timestamps, code blocks
- Phase 4.4 (Diff Block) - Enhanced to COMPLETE
  - Converted DiffBlock from RenderOnce to Entity (Render trait)
  - Added proper line numbers (old/new) with alignment
  - Added hunk parsing support
  - Clickable header for collapse/expand
- Phase 5.3 (Conversation Search) - IN PROGRESS
  - Added FTS5 virtual table and sync triggers in database.rs
  - Added SearchResult model
  - Added search_messages() and rebuild_fts_index() methods
  - Enhanced HistorySidebar with search UI and results display

### 2025-01-25
- Updated Phase 4.4 (Diff Block) status to COMPLETE (Basic)
- Added Phase 8: MCP Integration
- Added Phase 9: Cloud & Collaboration
- Detailed Phase 7 tasks (Multi-Tab, Vim Mode, Split Views)
- Added Sprint Actuel section with immediate priorities
- Added technical notes for Settings UI, FTS5, MCP

### 2025-01-26 (Session 19 - Continued x13)
- **Theme Toggle Feature** - COMPLETE
  - Added `ToggleTheme` action and Cmd+Shift+T keybinding
  - Added `handle_toggle_theme()` method to Workspace
  - Toggles between dark and light themes
  - Added Theme toggle button (☀/🌙) in toolbar with shortcut hint

- **Vim Mode Toggle Feature** - COMPLETE
  - Added `toggle_vim_mode()` and `is_vim_mode_enabled()` methods to ChatView
  - Added `handle_toggle_vim_mode()` to Workspace
  - Registered Ctrl+Shift+V action handler
  - Added Vim mode toggle button in toolbar with status indicator
  - Green color when vim mode enabled, muted when disabled

- **Theme Toggle from Toolbar** - COMPLETE
  - Added `ThemeToggleRequested` event to ChatViewEvent
  - Added `request_theme_toggle()` method to ChatView
  - Theme toggle button shows sun (☀) in dark mode, moon (🌙) in light mode
  - Event handled in Workspace to update theme globally

- **Word Wrap Toggle** - COMPLETE
  - Added `word_wrap: bool` field to ChatView
  - Added `toggle_word_wrap()` and `is_word_wrap_enabled()` methods
  - Added `ToggleWordWrap` action with Alt+W keybinding
  - Added `handle_toggle_word_wrap()` handler in Workspace
  - Added "Wrap" toggle button in toolbar with ⌥W shortcut hint
  - Accent color when enabled, muted when disabled

- **Line Numbers Toggle** - COMPLETE
  - Added `show_line_numbers: bool` field to ChatView
  - Added `toggle_line_numbers()` and `is_line_numbers_enabled()` methods
  - Added `ToggleLineNumbers` action with Alt+L keybinding
  - Added `handle_toggle_line_numbers()` handler in Workspace
  - Added "#" toggle button in toolbar with ⌥L shortcut hint
  - Line numbers shown by default

- **Focus Mode** - COMPLETE
  - Added `focus_mode: bool` field to Workspace
  - Added `toggle_focus_mode()` and `is_focus_mode()` methods
  - Added `ToggleFocusMode` action with Cmd+Shift+F keybinding
  - Added `handle_toggle_focus_mode()` handler
  - Hides sidebar and tab bar for distraction-free editing
  - Press Cmd+Shift+F again to exit focus mode

### 2025-01-26 (Session 19 - Continued x14)
- **Keyboard Shortcuts Panel** - COMPLETE
  - Created `shortcuts_panel.rs` component in `src/ui/components/`
  - Added `ShortcutsPanel` and `ShortcutsPanelEvent` types
  - Defined 7 shortcut groups with all keyboard shortcuts:
    - General: Quit, Settings, Command Palette, Sidebar, Theme, Focus Mode
    - Conversations: New, Export, Copy, Clear
    - Tabs: New/Close/Navigate tabs
    - Split View: Horizontal/Vertical split, Focus/Close panes
    - Chat View: Search, Stats, Collapse/Expand
    - Message Navigation: Select, Copy, Bookmark messages
    - Editor: Vim mode, Word wrap, Line numbers
  - Added `ShowShortcuts` action with Shift+/ (?) keybinding
  - Added `show_shortcuts_panel()` and `hide_shortcuts_panel()` methods
  - Added `handle_show_shortcuts()` handler in Workspace
  - Panel features:
    - Search/filter shortcuts by key or description
    - Key combinations rendered as styled key badges
    - Grouped by category with section headers
    - Escape or click outside to close
    - Press ? to toggle the panel

- **Font Size Controls** - COMPLETE
  - Added font size control methods to Settings:
    - `increase_font_size()` / `decrease_font_size()`
    - `increase_ui_font_size()` / `decrease_ui_font_size()`
    - `increase_code_font_size()` / `decrease_code_font_size()`
    - `reset_font_size()`
  - Added constants: MIN_FONT_SIZE (8), MAX_FONT_SIZE (32), FONT_SIZE_STEP (1)
  - Added actions: `IncreaseFontSize`, `DecreaseFontSize`, `ResetFontSize`
  - Added keybindings:
    - Cmd+= / Cmd++ to increase font size
    - Cmd+- to decrease font size
    - Cmd+0 to reset font size
  - Added handlers in Workspace with auto-save
  - Updated shortcuts panel with font size shortcuts

- **High Contrast Toggle** - COMPLETE
  - Added `ToggleHighContrast` action with Cmd+Shift+H keybinding
  - Added `handle_toggle_high_contrast()` handler in Workspace
  - Uses theme's existing `toggle_high_contrast()` method
  - Preserves dark/light mode while switching high contrast on/off
  - Updated shortcuts panel with high contrast shortcut

- **Toast Notification System** - COMPLETE
  - Created `toast.rs` component in `src/ui/components/`
  - Added `Toast` struct with builder pattern:
    - `Toast::info()`, `Toast::success()`, `Toast::warning()`, `Toast::error()`
    - `.with_action()`, `.persistent()`, `.non_dismissible()`, `.with_duration()`
  - Added `ToastContainer` component for managing multiple toasts
  - Features:
    - Auto-dismiss with configurable duration (3-5 seconds default)
    - Manual dismiss with X button
    - Optional action buttons
    - Four levels with distinct icons and colors (info, success, warning, error)
    - Stacks multiple toasts at bottom-right
  - Integrated into Workspace with `show_toast()` helper method
  - Added toasts for:
    - Theme toggle (dark/light)
    - High contrast toggle (enabled/disabled)
    - Font size changes (shows current size)

- **Compact Mode Toggle** - COMPLETE
  - Added `ToggleCompactMode` action with Alt+C keybinding
  - Added `handle_toggle_compact_mode()` handler in Workspace
  - Calls ChatView's existing `toggle_compact_mode()` method
  - Shows toast notification when toggled
  - Updated shortcuts panel

- **Timestamps Toggle** - COMPLETE
  - Added `ToggleTimestamps` action with Alt+T keybinding
  - Added `handle_toggle_timestamps()` handler in Workspace
  - Calls ChatView's existing `toggle_timestamps()` method
  - Shows toast notification when toggled
  - Updated shortcuts panel

- **Auto-Scroll Toggle** - COMPLETE
  - Added `ToggleAutoScroll` action with Alt+S keybinding
  - Added `handle_toggle_auto_scroll()` handler in Workspace
  - Calls ChatView's existing `toggle_auto_scroll()` method
  - Shows toast notification when toggled
  - Updated shortcuts panel

- **Bookmarked Filter Toggle** - COMPLETE
  - Added `ToggleBookmarkedFilter` action with Alt+Shift+B keybinding
  - Added `handle_toggle_bookmarked_filter()` handler in Workspace
  - Calls ChatView's existing `toggle_bookmarked_filter()` method
  - Shows "Showing bookmarked messages only" or "Showing all messages"
  - Updated shortcuts panel

- **Message Filter Cycle** - COMPLETE
  - Added `CycleMessageFilter` action with Alt+F keybinding
  - Added `handle_cycle_message_filter()` handler in Workspace
  - Calls ChatView's existing `next_filter()` method
  - Cycles through: All -> You -> Claude -> Tools -> All
  - Shows toast with current filter (e.g., "Filter: All", "Filter: You")
  - Updated shortcuts panel

### 2025-01-26 (Session 19 - Continued x18)
- **UI Component Library Expansion** - IN PROGRESS
  - Created new components (47 total modules in `src/ui/components/`):

  - **Drawer** (`drawer.rs`) - Slide-out panel components
    - `DrawerPosition`: Left, Right, Top, Bottom
    - `DrawerSize`: Small, Medium, Large, Full, Custom
    - `DrawerPanel`: Generic slide-out drawer with header, content, footer
    - `NavigationDrawer`: Menu drawer with items, icons, badges, selection
    - `SettingsDrawer`: Sectioned settings panel with toggles, values, links
    - `SettingsItem`, `SettingsSection`, `SettingsItemType` types
    - `DrawerEvent`: Opened, Closed, BackdropClicked

  - **Calendar** (`calendar.rs`) - Date selection components
    - `SimpleDate`: Year/month/day structure
    - `Weekday`: Enum with short names and iterator
    - `CalendarSize`: Small, Medium, Large
    - `CalendarMonth`: Full month view with navigation, selection, today highlight
    - `DateRangePicker`: Two-calendar range selection
    - `MiniCalendar`: Compact calendar display
    - `YearPicker`: 12-year grid picker
    - `CalendarEvent`: DateSelected, MonthChanged
    - Date utilities: days_in_month(), first_weekday()

  - **Spinner** (`spinner.rs`) - Loading indicators
    - `SpinnerSize`: XSmall, Small, Medium, Large, XLarge, Custom
    - `SpinnerVariant`: Circular, Dots, Bars, Ring
    - `Spinner`: Basic spinner with label
    - `LoadingOverlay`: Full-container overlay with spinner and message
    - `SkeletonLoader`: Content placeholder with avatar, title, lines
    - `LoadingButton`: Button with loading state
    - `ProgressSpinner`: Circular progress with percentage
    - `InlineLoader`: Inline text with spinner

  - **FileUpload** (`file_upload.rs`) - File upload components
    - `UploadState`: Idle, DragOver, Uploading, Success, Error
    - `FileSizeLimit`: kb(), mb(), gb() constructors
    - `FileDropzone`: Drop area with icons, labels, file type hints
    - `FilePreview`: File item with icon, size, progress bar, actions
    - `FileUploadList`: Multiple file list with totals
    - `ImageUpload`: Image upload with preview, circular crop option
    - `FileUploadEvent`: FilesSelected, UploadProgress, UploadCompleted, etc.

  - **ContextMenu** (`context_menu.rs`) - Right-click menus
    - `ContextMenuItemType`: Action, Submenu, Checkbox, Radio, Separator
    - `ContextMenuItem`: Item with icon, shortcut, disabled, danger states
    - `ContextMenu`: Menu container with items
    - `FileContextMenu`: Preconfigured file operations menu
    - `EditContextMenu`: Cut/copy/paste/undo/redo menu
    - `TabContextMenu`: Tab operations (close, pin, duplicate)
    - `ContextMenuEvent`: ItemClicked, CheckboxToggled, RadioSelected, Closed

  - **Kbd** (`kbd.rs`) - Keyboard shortcut display
    - `KbdSize`: Small, Medium, Large
    - `KbdStyle`: Default, Flat, Outline, Minimal
    - `Kbd`: Single key display with common key constructors
    - `KeyboardShortcut`: Multi-key combination with parser
    - `ShortcutHint`: Label + shortcut pair
    - `ShortcutList`: Grouped shortcuts list
    - `CommonShortcuts`: Predefined file, edit, navigation groups
    - Platform-aware modifier symbols (Mac/Windows/Linux)

  - **Code** (`code.rs`) - Inline code display components
    - `CodeSize`: Small, Medium, Large
    - `InlineCode`: Code spans with copyable option
    - `Variable`: Variable name with optional value
    - `Command`: Terminal command display with shell prompt
    - `FilePath`: File path with icon detection
    - `UrlDisplay`: URL with truncation and clickability
    - `KeyValue`: Key-value pair display (inline/stacked)
    - `JsonValue`: Colored JSON value display by type

  - **Highlight** (`highlight.rs`) - Text highlighting components
    - `HighlightColor`: Yellow, Green, Blue, Pink, Orange, Purple, Gray
    - `HighlightStyle`: Background, Underline, Border, Glow
    - `Highlight`: Highlighted text span with color/style
    - `Mark`: Text with annotation overlay
    - `Strikethrough`: Struck text with optional replacement
    - `TextDiff`: Old→new text change display (inline/stacked)
    - `SearchMatch`: Text with search query highlighting
    - `MultiHighlight`: Multiple highlights in one text

  - **AvatarGroup** (`avatar_group.rs`) - Avatar collections
    - `AvatarGroupSize`: XSmall, Small, Medium, Large, XLarge
    - `GroupAvatar`: Individual avatar with name, initials, color, online status
    - `AvatarGroup`: Overlapping avatar row with overflow count
    - `AvatarStack`: Vertical avatar list with names
    - `AssigneePicker`: Assigned avatars with add button
    - `TeamMemberItem`: Full member row with role, email

  - **List** (`list.rs`) - List and list item components
    - `ListSize`: Compact, Medium, Comfortable
    - `ListStyle`: Plain, Separated, Card, Striped
    - `ListItem`: Item with primary/secondary text, leading/trailing elements
    - `List`: Simple list with selection support
    - `ActionList`: Menu-style list with icons, shortcuts
    - `ActionItem`: Action with icon, shortcut, description, danger state
    - `DescriptionList`: Definition list (horizontal, vertical, grid layouts)
    - `BulletList`: Bulleted list with custom bullets
    - `NumberedList`: Numbered list with start offset

  - **Sheet** (`sheet.rs`) - Bottom/action sheet components
    - `SheetPosition`: Bottom, Top, Left, Right
    - `SheetSize`: Auto, Small, Medium, Large, Full, Custom
    - `Sheet`: Generic sheet panel with drag handle, title, close button
    - `ActionSheet`: iOS-style action list with cancel
    - `SheetAction`: Action with destructive/disabled states
    - `ConfirmSheet`: Confirmation dialog sheet
    - `ShareSheet`: Share options grid
    - `ShareItem`: Share option with icon and label

  - **Notification** (`notification.rs`) - Notification system
    - `NotificationType`: Info, Success, Warning, Error, System
    - `Notification`: Individual notification with title, message, actions, timestamp
    - `NotificationAction`: Action button with primary flag
    - `NotificationCenter`: Panel with notification list, header, clear all
    - `NotificationBadge`: Count indicator with max display
    - `NotificationBell`: Bell icon with badge
    - `NotificationGroup`: Date-grouped notifications

  **Total modules: 53**

### 2025-01-26 (Session 19 - Continued x19)
- **UI Component Library Expansion** - CONTINUED
  - Updated mod.rs to include carousel and dialog modules from previous session
  - Created new components (64 total modules in `src/ui/components/`):

  - **Collapsible** (`collapsible.rs`) - Expandable content sections
    - `CollapsibleAnimation`: Default, Fast, Slow, None
    - `Collapsible`: Basic collapsible with trigger and content
    - `CollapsibleTrigger`: Trigger with chevron icon and sublabel
    - `CollapsibleGroup`: Group of independent collapsibles
    - `Details`: Simple details/summary component
    - `ExpandableText`: "Show more/less" text truncation
    - `FaqItem`: Question/answer collapsible
    - `FaqSection`: FAQ list with title

  - **Image** (`image.rs`) - Image display components
    - `ImageFit`: Cover, Contain, Fill, None, ScaleDown
    - `ImageState`: Loading, Loaded, Error
    - `ImageShape`: Rectangle, Rounded, Circle, Square
    - `Image`: Basic image with placeholder and error states
    - `Figure`: Image with caption (top, bottom, overlay)
    - `Thumbnail`: Small image with optional badge
    - `ImagePlaceholder`: Placeholder with icon and label
    - `ImageComparison`: Before/after slider comparison
    - `LazyImage`: Lazy-loaded image with placeholder

  - **ScrollArea** (`scroll_area.rs`) - Custom scrollable areas
    - `ScrollbarVisibility`: Auto, Always, Hover, Never
    - `ScrollbarSize`: Thin, Default, Thick
    - `ScrollDirection`: Vertical, Horizontal, Both
    - `ScrollArea`: Scrollable container with styled scrollbars
    - `ScrollIndicator`: Visual scroll position indicator
    - `ScrollButtons`: Scroll to top/bottom buttons
    - `InfiniteScroll`: Load more on scroll
    - `PullToRefresh`: Pull-down refresh indicator
    - `ScrollAnchor`: Anchor link for scroll navigation
    - `ScrollSpy`: Table of contents with active tracking

  - **Link** (`link.rs`) - Link and anchor components
    - `LinkVariant`: Default, Subtle, Underline, Bold, Button
    - `LinkSize`: Small, Medium, Large
    - `Link`: Basic link with external indicator
    - `NavLink`: Navigation link with active state, icon, badge
    - `BreadcrumbLink`: Breadcrumb item with current state
    - `SkipLink`: Accessibility skip navigation link
    - `FooterLink`: Footer-style link
    - `LinkList`: Link list with title
    - `AnchorLink`: Hash navigation anchor

  - **Label** (`label.rs`) - Form label components
    - `LabelSize`: Small, Medium, Large
    - `Label`: Form label with required/optional indicators
    - `HelperText`: Helper text with variants (success, warning, error, info)
    - `CharacterCount`: Input character counter with warning threshold
    - `FormField`: Field wrapper with label, input, helper/error
    - `FieldGroup`: Grouped fields with title and description
    - `InlineLabel`: Horizontal form label
    - `DescriptionLabel`: Term/description pair

  - **AspectRatio** (`aspect_ratio.rs`) - Aspect ratio containers
    - `Ratio`: Square, Video (16:9), Classic (4:3), Portrait, Widescreen, Photo, Golden, Custom
    - `AspectRatio`: Container maintaining aspect ratio
    - `VideoPlaceholder`: Video with play button and duration
    - `EmbedContainer`: Embed/iframe container
    - `RatioImage`: Image with aspect ratio preservation
    - `AspectGrid`: Grid of aspect-ratio items
    - `AspectCard`: Card with fixed ratio header

  - **Separator** (`separator.rs`) - Visual separator components
    - `SeparatorOrientation`: Horizontal, Vertical
    - `SeparatorStyle`: Solid, Dashed, Dotted, Gradient
    - `SeparatorThickness`: Thin, Default, Thick
    - `Separator`: Basic horizontal/vertical separator
    - `LabeledSeparator`: Separator with centered text
    - `DecorativeSeparator`: Pattern separator (dots, stars, diamonds, arrows, wave)
    - `SectionSeparator`: Separator with centered icon
    - `TimelineSeparator`: Timeline connector with active state
    - `Spacer`: Flex spacer component
    - `Gap`: Gap with optional line

  - **HoverCard** (`hover_card.rs`) - Hover card components
    - `HoverCardPosition`: Top, Bottom, Left, Right, and corners
    - `ArrowPosition`: Start, Center, End, Hidden
    - `HoverCard`: Card that appears on hover
    - `UserHoverCard`: User profile preview (avatar, bio, stats)
    - `LinkPreviewCard`: Link preview with title, description, image
    - `DefinitionCard`: Code symbol definition card
    - `InfoCard`: Simple info card with title, content, footer

  - **Resizable** (`resizable.rs`) - Resizable panel components
    - `ResizeDirection`: Horizontal, Vertical, Both
    - `HandlePosition`: Start, End, Both
    - `HandleStyle`: Line, Dots, Grip, Hidden
    - `ResizablePanel`: Panel with resize handle
    - `ResizeHandle`: Standalone resize handle
    - `ResizableSplitView`: Two-panel split view
    - `CornerResizeHandle`: Corner resize grip for dialogs

  **Total modules: 64**

### 2025-01-26 (Session 19 - Continued x20)
- **UI Component Library Expansion** - CONTINUED
  - Registered checkbox, radio_group, toggle_group modules from previous session
  - Created new components (73 total modules in `src/ui/components/`):

  - **Checkbox** (`checkbox.rs`) - Checkbox components (from previous session)
    - `CheckboxSize`: Small, Medium, Large
    - `CheckboxState`: Unchecked, Checked, Indeterminate
    - `Checkbox`: Basic checkbox with label
    - `CheckboxGroup`: Group of checkboxes
    - `CheckboxCardGroup`: Card-style checkbox options
    - `CheckboxToggle`: Checkbox styled as toggle

  - **RadioGroup** (`radio_group.rs`) - Radio button components (from previous session)
    - `RadioSize`: Small, Medium, Large
    - `RadioGroupOrientation`: Horizontal, Vertical
    - `Radio`: Single radio button
    - `RadioGroup`: Group of radio buttons
    - `RadioCardGroup`: Card-style radio options
    - `InlineRadio`: Inline radio for forms

  - **ToggleGroup** (`toggle_group.rs`) - Toggle group components (from previous session)
    - `ToggleGroupSize`: Small, Medium, Large
    - `ToggleGroupVariant`: Default, Outline, Ghost
    - `ToggleGroup`: Mutually exclusive toggle buttons
    - `SegmentedControl`: iOS-style segmented control
    - `ButtonGroup`: Group of action buttons
    - `IconToggleGroup`: Icon-only toggle group

  - **Select** (`select.rs`) - Dropdown selection components
    - `SelectSize`: Small, Medium, Large
    - `SelectVariant`: Default, Outline, Ghost, Filled
    - `SelectOption`: Option with value, label, description, icon
    - `SelectGroup`: Grouped options for selects
    - `Select`: Basic dropdown select
    - `MultiSelect`: Multiple selection dropdown
    - `GroupedSelect`: Select with option groups
    - `NativeSelect`: Native-style select
    - `SelectDropdown`: Dropdown menu component

  - **Combobox** (`combobox.rs`) - Combobox/autocomplete components
    - `ComboboxSize`: Small, Medium, Large
    - `ComboboxMode`: Filter, Search, Create
    - `ComboboxItem`: Item with value, label, description, icon
    - `Combobox`: Searchable dropdown
    - `ComboboxDropdown`: Dropdown list with keyboard navigation
    - `TagInput`: Tag input with autocomplete
    - `Autocomplete`: Simple text autocomplete

  - **Nav** (`nav.rs`) - Navigation components
    - `NavOrientation`: Horizontal, Vertical
    - `NavSize`: Small, Medium, Large
    - `NavItemVariant`: Default, Subtle, Pill, Underline
    - `NavItem`: Navigation item with icon, badge, children
    - `NavSection`: Section with title and items
    - `Nav`: Main navigation component
    - `Navbar`: Top navigation bar with logo, items, actions
    - `SidebarNav`: Sidebar navigation with header, sections, footer
    - `PageIndicator`: Dot/number page indicator

  - **Toolbar** (`toolbar.rs`) - Toolbar components
    - `ToolbarVariant`: Default, Floating, Attached, Minimal
    - `ToolbarSize`: Small, Medium, Large
    - `ToolbarPosition`: Top, Bottom, Left, Right
    - `ToolbarItem`: Tool item with icon, label, tooltip
    - `Toolbar`: Main toolbar component
    - `ToolbarGroup`: Grouped toolbar items
    - `ToolbarSeparator`: Visual separator
    - `FloatingToolbar`: Contextual floating toolbar
    - `QuickActions`: Simple action bar

  - **DatePicker** (`date_picker.rs`) - Date/time picker components
    - `DatePickerMode`: Single, Range, Multiple
    - `DatePickerSize`: Small, Medium, Large
    - `DateValue`: Date representation (year, month, day)
    - `DateRange`: Date range with start and end
    - `TimeValue`: Time representation (hour, minute, second)
    - `DatePicker`: Date selection input
    - `DateRangePicker`: Date range selection
    - `TimePicker`: Time selection with 12/24h format
    - `DateTimePicker`: Combined date and time
    - `CalendarMonth`: Month calendar view

  - **Form** (`form.rs`) - Form layout and validation components
    - `FormLayout`: Vertical, Horizontal, Inline
    - `FormSize`: Small, Medium, Large
    - `ValidationState`: None, Valid, Invalid, Warning
    - `FormFieldState`: Field state tracking
    - `Form`: Form container
    - `FormField`: Field wrapper with label, validation
    - `FormSection`: Collapsible field group
    - `FormActions`: Submit/cancel button container
    - `FormRow`: Horizontal field grouping
    - `Fieldset`: Bordered field group with legend

  - **CopyButton** (`copy_button.rs`) - Copy to clipboard components
    - `CopyButtonSize`: Small, Medium, Large
    - `CopyButtonVariant`: Default, Ghost, Outline, Subtle
    - `CopyState`: Idle, Copying, Copied, Error
    - `CopyButton`: Basic copy button with feedback
    - `CopyCodeButton`: Copy button for code blocks with position
    - `CopyLink`: Copy URL with label and preview
    - `ShareButton`: Share button with copy and social options

  - **Meter** (`meter.rs`) - Progress meters and gauges
    - `MeterSize`: Small, Medium, Large
    - `MeterVariant`: Default, Success, Warning, Danger, Info, Gradient
    - `MeterOrientation`: Horizontal, Vertical
    - `Meter`: Basic progress meter with thresholds
    - `CircularGauge`: Circular gauge display
    - `BatteryIndicator`: Battery level indicator
    - `SignalStrength`: Signal strength bars
    - `Speedometer`: Speedometer-style gauge

  - **Toggle** (`toggle.rs`) - Toggle button components
    - `ToggleSize`: Small, Medium, Large
    - `ToggleVariant`: Default, Outline, Ghost, Subtle
    - `Toggle`: Basic toggle button
    - `IconToggle`: Icon-only toggle with on/off states
    - `TextStyleToggle`: Bold/Italic/Underline style toggles
    - `FavoriteToggle`: Star/favorite toggle with count
    - `LikeToggle`: Heart/like toggle with count
    - `BookmarkToggle`: Bookmark toggle

  - **Marquee** (`marquee.rs`) - Scrolling content components
    - `MarqueeDirection`: Left, Right, Up, Down
    - `MarqueeSpeed`: Slow, Normal, Fast
    - `Marquee`: Generic scrolling content container
    - `TextMarquee`: Scrolling text
    - `LogoCarousel`: Logo carousel with grayscale option
    - `Ticker`: Stock ticker style scrolling
    - `NewsTicker`: News headline ticker with label

  - **WordRotate** (`word_rotate.rs`) - Animated text switching
    - `RotateAnimation`: Fade, SlideUp, SlideDown, SlideLeft, SlideRight, Flip, Blur, Scale
    - `RotateSpeed`: Slow, Normal, Fast
    - `WordRotate`: Cycling word animation
    - `Typewriter`: Typewriter text effect with cursor
    - `TextScramble`: Scramble/reveal text effect
    - `GradientText`: Animated gradient text
    - `CountingNumber`: Animated counting number
    - `FlipWords`: Flip animation between words

  **Total modules: 78**

### 2025-01-26 (Session 19 - Continued x21 & x22)
- **UI Component Library Expansion** - CONTINUED (100 modules milestone!)
  - Registered modules from previous sessions:
    - countdown, otp_input, sparkline, floating_action, segmented_control
    - mention, emoji_picker, sortable_list
    - audio_player, video_player, code_editor
    - theme_toggle, password_strength, language_selector

  - **ThemeToggle** (`theme_toggle.rs`) - Theme switching components
    - `ThemeMode`: Light, Dark, System
    - `ThemeToggleVariant`: Switch, Button, Segmented, Dropdown
    - `ThemeToggle`: Toggle between light/dark modes
    - `ThemeButton`: Icon button for theme switching
    - `ThemePreview`: Preview theme colors
    - `AppearanceSettings`: Full appearance settings panel

  - **PasswordStrength** (`password_strength.rs`) - Password validation components
    - `PasswordStrength`: None, VeryWeak, Weak, Fair, Good, Strong
    - `from_password()`: Calculate strength from password
    - `PasswordStrengthMeter`: Visual strength indicator
    - `PasswordRequirements`: Checklist of requirements
    - `PasswordInput`: Input with strength and visibility toggle
    - `PasswordMatcher`: Confirm password with match indicator

  - **LanguageSelector** (`language_selector.rs`) - Language/locale components
    - `Language`: Struct with code, name, native_name, rtl flag
    - Built-in language presets (en, fr, es, de, zh, ja, etc.)
    - `LanguageSelector`: Dropdown language picker
    - `LanguageDropdown`: Full dropdown with flags/labels
    - `LocaleDisplay`: Display current locale info
    - `TranslationStatus`: Translation coverage indicator

  - **InfiniteScroll** (`infinite_scroll.rs`) - Infinite loading components
    - `ScrollDirection`: Down, Up, Both
    - `LoadingState`: Idle, Loading, LoadingMore, Error, EndReached
    - `PullState`: Idle, Pulling, ReadyToRefresh, Refreshing
    - `InfiniteScroll`: Container with load-more detection
    - `LazyLoadContainer`: Load content when visible
    - `PullToRefresh`: Pull-down refresh with indicator
    - `VirtualScroller`: Large list virtualization
    - `LoadMoreTrigger`: Button/auto load more

  - **Typewriter** (`typewriter.rs`) - Text animation components
    - `TypewriterStyle`: Classic, Modern, Terminal, Glitch
    - `CursorStyle`: Line, Block, Underscore, None
    - `TypewriterText`: Character-by-character reveal
    - `TypingIndicator`: Three dots animation (Dots, Wave, Pulse, Bounce)
    - `AnimatedText`: Various text effects (FadeIn, SlideUp, Scale, etc.)
    - `CharacterReveal`: Fade/Drop/Slide/Flip/Glow per character
    - `TextScramble`: Random chars before reveal
    - `WordReveal`: Word-by-word animation

  - **Gauge** (`gauge.rs`) - Radial visualization components
    - `GaugeStyle`: Arc, Semicircle, Circle, Linear
    - `GaugeSize`: Xs, Sm, Md, Lg, Xl
    - `GaugeZone`: Value range with color and label
    - `Gauge`: Meter with zones and needle
    - `SpeedGauge`: Speedometer-style with zones
    - `CircularProgress`: Ring progress with percentage
    - `RadialChart`: Multi-segment radial chart
    - `MultiGauge`: Concentric rings for comparison

  - **Confetti** (`confetti.rs`) - Celebration animation components
    - `ConfettiShape`: Square, Circle, Rectangle, Star, Heart, Triangle
    - `ConfettiStyle`: Burst, Rain, Cannon, Fireworks, Spray
    - `ConfettiIntensity`: Low, Medium, High, Extreme
    - `ConfettiParticle`: Individual particle with physics
    - `Confetti`: Main confetti burst component
    - `EmojiBurst`: Emoji particle burst
    - `SparkleEffect`: Sparkle overlay
    - `Firework`: Firework explosion
    - `Balloon`: Floating balloon with string
    - `PartyPopper`: Directional party popper

  - **HeatMap** (`heat_map.rs`) - Heat map visualization components
    - `HeatMapScale`: Green, Blue, Purple, Orange, Red, Gray, Custom
    - `HeatMapCell`: Cell with value, label, date
    - `HeatMap`: Grid heat map with legend
    - `ContributionCalendar`: GitHub-style contribution graph
    - `ActivityStreak`: Streak display with flame icon
    - `DataGridHeatMap`: Table-style heat map

  - **Watermark** (`watermark.rs`) - Watermark overlay components
    - `WatermarkPosition`: Center, corners, edges, Tiled
    - `WatermarkVariant`: Text, Image, Pattern, Diagonal
    - `Watermark`: Text watermark with tiling
    - `ImageWatermark`: Image-based watermark
    - `SecurityWatermark`: User info security mark
    - `StampOverlay`: DRAFT/CONFIDENTIAL stamps
    - `StampType`: Draft, Confidential, Approved, Rejected, etc.
    - `PatternOverlay`: Dots, Stripes, Grid, Checkerboard patterns

  - **Steps** (`steps.rs`) - Step indicator and wizard components
    - `StepStatus`: Pending, Current, Completed, Error, Skipped
    - `StepsOrientation`: Horizontal, Vertical
    - `StepsSize`: Sm, Md, Lg
    - `StepsVariant`: Circle, CircleAlt, Line, Dot, Icon
    - `Step`: Step definition with title, description, icon
    - `Steps`: Multi-step indicator with connectors
    - `ProgressStepper`: Progress bar with step dots
    - `WizardNav`: Back/Next navigation for wizards
    - `NumberedSteps`: Numbered checklist with completion

  - **DiffViewer** (`diff_viewer.rs`) - File diff visualization components
    - `DiffViewMode`: Unified, Split, Inline
    - `DiffLineType`: Context, Added, Removed, Modified, Header, Hunk
    - `DiffLine`: Line with type, content, line numbers
    - `DiffViewer`: Unified diff with stats header
    - `SplitDiffViewer`: Side-by-side comparison
    - `InlineChange`: Inline old→new text change
    - `DiffStat`: Summary with files, additions, deletions
    - `FileChangeBadge`: File change type indicator (A/M/D/R)

  **Total modules: 100** (milestone reached!)
