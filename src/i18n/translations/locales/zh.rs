//! Chinese translations

use super::super::bundle::TranslationBundle;
use crate::i18n::locale::Locale;

/// Chinese (Simplified) translations
pub fn chinese_bundle() -> TranslationBundle {
    let mut bundle = TranslationBundle::new(Locale::ZhCn);

    // App
    bundle.add("app.name", "Claude Visual");
    bundle.add("app.tagline", "Claude Code 可视化客户端");

    // Common
    bundle.add("common.ok", "确定");
    bundle.add("common.cancel", "取消");
    bundle.add("common.save", "保存");
    bundle.add("common.delete", "删除");
    bundle.add("common.edit", "编辑");
    bundle.add("common.close", "关闭");
    bundle.add("common.open", "打开");
    bundle.add("common.copy", "复制");
    bundle.add("common.paste", "粘贴");
    bundle.add("common.cut", "剪切");
    bundle.add("common.undo", "撤销");
    bundle.add("common.redo", "重做");
    bundle.add("common.search", "搜索");
    bundle.add("common.filter", "筛选");
    bundle.add("common.settings", "设置");
    bundle.add("common.help", "帮助");
    bundle.add("common.loading", "加载中...");
    bundle.add("common.error", "错误");
    bundle.add("common.success", "成功");
    bundle.add("common.warning", "警告");
    bundle.add("common.info", "信息");

    // Sidebar
    bundle.add("sidebar.projects", "项目");
    bundle.add("sidebar.history", "历史");
    bundle.add("sidebar.git", "Git");
    bundle.add("sidebar.files", "文件");
    bundle.add("sidebar.search_projects", "搜索项目...");
    bundle.add("sidebar.no_projects", "暂无项目");
    bundle.add("sidebar.add_project", "添加项目");
    bundle.add("sidebar.recent", "最近");
    bundle.add("sidebar.favorites", "收藏");

    // Chat
    bundle.add("chat.placeholder", "输入消息... (@提及文件)");
    bundle.add("chat.send", "发送");
    bundle.add("chat.thinking", "Claude 正在思考...");
    bundle.add("chat.stop", "停止");
    bundle.add("chat.new_conversation", "新对话");
    bundle.add("chat.export", "导出");
    bundle.add("chat.clear", "清空");
    bundle.add("chat.you", "你");
    bundle.add("chat.assistant", "Claude");

    // Settings
    bundle.add("settings.title", "设置");
    bundle.add("settings.appearance", "外观");
    bundle.add("settings.editor", "编辑器");
    bundle.add("settings.git", "Git");
    bundle.add("settings.claude", "Claude");
    bundle.add("settings.accessibility", "无障碍");
    bundle.add("settings.language", "语言");
    bundle.add("settings.theme", "主题");
    bundle.add("settings.theme_dark", "深色");
    bundle.add("settings.theme_light", "浅色");
    bundle.add("settings.theme_hc_dark", "高对比度深色");
    bundle.add("settings.theme_hc_light", "高对比度浅色");
    bundle.add("settings.font_size", "字体大小");
    bundle.add("settings.font_family", "字体");
    bundle.add("settings.sidebar_width", "侧边栏宽度");
    bundle.add("settings.vim_mode", "Vim 模式");
    bundle.add("settings.reduce_motion", "减少动画");
    bundle.add("settings.high_contrast", "高对比度");

    // Tabs
    bundle.add("tabs.new_tab", "新标签页");
    bundle.add("tabs.close_tab", "关闭标签页");
    bundle.add("tabs.close_all", "关闭全部");
    bundle.add("tabs.untitled", "未命名");

    // Code blocks
    bundle.add("code.copy", "复制代码");
    bundle.add("code.copied", "已复制！");
    bundle.add("code.run", "运行");
    bundle.add("code.save", "保存");
    bundle.add("code.lines", "{count} 行");
    bundle.add("code.collapse", "折叠");
    bundle.add("code.expand", "展开");

    // Diff
    bundle.add("diff.additions", "+{count}");
    bundle.add("diff.deletions", "-{count}");
    bundle.add("diff.no_changes", "无更改");

    // File explorer
    bundle.add("explorer.new_file", "新建文件");
    bundle.add("explorer.new_folder", "新建文件夹");
    bundle.add("explorer.rename", "重命名");
    bundle.add("explorer.add_to_context", "添加到上下文");
    bundle.add("explorer.open_in_terminal", "在终端中打开");

    // Git
    bundle.add("git.branch", "分支");
    bundle.add("git.commit", "提交");
    bundle.add("git.push", "推送");
    bundle.add("git.pull", "拉取");
    bundle.add("git.status", "状态");
    bundle.add("git.staged", "已暂存");
    bundle.add("git.unstaged", "未暂存");
    bundle.add("git.untracked", "未跟踪");
    bundle.add("git.modified", "已修改");
    bundle.add("git.added", "已添加");
    bundle.add("git.deleted", "已删除");

    // Terminal
    bundle.add("terminal.title", "终端");
    bundle.add("terminal.clear", "清空");

    // Agent
    bundle.add("agent.title", "代理模式");
    bundle.add("agent.planning", "规划中...");
    bundle.add("agent.executing", "执行中...");
    bundle.add("agent.paused", "已暂停");
    bundle.add("agent.completed", "已完成");
    bundle.add("agent.failed", "失败");
    bundle.add("agent.approve", "批准");
    bundle.add("agent.reject", "拒绝");
    bundle.add("agent.pause", "暂停");
    bundle.add("agent.resume", "继续");
    bundle.add("agent.cancel", "取消");

    // MCP
    bundle.add("mcp.servers", "MCP 服务器");
    bundle.add("mcp.tools", "工具");
    bundle.add("mcp.resources", "资源");
    bundle.add("mcp.prompts", "提示词");
    bundle.add("mcp.connected", "已连接");
    bundle.add("mcp.disconnected", "已断开");
    bundle.add("mcp.connecting", "连接中...");

    // Keyboard shortcuts
    bundle.add("shortcuts.title", "快捷键");
    bundle.add("shortcuts.general", "通用");
    bundle.add("shortcuts.navigation", "导航");
    bundle.add("shortcuts.editing", "编辑");

    // Accessibility
    bundle.add("a11y.skip_to_main", "跳转到主内容");
    bundle.add("a11y.skip_to_chat", "跳转到聊天输入");
    bundle.add("a11y.skip_to_nav", "跳转到导航");
    bundle.add("a11y.dialog_opened", "对话框已打开：{title}");
    bundle.add("a11y.dialog_closed", "对话框已关闭");
    bundle.add("a11y.loading", "加载中");
    bundle.add("a11y.message_sent", "消息已发送");
    bundle.add("a11y.response_complete", "响应完成");

    bundle
}
