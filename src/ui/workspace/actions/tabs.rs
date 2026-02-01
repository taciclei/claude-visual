//! Tab action handlers

use super::super::core::Workspace;
use crate::{CloseTab, NewTab, NextTab, PrevTab};
use gpui::*;

impl Workspace {
    /// Handle new tab action
    pub(in crate::ui::workspace) fn handle_new_tab(
        &mut self,
        _: &NewTab,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.tab_bar.update(cx, |bar, cx| {
            bar.add_tab(cx);
        });
    }

    /// Handle close tab action
    pub(in crate::ui::workspace) fn handle_close_tab(
        &mut self,
        _: &CloseTab,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let index = self.active_chat_index;
        self.tab_bar.update(cx, |bar, cx| {
            bar.close_tab(index, cx);
        });
    }

    /// Handle next tab action
    pub(in crate::ui::workspace) fn handle_next_tab(
        &mut self,
        _: &NextTab,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.tab_bar.update(cx, |bar, cx| {
            bar.select_next_tab(cx);
        });
    }

    /// Handle previous tab action
    pub(in crate::ui::workspace) fn handle_prev_tab(
        &mut self,
        _: &PrevTab,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.tab_bar.update(cx, |bar, cx| {
            bar.select_prev_tab(cx);
        });
    }

    /// Handle select tab by number
    pub(in crate::ui::workspace) fn handle_select_tab(
        &mut self,
        num: usize,
        cx: &mut Context<Self>,
    ) {
        self.tab_bar.update(cx, |bar, cx| {
            bar.select_tab_by_number(num, cx);
        });
    }
}
