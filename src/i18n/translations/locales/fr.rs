//! French translations

use super::super::bundle::TranslationBundle;
use crate::i18n::locale::Locale;

/// French translations
pub fn french_bundle() -> TranslationBundle {
    let mut bundle = TranslationBundle::new(Locale::FrFr);

    // App
    bundle.add("app.name", "Claude Visual");
    bundle.add("app.tagline", "Un client visuel pour Claude Code");

    // Common
    bundle.add("common.ok", "OK");
    bundle.add("common.cancel", "Annuler");
    bundle.add("common.save", "Enregistrer");
    bundle.add("common.delete", "Supprimer");
    bundle.add("common.edit", "Modifier");
    bundle.add("common.close", "Fermer");
    bundle.add("common.open", "Ouvrir");
    bundle.add("common.copy", "Copier");
    bundle.add("common.paste", "Coller");
    bundle.add("common.cut", "Couper");
    bundle.add("common.undo", "Annuler");
    bundle.add("common.redo", "Rétablir");
    bundle.add("common.search", "Rechercher");
    bundle.add("common.filter", "Filtrer");
    bundle.add("common.settings", "Paramètres");
    bundle.add("common.help", "Aide");
    bundle.add("common.loading", "Chargement...");
    bundle.add("common.error", "Erreur");
    bundle.add("common.success", "Succès");
    bundle.add("common.warning", "Avertissement");
    bundle.add("common.info", "Info");

    // Sidebar
    bundle.add("sidebar.projects", "Projets");
    bundle.add("sidebar.history", "Historique");
    bundle.add("sidebar.git", "Git");
    bundle.add("sidebar.files", "Fichiers");
    bundle.add("sidebar.search_projects", "Rechercher des projets...");
    bundle.add("sidebar.no_projects", "Aucun projet");
    bundle.add("sidebar.add_project", "Ajouter un projet");
    bundle.add("sidebar.recent", "Récents");
    bundle.add("sidebar.favorites", "Favoris");

    // Chat
    bundle.add("chat.placeholder", "Tapez un message... (@ pour mentionner des fichiers)");
    bundle.add("chat.send", "Envoyer");
    bundle.add("chat.thinking", "Claude réfléchit...");
    bundle.add("chat.stop", "Arrêter");
    bundle.add("chat.new_conversation", "Nouvelle conversation");
    bundle.add("chat.export", "Exporter");
    bundle.add("chat.clear", "Effacer");
    bundle.add("chat.you", "Vous");
    bundle.add("chat.assistant", "Claude");

    // Settings
    bundle.add("settings.title", "Paramètres");
    bundle.add("settings.appearance", "Apparence");
    bundle.add("settings.editor", "Éditeur");
    bundle.add("settings.git", "Git");
    bundle.add("settings.claude", "Claude");
    bundle.add("settings.accessibility", "Accessibilité");
    bundle.add("settings.language", "Langue");
    bundle.add("settings.theme", "Thème");
    bundle.add("settings.theme_dark", "Sombre");
    bundle.add("settings.theme_light", "Clair");
    bundle.add("settings.theme_hc_dark", "Contraste élevé sombre");
    bundle.add("settings.theme_hc_light", "Contraste élevé clair");
    bundle.add("settings.font_size", "Taille de police");
    bundle.add("settings.font_family", "Police");
    bundle.add("settings.sidebar_width", "Largeur du panneau");
    bundle.add("settings.vim_mode", "Mode Vim");
    bundle.add("settings.reduce_motion", "Réduire les animations");
    bundle.add("settings.high_contrast", "Contraste élevé");

    // Tabs
    bundle.add("tabs.new_tab", "Nouvel onglet");
    bundle.add("tabs.close_tab", "Fermer l'onglet");
    bundle.add("tabs.close_all", "Tout fermer");
    bundle.add("tabs.untitled", "Sans titre");

    // Code blocks
    bundle.add("code.copy", "Copier le code");
    bundle.add("code.copied", "Copié !");
    bundle.add("code.run", "Exécuter");
    bundle.add("code.save", "Enregistrer");
    bundle.add("code.lines", "{count} lignes");
    bundle.add("code.collapse", "Réduire");
    bundle.add("code.expand", "Développer");

    // Diff
    bundle.add("diff.additions", "+{count}");
    bundle.add("diff.deletions", "-{count}");
    bundle.add("diff.no_changes", "Aucune modification");

    // File explorer
    bundle.add("explorer.new_file", "Nouveau fichier");
    bundle.add("explorer.new_folder", "Nouveau dossier");
    bundle.add("explorer.rename", "Renommer");
    bundle.add("explorer.add_to_context", "Ajouter au contexte");
    bundle.add("explorer.open_in_terminal", "Ouvrir dans le terminal");

    // Git
    bundle.add("git.branch", "Branche");
    bundle.add("git.commit", "Valider");
    bundle.add("git.push", "Pousser");
    bundle.add("git.pull", "Tirer");
    bundle.add("git.status", "Statut");
    bundle.add("git.staged", "Indexé");
    bundle.add("git.unstaged", "Non indexé");
    bundle.add("git.untracked", "Non suivi");
    bundle.add("git.modified", "Modifié");
    bundle.add("git.added", "Ajouté");
    bundle.add("git.deleted", "Supprimé");

    // Terminal
    bundle.add("terminal.title", "Terminal");
    bundle.add("terminal.clear", "Effacer");

    // Agent
    bundle.add("agent.title", "Mode Agent");
    bundle.add("agent.planning", "Planification...");
    bundle.add("agent.executing", "Exécution...");
    bundle.add("agent.paused", "En pause");
    bundle.add("agent.completed", "Terminé");
    bundle.add("agent.failed", "Échoué");
    bundle.add("agent.approve", "Approuver");
    bundle.add("agent.reject", "Rejeter");
    bundle.add("agent.pause", "Pause");
    bundle.add("agent.resume", "Reprendre");
    bundle.add("agent.cancel", "Annuler");

    // MCP
    bundle.add("mcp.servers", "Serveurs MCP");
    bundle.add("mcp.tools", "Outils");
    bundle.add("mcp.resources", "Ressources");
    bundle.add("mcp.prompts", "Prompts");
    bundle.add("mcp.connected", "Connecté");
    bundle.add("mcp.disconnected", "Déconnecté");
    bundle.add("mcp.connecting", "Connexion...");

    // Keyboard shortcuts
    bundle.add("shortcuts.title", "Raccourcis clavier");
    bundle.add("shortcuts.general", "Général");
    bundle.add("shortcuts.navigation", "Navigation");
    bundle.add("shortcuts.editing", "Édition");

    // Accessibility
    bundle.add("a11y.skip_to_main", "Aller au contenu principal");
    bundle.add("a11y.skip_to_chat", "Aller à la saisie");
    bundle.add("a11y.skip_to_nav", "Aller à la navigation");
    bundle.add("a11y.dialog_opened", "Dialogue ouvert : {title}");
    bundle.add("a11y.dialog_closed", "Dialogue fermé");
    bundle.add("a11y.loading", "Chargement");
    bundle.add("a11y.message_sent", "Message envoyé");
    bundle.add("a11y.response_complete", "Réponse terminée");

    bundle
}
