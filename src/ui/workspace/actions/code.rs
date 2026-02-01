//! Code action handlers

use gpui::*;
use crate::claude::message::ClaudeMessage;
use crate::ui::chat::view::ChatViewEvent;
use crate::ui::components::toast::Toast;
use crate::{ExecuteCodeAction, SaveCodeToFileAction, ExplainCodeAction, ImproveCodeAction, AddTestsAction, ReviewCodeAction, RefactorCodeAction};
use super::super::core::Workspace;

impl Workspace {
    /// Handle execute code action
    pub(in crate::ui::workspace) fn handle_execute_code(&mut self, action: &ExecuteCodeAction, _window: &mut Window, cx: &mut Context<Self>) {
        let code = action.code.clone();
        let cwd = self.app_state.current_directory();

        tracing::info!("Executing code: {}", &code[..code.len().min(50)]);

        // Add a message showing the execution
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |chat, cx| {
                chat.add_message(
                    ClaudeMessage::tool_use("bash".to_string(), serde_json::json!({ "command": &code })),
                    cx,
                );
            });
        }

        // Execute the command
        let active_index = self.active_chat_index;
        cx.spawn(async move |this, cx| {
            let output = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&code)
                .current_dir(cwd.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))))
                .output()
                .await;

            let result_message = match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let combined = if stderr.is_empty() {
                        stdout.to_string()
                    } else if stdout.is_empty() {
                        stderr.to_string()
                    } else {
                        format!("{}\n{}", stdout, stderr)
                    };

                    if output.status.success() {
                        ClaudeMessage::tool_result(combined, false)
                    } else {
                        ClaudeMessage::tool_result(combined, true)
                    }
                }
                Err(e) => ClaudeMessage::error(format!("Failed to execute: {}", e)),
            };

            let _ = this.update(cx, |workspace, cx| {
                if let Some(chat_view) = workspace.chat_views.get(active_index) {
                    chat_view.update(cx, |chat, cx| {
                        chat.add_message(result_message, cx);
                    });
                }
            });
        }).detach();
    }

    /// Handle save code to file action
    pub(in crate::ui::workspace) fn handle_save_code_to_file(&mut self, action: &SaveCodeToFileAction, _window: &mut Window, cx: &mut Context<Self>) {
        let code = action.code.clone();

        cx.background_executor().spawn(async move {
            let file = rfd::AsyncFileDialog::new()
                .set_title("Save Code")
                .save_file()
                .await;

            if let Some(file) = file {
                let path = file.path();
                match std::fs::write(path, &code) {
                    Ok(_) => tracing::info!("Code saved to {:?}", path),
                    Err(e) => tracing::error!("Failed to save code: {}", e),
                }
            }
        }).detach();
    }

    /// Handle explain code action - sends code to Claude for explanation
    pub(in crate::ui::workspace) fn handle_explain_code(&mut self, action: &ExplainCodeAction, _window: &mut Window, cx: &mut Context<Self>) {
        let code = &action.code;
        let lang = action.language.as_deref().unwrap_or("code");

        // Build a prompt asking Claude to explain the code
        let prompt = format!(
            "Please explain this {} code in detail:\n\n```{}\n{}\n```\n\nExplain what it does, how it works, and any important concepts or patterns used.",
            lang, lang, code
        );

        // Send to current chat view
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                cx.emit(ChatViewEvent::Submit(prompt));
            });
        }
        self.show_toast(Toast::info("Explaining code..."), cx);
    }

    /// Handle improve code action - sends code to Claude for improvement
    pub(in crate::ui::workspace) fn handle_improve_code(&mut self, action: &ImproveCodeAction, _window: &mut Window, cx: &mut Context<Self>) {
        let code = &action.code;
        let lang = action.language.as_deref().unwrap_or("code");

        // Build a prompt asking Claude to improve the code
        let prompt = format!(
            "Please improve and refactor this {} code:\n\n```{}\n{}\n```\n\nFocus on:\n- Code quality and readability\n- Performance optimizations\n- Best practices and patterns\n- Error handling improvements\n\nProvide the improved code with explanations of the changes.",
            lang, lang, code
        );

        // Send to current chat view
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                cx.emit(ChatViewEvent::Submit(prompt));
            });
        }
        self.show_toast(Toast::info("Improving code..."), cx);
    }

    /// Handle add tests action - sends code to Claude for test generation
    pub(in crate::ui::workspace) fn handle_add_tests(&mut self, action: &AddTestsAction, _window: &mut Window, cx: &mut Context<Self>) {
        let code = &action.code;
        let lang = action.language.as_deref().unwrap_or("code");

        // Build a prompt asking Claude to add tests
        let prompt = format!(
            "Please write comprehensive tests for this {} code:\n\n```{}\n{}\n```\n\nInclude:\n- Unit tests for each function/method\n- Edge case testing\n- Error handling tests\n- Descriptive test names\n\nUse the appropriate testing framework for {}.",
            lang, lang, code, lang
        );

        // Send to current chat view
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                cx.emit(ChatViewEvent::Submit(prompt));
            });
        }
        self.show_toast(Toast::info("Generating tests..."), cx);
    }

    /// Handle review code action - sends code to Claude for code review using /review skill
    pub(in crate::ui::workspace) fn handle_review_code(&mut self, action: &ReviewCodeAction, _window: &mut Window, cx: &mut Context<Self>) {
        let code = &action.code;
        let lang = action.language.as_deref().unwrap_or("code");

        // Build a prompt for code review using /review skill approach
        let prompt = format!(
            "/review\n\nPlease review this {} code for:\n\n```{}\n{}\n```\n\nFocus on:\n- Security vulnerabilities (OWASP top 10)\n- Logic errors and edge cases\n- Clean code principles (SOLID, DRY, naming)\n- Performance concerns\n- Error handling gaps",
            lang, lang, code
        );

        // Send to current chat view
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                cx.emit(ChatViewEvent::Submit(prompt));
            });
        }
        self.show_toast(Toast::info("Reviewing code..."), cx);
    }

    /// Handle refactor code action - sends code to Claude for refactoring using /refactor skill
    pub(in crate::ui::workspace) fn handle_refactor_code(&mut self, action: &RefactorCodeAction, _window: &mut Window, cx: &mut Context<Self>) {
        let code = &action.code;
        let lang = action.language.as_deref().unwrap_or("code");

        // Build a prompt for code refactoring
        let prompt = format!(
            "/refactor\n\nPlease refactor this {} code:\n\n```{}\n{}\n```\n\nApply:\n- Clean code principles\n- Design patterns where appropriate\n- Better abstraction and modularity\n- Improved naming and structure\n- Performance optimizations",
            lang, lang, code
        );

        // Send to current chat view
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                cx.emit(ChatViewEvent::Submit(prompt));
            });
        }
        self.show_toast(Toast::info("Refactoring code..."), cx);
    }
}
