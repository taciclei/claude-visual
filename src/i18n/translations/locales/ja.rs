//! Japanese translations

use super::super::bundle::TranslationBundle;
use crate::i18n::locale::Locale;

/// Japanese translations
pub fn japanese_bundle() -> TranslationBundle {
    let mut bundle = TranslationBundle::new(Locale::JaJp);

    // App
    bundle.add("app.name", "Claude Visual");
    bundle.add("app.tagline", "Claude Code用ビジュアルクライアント");

    // Common
    bundle.add("common.ok", "OK");
    bundle.add("common.cancel", "キャンセル");
    bundle.add("common.save", "保存");
    bundle.add("common.delete", "削除");
    bundle.add("common.edit", "編集");
    bundle.add("common.close", "閉じる");
    bundle.add("common.open", "開く");
    bundle.add("common.copy", "コピー");
    bundle.add("common.paste", "貼り付け");
    bundle.add("common.cut", "切り取り");
    bundle.add("common.undo", "元に戻す");
    bundle.add("common.redo", "やり直し");
    bundle.add("common.search", "検索");
    bundle.add("common.filter", "フィルター");
    bundle.add("common.settings", "設定");
    bundle.add("common.help", "ヘルプ");
    bundle.add("common.loading", "読み込み中...");
    bundle.add("common.error", "エラー");
    bundle.add("common.success", "成功");
    bundle.add("common.warning", "警告");
    bundle.add("common.info", "情報");

    // Sidebar
    bundle.add("sidebar.projects", "プロジェクト");
    bundle.add("sidebar.history", "履歴");
    bundle.add("sidebar.git", "Git");
    bundle.add("sidebar.files", "ファイル");
    bundle.add("sidebar.search_projects", "プロジェクトを検索...");
    bundle.add("sidebar.no_projects", "プロジェクトがありません");
    bundle.add("sidebar.add_project", "プロジェクトを追加");
    bundle.add("sidebar.recent", "最近");
    bundle.add("sidebar.favorites", "お気に入り");

    // Chat
    bundle.add("chat.placeholder", "メッセージを入力... (@でファイルをメンション)");
    bundle.add("chat.send", "送信");
    bundle.add("chat.thinking", "Claudeが考え中...");
    bundle.add("chat.stop", "停止");
    bundle.add("chat.new_conversation", "新しい会話");
    bundle.add("chat.export", "エクスポート");
    bundle.add("chat.clear", "クリア");
    bundle.add("chat.you", "あなた");
    bundle.add("chat.assistant", "Claude");

    // Settings
    bundle.add("settings.title", "設定");
    bundle.add("settings.appearance", "外観");
    bundle.add("settings.editor", "エディター");
    bundle.add("settings.git", "Git");
    bundle.add("settings.claude", "Claude");
    bundle.add("settings.accessibility", "アクセシビリティ");
    bundle.add("settings.language", "言語");
    bundle.add("settings.theme", "テーマ");
    bundle.add("settings.theme_dark", "ダーク");
    bundle.add("settings.theme_light", "ライト");
    bundle.add("settings.theme_hc_dark", "ハイコントラスト（ダーク）");
    bundle.add("settings.theme_hc_light", "ハイコントラスト（ライト）");
    bundle.add("settings.font_size", "フォントサイズ");
    bundle.add("settings.font_family", "フォント");
    bundle.add("settings.sidebar_width", "サイドバーの幅");
    bundle.add("settings.vim_mode", "Vimモード");
    bundle.add("settings.reduce_motion", "モーションを減らす");
    bundle.add("settings.high_contrast", "ハイコントラスト");

    // Tabs
    bundle.add("tabs.new_tab", "新しいタブ");
    bundle.add("tabs.close_tab", "タブを閉じる");
    bundle.add("tabs.close_all", "すべて閉じる");
    bundle.add("tabs.untitled", "無題");

    // Code blocks
    bundle.add("code.copy", "コードをコピー");
    bundle.add("code.copied", "コピーしました！");
    bundle.add("code.run", "実行");
    bundle.add("code.save", "保存");
    bundle.add("code.lines", "{count}行");
    bundle.add("code.collapse", "折りたたむ");
    bundle.add("code.expand", "展開");

    // Diff
    bundle.add("diff.additions", "+{count}");
    bundle.add("diff.deletions", "-{count}");
    bundle.add("diff.no_changes", "変更なし");

    // File explorer
    bundle.add("explorer.new_file", "新規ファイル");
    bundle.add("explorer.new_folder", "新規フォルダー");
    bundle.add("explorer.rename", "名前を変更");
    bundle.add("explorer.add_to_context", "コンテキストに追加");
    bundle.add("explorer.open_in_terminal", "ターミナルで開く");

    // Git
    bundle.add("git.branch", "ブランチ");
    bundle.add("git.commit", "コミット");
    bundle.add("git.push", "プッシュ");
    bundle.add("git.pull", "プル");
    bundle.add("git.status", "ステータス");
    bundle.add("git.staged", "ステージ済み");
    bundle.add("git.unstaged", "未ステージ");
    bundle.add("git.untracked", "未追跡");
    bundle.add("git.modified", "変更済み");
    bundle.add("git.added", "追加済み");
    bundle.add("git.deleted", "削除済み");

    // Terminal
    bundle.add("terminal.title", "ターミナル");
    bundle.add("terminal.clear", "クリア");

    // Agent
    bundle.add("agent.title", "エージェントモード");
    bundle.add("agent.planning", "計画中...");
    bundle.add("agent.executing", "実行中...");
    bundle.add("agent.paused", "一時停止");
    bundle.add("agent.completed", "完了");
    bundle.add("agent.failed", "失敗");
    bundle.add("agent.approve", "承認");
    bundle.add("agent.reject", "拒否");
    bundle.add("agent.pause", "一時停止");
    bundle.add("agent.resume", "再開");
    bundle.add("agent.cancel", "キャンセル");

    // MCP
    bundle.add("mcp.servers", "MCPサーバー");
    bundle.add("mcp.tools", "ツール");
    bundle.add("mcp.resources", "リソース");
    bundle.add("mcp.prompts", "プロンプト");
    bundle.add("mcp.connected", "接続済み");
    bundle.add("mcp.disconnected", "切断");
    bundle.add("mcp.connecting", "接続中...");

    // Keyboard shortcuts
    bundle.add("shortcuts.title", "キーボードショートカット");
    bundle.add("shortcuts.general", "一般");
    bundle.add("shortcuts.navigation", "ナビゲーション");
    bundle.add("shortcuts.editing", "編集");

    // Accessibility
    bundle.add("a11y.skip_to_main", "メインコンテンツへスキップ");
    bundle.add("a11y.skip_to_chat", "チャット入力へスキップ");
    bundle.add("a11y.skip_to_nav", "ナビゲーションへスキップ");
    bundle.add("a11y.dialog_opened", "ダイアログが開きました: {title}");
    bundle.add("a11y.dialog_closed", "ダイアログが閉じました");
    bundle.add("a11y.loading", "読み込み中");
    bundle.add("a11y.message_sent", "メッセージを送信しました");
    bundle.add("a11y.response_complete", "応答が完了しました");

    bundle
}
