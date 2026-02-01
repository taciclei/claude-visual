//! File list rendering for diagnostics panel

use gpui::prelude::*;
use gpui::*;

use crate::ui::lsp::diagnostics::core::DiagnosticsPanel;
use crate::ui::lsp::diagnostics::types::SimpleColors;

use super::diagnostic_item::render_diagnostic_items;
use super::empty_state::render_empty_state;

pub fn render_file_list(
    panel: &DiagnosticsPanel,
    cx: &mut Context<DiagnosticsPanel>,
    colors: &SimpleColors,
) -> impl IntoElement {
    let colors_hover = colors.hover;
    let colors_text = colors.text;
    let colors_text_muted = colors.text_muted;
    let colors_error = colors.error;
    let colors_warning = colors.warning;

    div()
        .flex()
        .flex_col()
        .max_h(px(300.0))
        .id("scroll-diagnostics-panel")
        .overflow_y_scroll()
        .children(panel.file_order.iter().filter_map(|path| {
            let file = panel.files.get(path)?;
            let filtered = panel.filtered_diagnostics(file);
            if filtered.is_empty() {
                return None;
            }

            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());
            let error_count = file.error_count();
            let warning_count = file.warning_count();
            let is_expanded = file.is_expanded;
            let path_clone = path.clone();

            let toggle_file_click = {
                let path = path.clone();
                cx.listener(move |this, _, _window, cx| {
                    this.toggle_file(&path, cx);
                })
            };

            Some(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .id(ElementId::Name(format!("file-{}", path.display()).into()))
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_1()
                            .cursor_pointer()
                            .hover(|style| style.bg(colors_hover))
                            .on_click(toggle_file_click)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors_text_muted)
                                    .child(if is_expanded { "▼" } else { "▶" }),
                            )
                            .child(div().text_sm().text_color(colors_text).child(file_name))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .when(error_count > 0, |this| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(colors_error)
                                                .child(format!("{} errors", error_count)),
                                        )
                                    })
                                    .when(warning_count > 0, |this| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(colors_warning)
                                                .child(format!("{} warnings", warning_count)),
                                        )
                                    }),
                            ),
                    )
                    .when(is_expanded, |this| {
                        this.child(render_diagnostic_items(&filtered, &path_clone, cx, colors))
                    }),
            )
        }))
        .when(panel.files.is_empty(), |this| {
            this.child(render_empty_state(colors))
        })
}
