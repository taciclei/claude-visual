//! Spanish translations

use super::super::bundle::TranslationBundle;
use crate::i18n::locale::Locale;

/// Spanish translations
pub fn spanish_bundle() -> TranslationBundle {
    let mut bundle = TranslationBundle::new(Locale::EsEs);

    // App
    bundle.add("app.name", "Claude Visual");
    bundle.add("app.tagline", "Un cliente visual para Claude Code");

    // Common
    bundle.add("common.ok", "Aceptar");
    bundle.add("common.cancel", "Cancelar");
    bundle.add("common.save", "Guardar");
    bundle.add("common.delete", "Eliminar");
    bundle.add("common.edit", "Editar");
    bundle.add("common.close", "Cerrar");
    bundle.add("common.open", "Abrir");
    bundle.add("common.copy", "Copiar");
    bundle.add("common.paste", "Pegar");
    bundle.add("common.cut", "Cortar");
    bundle.add("common.undo", "Deshacer");
    bundle.add("common.redo", "Rehacer");
    bundle.add("common.search", "Buscar");
    bundle.add("common.filter", "Filtrar");
    bundle.add("common.settings", "Configuración");
    bundle.add("common.help", "Ayuda");
    bundle.add("common.loading", "Cargando...");
    bundle.add("common.error", "Error");
    bundle.add("common.success", "Éxito");
    bundle.add("common.warning", "Advertencia");
    bundle.add("common.info", "Información");

    // Sidebar
    bundle.add("sidebar.projects", "Proyectos");
    bundle.add("sidebar.history", "Historial");
    bundle.add("sidebar.git", "Git");
    bundle.add("sidebar.files", "Archivos");
    bundle.add("sidebar.search_projects", "Buscar proyectos...");
    bundle.add("sidebar.no_projects", "Sin proyectos");
    bundle.add("sidebar.add_project", "Añadir proyecto");
    bundle.add("sidebar.recent", "Recientes");
    bundle.add("sidebar.favorites", "Favoritos");

    // Chat
    bundle.add(
        "chat.placeholder",
        "Escribe un mensaje... (@ para mencionar archivos)",
    );
    bundle.add("chat.send", "Enviar");
    bundle.add("chat.thinking", "Claude está pensando...");
    bundle.add("chat.stop", "Detener");
    bundle.add("chat.new_conversation", "Nueva conversación");
    bundle.add("chat.export", "Exportar");
    bundle.add("chat.clear", "Limpiar");
    bundle.add("chat.you", "Tú");
    bundle.add("chat.assistant", "Claude");

    // Settings
    bundle.add("settings.title", "Configuración");
    bundle.add("settings.appearance", "Apariencia");
    bundle.add("settings.editor", "Editor");
    bundle.add("settings.git", "Git");
    bundle.add("settings.claude", "Claude");
    bundle.add("settings.accessibility", "Accesibilidad");
    bundle.add("settings.language", "Idioma");
    bundle.add("settings.theme", "Tema");
    bundle.add("settings.theme_dark", "Oscuro");
    bundle.add("settings.theme_light", "Claro");
    bundle.add("settings.theme_hc_dark", "Alto contraste oscuro");
    bundle.add("settings.theme_hc_light", "Alto contraste claro");
    bundle.add("settings.font_size", "Tamaño de fuente");
    bundle.add("settings.font_family", "Familia de fuentes");
    bundle.add("settings.sidebar_width", "Ancho del panel");
    bundle.add("settings.vim_mode", "Modo Vim");
    bundle.add("settings.reduce_motion", "Reducir animaciones");
    bundle.add("settings.high_contrast", "Alto contraste");

    // Tabs
    bundle.add("tabs.new_tab", "Nueva pestaña");
    bundle.add("tabs.close_tab", "Cerrar pestaña");
    bundle.add("tabs.close_all", "Cerrar todo");
    bundle.add("tabs.untitled", "Sin título");

    // Code blocks
    bundle.add("code.copy", "Copiar código");
    bundle.add("code.copied", "¡Copiado!");
    bundle.add("code.run", "Ejecutar");
    bundle.add("code.save", "Guardar");
    bundle.add("code.lines", "{count} líneas");
    bundle.add("code.collapse", "Contraer");
    bundle.add("code.expand", "Expandir");

    // Diff
    bundle.add("diff.additions", "+{count}");
    bundle.add("diff.deletions", "-{count}");
    bundle.add("diff.no_changes", "Sin cambios");

    // File explorer
    bundle.add("explorer.new_file", "Nuevo archivo");
    bundle.add("explorer.new_folder", "Nueva carpeta");
    bundle.add("explorer.rename", "Renombrar");
    bundle.add("explorer.add_to_context", "Añadir al contexto");
    bundle.add("explorer.open_in_terminal", "Abrir en terminal");

    // Git
    bundle.add("git.branch", "Rama");
    bundle.add("git.commit", "Confirmar");
    bundle.add("git.push", "Subir");
    bundle.add("git.pull", "Bajar");
    bundle.add("git.status", "Estado");
    bundle.add("git.staged", "Preparado");
    bundle.add("git.unstaged", "No preparado");
    bundle.add("git.untracked", "Sin seguimiento");
    bundle.add("git.modified", "Modificado");
    bundle.add("git.added", "Añadido");
    bundle.add("git.deleted", "Eliminado");

    // Terminal
    bundle.add("terminal.title", "Terminal");
    bundle.add("terminal.clear", "Limpiar");

    // Agent
    bundle.add("agent.title", "Modo Agente");
    bundle.add("agent.planning", "Planificando...");
    bundle.add("agent.executing", "Ejecutando...");
    bundle.add("agent.paused", "Pausado");
    bundle.add("agent.completed", "Completado");
    bundle.add("agent.failed", "Fallido");
    bundle.add("agent.approve", "Aprobar");
    bundle.add("agent.reject", "Rechazar");
    bundle.add("agent.pause", "Pausar");
    bundle.add("agent.resume", "Reanudar");
    bundle.add("agent.cancel", "Cancelar");

    // MCP
    bundle.add("mcp.servers", "Servidores MCP");
    bundle.add("mcp.tools", "Herramientas");
    bundle.add("mcp.resources", "Recursos");
    bundle.add("mcp.prompts", "Prompts");
    bundle.add("mcp.connected", "Conectado");
    bundle.add("mcp.disconnected", "Desconectado");
    bundle.add("mcp.connecting", "Conectando...");

    // Keyboard shortcuts
    bundle.add("shortcuts.title", "Atajos de teclado");
    bundle.add("shortcuts.general", "General");
    bundle.add("shortcuts.navigation", "Navegación");
    bundle.add("shortcuts.editing", "Edición");

    // Accessibility
    bundle.add("a11y.skip_to_main", "Ir al contenido principal");
    bundle.add("a11y.skip_to_chat", "Ir al chat");
    bundle.add("a11y.skip_to_nav", "Ir a la navegación");
    bundle.add("a11y.dialog_opened", "Diálogo abierto: {title}");
    bundle.add("a11y.dialog_closed", "Diálogo cerrado");
    bundle.add("a11y.loading", "Cargando");
    bundle.add("a11y.message_sent", "Mensaje enviado");
    bundle.add("a11y.response_complete", "Respuesta completa");

    bundle
}
