//! Core data types for icon themes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Icon definition with optional color
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconDefinition {
    /// Path to the icon file (SVG or PNG)
    pub path: PathBuf,
    /// Optional tint color (for monochrome icons)
    pub color: Option<String>,
}

/// File icon association
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileIcons {
    /// Default file icon
    #[serde(default)]
    pub file: Option<String>,
    /// Default folder icon
    #[serde(default)]
    pub folder: Option<String>,
    /// Default folder expanded icon
    #[serde(default)]
    pub folder_expanded: Option<String>,
    /// Root folder icon
    #[serde(default)]
    pub root_folder: Option<String>,
    /// Root folder expanded icon
    #[serde(default)]
    pub root_folder_expanded: Option<String>,
    /// File extension to icon mapping
    #[serde(default)]
    pub file_extensions: HashMap<String, String>,
    /// File name to icon mapping
    #[serde(default)]
    pub file_names: HashMap<String, String>,
    /// Folder name to icon mapping
    #[serde(default)]
    pub folder_names: HashMap<String, String>,
    /// Folder name expanded to icon mapping
    #[serde(default)]
    pub folder_names_expanded: HashMap<String, String>,
    /// Language ID to icon mapping
    #[serde(default)]
    pub language_ids: HashMap<String, String>,
}

/// UI icon category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UiIconKind {
    // Navigation
    ChevronRight,
    ChevronDown,
    ChevronLeft,
    ChevronUp,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,

    // Actions
    Close,
    Add,
    Remove,
    Edit,
    Save,
    Copy,
    Paste,
    Cut,
    Undo,
    Redo,
    Refresh,
    Search,
    Filter,
    Settings,

    // Status
    Check,
    Warning,
    Error,
    Info,
    Loading,

    // Git
    GitBranch,
    GitCommit,
    GitMerge,
    GitPullRequest,
    GitCompare,

    // Files
    NewFile,
    NewFolder,
    OpenFile,
    OpenFolder,
    SaveFile,
    CloseFile,

    // UI
    Sidebar,
    Terminal,
    Extensions,
    Debug,
    Source,
    Output,
    Problems,

    // Chat
    Send,
    Microphone,
    Attachment,
    User,
    Assistant,
}

impl UiIconKind {
    /// Get all UI icon kinds
    pub fn all() -> &'static [UiIconKind] {
        &[
            Self::ChevronRight,
            Self::ChevronDown,
            Self::ChevronLeft,
            Self::ChevronUp,
            Self::ArrowLeft,
            Self::ArrowRight,
            Self::ArrowUp,
            Self::ArrowDown,
            Self::Close,
            Self::Add,
            Self::Remove,
            Self::Edit,
            Self::Save,
            Self::Copy,
            Self::Paste,
            Self::Cut,
            Self::Undo,
            Self::Redo,
            Self::Refresh,
            Self::Search,
            Self::Filter,
            Self::Settings,
            Self::Check,
            Self::Warning,
            Self::Error,
            Self::Info,
            Self::Loading,
            Self::GitBranch,
            Self::GitCommit,
            Self::GitMerge,
            Self::GitPullRequest,
            Self::GitCompare,
            Self::NewFile,
            Self::NewFolder,
            Self::OpenFile,
            Self::OpenFolder,
            Self::SaveFile,
            Self::CloseFile,
            Self::Sidebar,
            Self::Terminal,
            Self::Extensions,
            Self::Debug,
            Self::Source,
            Self::Output,
            Self::Problems,
            Self::Send,
            Self::Microphone,
            Self::Attachment,
            Self::User,
            Self::Assistant,
        ]
    }

    /// Get the default icon name
    pub fn default_name(&self) -> &'static str {
        match self {
            Self::ChevronRight => "chevron-right",
            Self::ChevronDown => "chevron-down",
            Self::ChevronLeft => "chevron-left",
            Self::ChevronUp => "chevron-up",
            Self::ArrowLeft => "arrow-left",
            Self::ArrowRight => "arrow-right",
            Self::ArrowUp => "arrow-up",
            Self::ArrowDown => "arrow-down",
            Self::Close => "close",
            Self::Add => "add",
            Self::Remove => "remove",
            Self::Edit => "edit",
            Self::Save => "save",
            Self::Copy => "copy",
            Self::Paste => "paste",
            Self::Cut => "cut",
            Self::Undo => "undo",
            Self::Redo => "redo",
            Self::Refresh => "refresh",
            Self::Search => "search",
            Self::Filter => "filter",
            Self::Settings => "settings",
            Self::Check => "check",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Info => "info",
            Self::Loading => "loading",
            Self::GitBranch => "git-branch",
            Self::GitCommit => "git-commit",
            Self::GitMerge => "git-merge",
            Self::GitPullRequest => "git-pull-request",
            Self::GitCompare => "git-compare",
            Self::NewFile => "new-file",
            Self::NewFolder => "new-folder",
            Self::OpenFile => "open-file",
            Self::OpenFolder => "open-folder",
            Self::SaveFile => "save-file",
            Self::CloseFile => "close-file",
            Self::Sidebar => "sidebar",
            Self::Terminal => "terminal",
            Self::Extensions => "extensions",
            Self::Debug => "debug",
            Self::Source => "source",
            Self::Output => "output",
            Self::Problems => "problems",
            Self::Send => "send",
            Self::Microphone => "microphone",
            Self::Attachment => "attachment",
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

/// Icon theme manifest (icons.json or icon-theme.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconThemeManifest {
    /// Theme ID
    pub id: String,
    /// Theme name for display
    pub label: String,
    /// Icon definitions
    #[serde(default)]
    pub icon_definitions: HashMap<String, IconDefinition>,
    /// File icon associations
    #[serde(flatten)]
    pub file_icons: FileIcons,
    /// UI icon overrides
    #[serde(default)]
    pub ui_icons: HashMap<String, String>,
    /// Light theme variant overrides
    #[serde(default)]
    pub light: Option<Box<IconThemeManifest>>,
    /// High contrast variant overrides
    #[serde(default)]
    pub high_contrast: Option<Box<IconThemeManifest>>,
}

/// Metadata for loaded icon themes
#[derive(Debug, Clone)]
pub struct IconThemeMetadata {
    pub id: String,
    pub name: String,
    pub extension_id: Option<String>,
    pub path: Option<PathBuf>,
}

/// Loaded icon theme
#[derive(Debug, Clone)]
pub struct IconTheme {
    pub(crate) metadata: IconThemeMetadata,
    pub(crate) manifest: IconThemeManifest,
    /// Base path for resolving icon paths
    pub(crate) base_path: PathBuf,
}
