//! Portuguese translations

use super::super::bundle::TranslationBundle;
use crate::i18n::locale::Locale;

/// Portuguese (Brazil) translations
pub fn portuguese_bundle() -> TranslationBundle {
    let mut bundle = TranslationBundle::new(Locale::PtBr);

    // App
    bundle.add("app.name", "Claude Visual");
    bundle.add("app.tagline", "Um cliente visual para Claude Code");

    // Common
    bundle.add("common.ok", "OK");
    bundle.add("common.cancel", "Cancelar");
    bundle.add("common.save", "Salvar");
    bundle.add("common.delete", "Excluir");
    bundle.add("common.edit", "Editar");
    bundle.add("common.close", "Fechar");
    bundle.add("common.open", "Abrir");
    bundle.add("common.copy", "Copiar");
    bundle.add("common.paste", "Colar");
    bundle.add("common.cut", "Recortar");
    bundle.add("common.undo", "Desfazer");
    bundle.add("common.redo", "Refazer");
    bundle.add("common.search", "Pesquisar");
    bundle.add("common.filter", "Filtrar");
    bundle.add("common.settings", "Configurações");
    bundle.add("common.help", "Ajuda");
    bundle.add("common.loading", "Carregando...");
    bundle.add("common.error", "Erro");
    bundle.add("common.success", "Sucesso");
    bundle.add("common.warning", "Aviso");
    bundle.add("common.info", "Informação");

    // Sidebar
    bundle.add("sidebar.projects", "Projetos");
    bundle.add("sidebar.history", "Histórico");
    bundle.add("sidebar.git", "Git");
    bundle.add("sidebar.files", "Arquivos");
    bundle.add("sidebar.search_projects", "Pesquisar projetos...");
    bundle.add("sidebar.no_projects", "Nenhum projeto");
    bundle.add("sidebar.add_project", "Adicionar projeto");
    bundle.add("sidebar.recent", "Recentes");
    bundle.add("sidebar.favorites", "Favoritos");

    // Chat
    bundle.add(
        "chat.placeholder",
        "Digite uma mensagem... (@ para mencionar arquivos)",
    );
    bundle.add("chat.send", "Enviar");
    bundle.add("chat.thinking", "Claude está pensando...");
    bundle.add("chat.stop", "Parar");
    bundle.add("chat.new_conversation", "Nova conversa");
    bundle.add("chat.export", "Exportar");
    bundle.add("chat.clear", "Limpar");
    bundle.add("chat.you", "Você");
    bundle.add("chat.assistant", "Claude");

    // Settings
    bundle.add("settings.title", "Configurações");
    bundle.add("settings.appearance", "Aparência");
    bundle.add("settings.editor", "Editor");
    bundle.add("settings.git", "Git");
    bundle.add("settings.claude", "Claude");
    bundle.add("settings.accessibility", "Acessibilidade");
    bundle.add("settings.language", "Idioma");
    bundle.add("settings.theme", "Tema");
    bundle.add("settings.theme_dark", "Escuro");
    bundle.add("settings.theme_light", "Claro");
    bundle.add("settings.theme_hc_dark", "Alto contraste escuro");
    bundle.add("settings.theme_hc_light", "Alto contraste claro");
    bundle.add("settings.font_size", "Tamanho da fonte");
    bundle.add("settings.font_family", "Família da fonte");
    bundle.add("settings.sidebar_width", "Largura da barra lateral");
    bundle.add("settings.vim_mode", "Modo Vim");
    bundle.add("settings.reduce_motion", "Reduzir movimento");
    bundle.add("settings.high_contrast", "Alto contraste");

    // Tabs
    bundle.add("tabs.new_tab", "Nova aba");
    bundle.add("tabs.close_tab", "Fechar aba");
    bundle.add("tabs.close_all", "Fechar todas");
    bundle.add("tabs.untitled", "Sem título");

    // Code blocks
    bundle.add("code.copy", "Copiar código");
    bundle.add("code.copied", "Copiado!");
    bundle.add("code.run", "Executar");
    bundle.add("code.save", "Salvar");
    bundle.add("code.lines", "{count} linhas");
    bundle.add("code.collapse", "Recolher");
    bundle.add("code.expand", "Expandir");

    // Diff
    bundle.add("diff.additions", "+{count}");
    bundle.add("diff.deletions", "-{count}");
    bundle.add("diff.no_changes", "Sem alterações");

    // File explorer
    bundle.add("explorer.new_file", "Novo arquivo");
    bundle.add("explorer.new_folder", "Nova pasta");
    bundle.add("explorer.rename", "Renomear");
    bundle.add("explorer.add_to_context", "Adicionar ao contexto");
    bundle.add("explorer.open_in_terminal", "Abrir no terminal");

    // Git
    bundle.add("git.branch", "Branch");
    bundle.add("git.commit", "Commit");
    bundle.add("git.push", "Push");
    bundle.add("git.pull", "Pull");
    bundle.add("git.status", "Status");
    bundle.add("git.staged", "Preparado");
    bundle.add("git.unstaged", "Não preparado");
    bundle.add("git.untracked", "Não rastreado");
    bundle.add("git.modified", "Modificado");
    bundle.add("git.added", "Adicionado");
    bundle.add("git.deleted", "Excluído");

    // Terminal
    bundle.add("terminal.title", "Terminal");
    bundle.add("terminal.clear", "Limpar");

    // Agent
    bundle.add("agent.title", "Modo Agente");
    bundle.add("agent.planning", "Planejando...");
    bundle.add("agent.executing", "Executando...");
    bundle.add("agent.paused", "Pausado");
    bundle.add("agent.completed", "Concluído");
    bundle.add("agent.failed", "Falhou");
    bundle.add("agent.approve", "Aprovar");
    bundle.add("agent.reject", "Rejeitar");
    bundle.add("agent.pause", "Pausar");
    bundle.add("agent.resume", "Retomar");
    bundle.add("agent.cancel", "Cancelar");

    // MCP
    bundle.add("mcp.servers", "Servidores MCP");
    bundle.add("mcp.tools", "Ferramentas");
    bundle.add("mcp.resources", "Recursos");
    bundle.add("mcp.prompts", "Prompts");
    bundle.add("mcp.connected", "Conectado");
    bundle.add("mcp.disconnected", "Desconectado");
    bundle.add("mcp.connecting", "Conectando...");

    // Keyboard shortcuts
    bundle.add("shortcuts.title", "Atalhos de teclado");
    bundle.add("shortcuts.general", "Geral");
    bundle.add("shortcuts.navigation", "Navegação");
    bundle.add("shortcuts.editing", "Edição");

    // Accessibility
    bundle.add("a11y.skip_to_main", "Pular para conteúdo principal");
    bundle.add("a11y.skip_to_chat", "Pular para o chat");
    bundle.add("a11y.skip_to_nav", "Pular para navegação");
    bundle.add("a11y.dialog_opened", "Diálogo aberto: {title}");
    bundle.add("a11y.dialog_closed", "Diálogo fechado");
    bundle.add("a11y.loading", "Carregando");
    bundle.add("a11y.message_sent", "Mensagem enviada");
    bundle.add("a11y.response_complete", "Resposta concluída");

    bundle
}
