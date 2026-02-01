//! English translations

use super::super::bundle::TranslationBundle;
use crate::i18n::locale::Locale;

/// Default English translations
pub fn english_bundle() -> TranslationBundle {
    let mut bundle = TranslationBundle::new(Locale::EnUs);

    // App
    bundle.add("app.name", "Claude Visual");
    bundle.add("app.tagline", "A visual client for Claude Code");

    // Common
    bundle.add("common.ok", "OK");
    bundle.add("common.cancel", "Cancel");
    bundle.add("common.save", "Save");
    bundle.add("common.delete", "Delete");
    bundle.add("common.edit", "Edit");
    bundle.add("common.close", "Close");
    bundle.add("common.open", "Open");
    bundle.add("common.copy", "Copy");
    bundle.add("common.paste", "Paste");
    bundle.add("common.cut", "Cut");
    bundle.add("common.undo", "Undo");
    bundle.add("common.redo", "Redo");
    bundle.add("common.search", "Search");
    bundle.add("common.filter", "Filter");
    bundle.add("common.settings", "Settings");
    bundle.add("common.help", "Help");
    bundle.add("common.loading", "Loading...");
    bundle.add("common.error", "Error");
    bundle.add("common.success", "Success");
    bundle.add("common.warning", "Warning");
    bundle.add("common.info", "Info");

    // Sidebar
    bundle.add("sidebar.projects", "Projects");
    bundle.add("sidebar.history", "History");
    bundle.add("sidebar.git", "Git");
    bundle.add("sidebar.files", "Files");
    bundle.add("sidebar.search_projects", "Search projects...");
    bundle.add("sidebar.no_projects", "No projects yet");
    bundle.add("sidebar.add_project", "Add Project");
    bundle.add("sidebar.recent", "Recent");
    bundle.add("sidebar.favorites", "Favorites");

    // Chat
    bundle.add("chat.placeholder", "Type a message... (@ to mention files)");
    bundle.add("chat.send", "Send");
    bundle.add("chat.thinking", "Claude is thinking...");
    bundle.add("chat.stop", "Stop");
    bundle.add("chat.new_conversation", "New Conversation");
    bundle.add("chat.export", "Export");
    bundle.add("chat.clear", "Clear");
    bundle.add("chat.you", "You");
    bundle.add("chat.assistant", "Claude");

    // Settings
    bundle.add("settings.title", "Settings");
    bundle.add("settings.appearance", "Appearance");
    bundle.add("settings.editor", "Editor");
    bundle.add("settings.git", "Git");
    bundle.add("settings.claude", "Claude");
    bundle.add("settings.accessibility", "Accessibility");
    bundle.add("settings.language", "Language");
    bundle.add("settings.theme", "Theme");
    bundle.add("settings.theme_dark", "Dark");
    bundle.add("settings.theme_light", "Light");
    bundle.add("settings.theme_hc_dark", "High Contrast Dark");
    bundle.add("settings.theme_hc_light", "High Contrast Light");
    bundle.add("settings.font_size", "Font Size");
    bundle.add("settings.font_family", "Font Family");
    bundle.add("settings.sidebar_width", "Sidebar Width");
    bundle.add("settings.vim_mode", "Vim Mode");
    bundle.add("settings.reduce_motion", "Reduce Motion");
    bundle.add("settings.high_contrast", "High Contrast");

    // Tabs
    bundle.add("tabs.new_tab", "New Tab");
    bundle.add("tabs.close_tab", "Close Tab");
    bundle.add("tabs.close_all", "Close All");
    bundle.add("tabs.untitled", "Untitled");

    // Code blocks
    bundle.add("code.copy", "Copy Code");
    bundle.add("code.copied", "Copied!");
    bundle.add("code.run", "Run");
    bundle.add("code.save", "Save");
    bundle.add("code.lines", "{count} lines");
    bundle.add("code.collapse", "Collapse");
    bundle.add("code.expand", "Expand");

    // Diff
    bundle.add("diff.additions", "+{count}");
    bundle.add("diff.deletions", "-{count}");
    bundle.add("diff.no_changes", "No changes");

    // File explorer
    bundle.add("explorer.new_file", "New File");
    bundle.add("explorer.new_folder", "New Folder");
    bundle.add("explorer.rename", "Rename");
    bundle.add("explorer.add_to_context", "Add to Context");
    bundle.add("explorer.open_in_terminal", "Open in Terminal");

    // Git
    bundle.add("git.branch", "Branch");
    bundle.add("git.commit", "Commit");
    bundle.add("git.push", "Push");
    bundle.add("git.pull", "Pull");
    bundle.add("git.status", "Status");
    bundle.add("git.staged", "Staged");
    bundle.add("git.unstaged", "Unstaged");
    bundle.add("git.untracked", "Untracked");
    bundle.add("git.modified", "Modified");
    bundle.add("git.added", "Added");
    bundle.add("git.deleted", "Deleted");

    // Terminal
    bundle.add("terminal.title", "Terminal");
    bundle.add("terminal.clear", "Clear");

    // Agent
    bundle.add("agent.title", "Agent Mode");
    bundle.add("agent.planning", "Planning...");
    bundle.add("agent.executing", "Executing...");
    bundle.add("agent.paused", "Paused");
    bundle.add("agent.completed", "Completed");
    bundle.add("agent.failed", "Failed");
    bundle.add("agent.approve", "Approve");
    bundle.add("agent.reject", "Reject");
    bundle.add("agent.pause", "Pause");
    bundle.add("agent.resume", "Resume");
    bundle.add("agent.cancel", "Cancel");

    // MCP
    bundle.add("mcp.servers", "MCP Servers");
    bundle.add("mcp.tools", "Tools");
    bundle.add("mcp.resources", "Resources");
    bundle.add("mcp.prompts", "Prompts");
    bundle.add("mcp.connected", "Connected");
    bundle.add("mcp.disconnected", "Disconnected");
    bundle.add("mcp.connecting", "Connecting...");

    // Keyboard shortcuts
    bundle.add("shortcuts.title", "Keyboard Shortcuts");
    bundle.add("shortcuts.general", "General");
    bundle.add("shortcuts.navigation", "Navigation");
    bundle.add("shortcuts.editing", "Editing");

    // Accessibility
    bundle.add("a11y.skip_to_main", "Skip to main content");
    bundle.add("a11y.skip_to_chat", "Skip to chat input");
    bundle.add("a11y.skip_to_nav", "Skip to navigation");
    bundle.add("a11y.dialog_opened", "Dialog opened: {title}");
    bundle.add("a11y.dialog_closed", "Dialog closed");
    bundle.add("a11y.loading", "Loading");
    bundle.add("a11y.message_sent", "Message sent");
    bundle.add("a11y.response_complete", "Response complete");

    bundle
}
