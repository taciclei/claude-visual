//! Action dispatch methods for code blocks

use gpui::*;

use crate::{AddTestsAction, ExecuteCodeAction, ExplainCodeAction, ImproveCodeAction, RefactorCodeAction, ReviewCodeAction, SaveCodeToFileAction};

use super::CodeBlockView;

impl CodeBlockView {
    /// Request to execute the code
    pub fn execute(&self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.is_executable() {
            let action = ExecuteCodeAction { code: self.code.clone() };
            cx.dispatch_action(&action);
            tracing::info!("Dispatched code execution action");
        }
    }

    /// Request to save the code to a file
    pub fn save_to_file(&self, _window: &mut Window, cx: &mut Context<Self>) {
        let action = SaveCodeToFileAction { code: self.code.clone() };
        cx.dispatch_action(&action);
        tracing::info!("Dispatched save to file action");
    }

    /// Request Claude to explain this code
    pub fn explain_code(&self, cx: &mut Context<Self>) {
        let action = ExplainCodeAction {
            code: self.code.clone(),
            language: self.language.clone(),
        };
        cx.dispatch_action(&action);
        tracing::info!("Dispatched explain code action");
    }

    /// Request Claude to improve/refactor this code
    pub fn improve_code(&self, cx: &mut Context<Self>) {
        let action = ImproveCodeAction {
            code: self.code.clone(),
            language: self.language.clone(),
        };
        cx.dispatch_action(&action);
        tracing::info!("Dispatched improve code action");
    }

    /// Request Claude to add tests for this code
    pub fn add_tests(&self, cx: &mut Context<Self>) {
        let action = AddTestsAction {
            code: self.code.clone(),
            language: self.language.clone(),
        };
        cx.dispatch_action(&action);
        tracing::info!("Dispatched add tests action");
    }

    /// Request Claude to review this code (uses /review skill)
    pub fn review_code(&self, cx: &mut Context<Self>) {
        let action = ReviewCodeAction {
            code: self.code.clone(),
            language: self.language.clone(),
        };
        cx.dispatch_action(&action);
        tracing::info!("Dispatched review code action");
    }

    /// Request Claude to refactor this code (uses /refactor skill)
    pub fn refactor_code(&self, cx: &mut Context<Self>) {
        let action = RefactorCodeAction {
            code: self.code.clone(),
            language: self.language.clone(),
        };
        cx.dispatch_action(&action);
        tracing::info!("Dispatched refactor code action");
    }
}
