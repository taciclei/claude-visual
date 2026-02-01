//! German translations

use super::super::bundle::TranslationBundle;
use crate::i18n::locale::Locale;

/// German translations
pub fn german_bundle() -> TranslationBundle {
    let mut bundle = TranslationBundle::new(Locale::DeDe);

    // App
    bundle.add("app.name", "Claude Visual");
    bundle.add("app.tagline", "Ein visueller Client für Claude Code");

    // Common
    bundle.add("common.ok", "OK");
    bundle.add("common.cancel", "Abbrechen");
    bundle.add("common.save", "Speichern");
    bundle.add("common.delete", "Löschen");
    bundle.add("common.edit", "Bearbeiten");
    bundle.add("common.close", "Schließen");
    bundle.add("common.open", "Öffnen");
    bundle.add("common.copy", "Kopieren");
    bundle.add("common.paste", "Einfügen");
    bundle.add("common.cut", "Ausschneiden");
    bundle.add("common.undo", "Rückgängig");
    bundle.add("common.redo", "Wiederholen");
    bundle.add("common.search", "Suchen");
    bundle.add("common.filter", "Filtern");
    bundle.add("common.settings", "Einstellungen");
    bundle.add("common.help", "Hilfe");
    bundle.add("common.loading", "Laden...");
    bundle.add("common.error", "Fehler");
    bundle.add("common.success", "Erfolg");
    bundle.add("common.warning", "Warnung");
    bundle.add("common.info", "Info");

    // Sidebar
    bundle.add("sidebar.projects", "Projekte");
    bundle.add("sidebar.history", "Verlauf");
    bundle.add("sidebar.git", "Git");
    bundle.add("sidebar.files", "Dateien");
    bundle.add("sidebar.search_projects", "Projekte suchen...");
    bundle.add("sidebar.no_projects", "Keine Projekte");
    bundle.add("sidebar.add_project", "Projekt hinzufügen");
    bundle.add("sidebar.recent", "Zuletzt verwendet");
    bundle.add("sidebar.favorites", "Favoriten");

    // Chat
    bundle.add(
        "chat.placeholder",
        "Nachricht eingeben... (@ für Dateien erwähnen)",
    );
    bundle.add("chat.send", "Senden");
    bundle.add("chat.thinking", "Claude denkt nach...");
    bundle.add("chat.stop", "Stopp");
    bundle.add("chat.new_conversation", "Neue Unterhaltung");
    bundle.add("chat.export", "Exportieren");
    bundle.add("chat.clear", "Löschen");
    bundle.add("chat.you", "Du");
    bundle.add("chat.assistant", "Claude");

    // Settings
    bundle.add("settings.title", "Einstellungen");
    bundle.add("settings.appearance", "Erscheinungsbild");
    bundle.add("settings.editor", "Editor");
    bundle.add("settings.git", "Git");
    bundle.add("settings.claude", "Claude");
    bundle.add("settings.accessibility", "Barrierefreiheit");
    bundle.add("settings.language", "Sprache");
    bundle.add("settings.theme", "Design");
    bundle.add("settings.theme_dark", "Dunkel");
    bundle.add("settings.theme_light", "Hell");
    bundle.add("settings.theme_hc_dark", "Hoher Kontrast dunkel");
    bundle.add("settings.theme_hc_light", "Hoher Kontrast hell");
    bundle.add("settings.font_size", "Schriftgröße");
    bundle.add("settings.font_family", "Schriftart");
    bundle.add("settings.sidebar_width", "Seitenleistenbreite");
    bundle.add("settings.vim_mode", "Vim-Modus");
    bundle.add("settings.reduce_motion", "Bewegung reduzieren");
    bundle.add("settings.high_contrast", "Hoher Kontrast");

    // Tabs
    bundle.add("tabs.new_tab", "Neuer Tab");
    bundle.add("tabs.close_tab", "Tab schließen");
    bundle.add("tabs.close_all", "Alle schließen");
    bundle.add("tabs.untitled", "Unbenannt");

    // Code blocks
    bundle.add("code.copy", "Code kopieren");
    bundle.add("code.copied", "Kopiert!");
    bundle.add("code.run", "Ausführen");
    bundle.add("code.save", "Speichern");
    bundle.add("code.lines", "{count} Zeilen");
    bundle.add("code.collapse", "Einklappen");
    bundle.add("code.expand", "Ausklappen");

    // Diff
    bundle.add("diff.additions", "+{count}");
    bundle.add("diff.deletions", "-{count}");
    bundle.add("diff.no_changes", "Keine Änderungen");

    // File explorer
    bundle.add("explorer.new_file", "Neue Datei");
    bundle.add("explorer.new_folder", "Neuer Ordner");
    bundle.add("explorer.rename", "Umbenennen");
    bundle.add("explorer.add_to_context", "Zum Kontext hinzufügen");
    bundle.add("explorer.open_in_terminal", "Im Terminal öffnen");

    // Git
    bundle.add("git.branch", "Branch");
    bundle.add("git.commit", "Commit");
    bundle.add("git.push", "Push");
    bundle.add("git.pull", "Pull");
    bundle.add("git.status", "Status");
    bundle.add("git.staged", "Bereitgestellt");
    bundle.add("git.unstaged", "Nicht bereitgestellt");
    bundle.add("git.untracked", "Unverfolgt");
    bundle.add("git.modified", "Geändert");
    bundle.add("git.added", "Hinzugefügt");
    bundle.add("git.deleted", "Gelöscht");

    // Terminal
    bundle.add("terminal.title", "Terminal");
    bundle.add("terminal.clear", "Löschen");

    // Agent
    bundle.add("agent.title", "Agent-Modus");
    bundle.add("agent.planning", "Planung...");
    bundle.add("agent.executing", "Ausführung...");
    bundle.add("agent.paused", "Pausiert");
    bundle.add("agent.completed", "Abgeschlossen");
    bundle.add("agent.failed", "Fehlgeschlagen");
    bundle.add("agent.approve", "Genehmigen");
    bundle.add("agent.reject", "Ablehnen");
    bundle.add("agent.pause", "Pause");
    bundle.add("agent.resume", "Fortsetzen");
    bundle.add("agent.cancel", "Abbrechen");

    // MCP
    bundle.add("mcp.servers", "MCP-Server");
    bundle.add("mcp.tools", "Werkzeuge");
    bundle.add("mcp.resources", "Ressourcen");
    bundle.add("mcp.prompts", "Prompts");
    bundle.add("mcp.connected", "Verbunden");
    bundle.add("mcp.disconnected", "Getrennt");
    bundle.add("mcp.connecting", "Verbindung wird hergestellt...");

    // Keyboard shortcuts
    bundle.add("shortcuts.title", "Tastaturkürzel");
    bundle.add("shortcuts.general", "Allgemein");
    bundle.add("shortcuts.navigation", "Navigation");
    bundle.add("shortcuts.editing", "Bearbeiten");

    // Accessibility
    bundle.add("a11y.skip_to_main", "Zum Hauptinhalt springen");
    bundle.add("a11y.skip_to_chat", "Zur Chat-Eingabe springen");
    bundle.add("a11y.skip_to_nav", "Zur Navigation springen");
    bundle.add("a11y.dialog_opened", "Dialog geöffnet: {title}");
    bundle.add("a11y.dialog_closed", "Dialog geschlossen");
    bundle.add("a11y.loading", "Laden");
    bundle.add("a11y.message_sent", "Nachricht gesendet");
    bundle.add("a11y.response_complete", "Antwort vollständig");

    bundle
}
