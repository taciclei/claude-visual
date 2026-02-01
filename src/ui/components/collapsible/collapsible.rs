//! Basic collapsible section with trigger and content

use super::types::CollapsibleAnimation;
use gpui::prelude::*;
use gpui::*;

/// A basic collapsible section with trigger and content
#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    expanded: bool,
    trigger: Div,
    content: Div,
    animation: CollapsibleAnimation,
    disabled: bool,
    border: bool,
    border_color: Option<Hsla>,
    background: Option<Hsla>,
    padding: Option<f32>,
}

impl Collapsible {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            expanded: false,
            trigger: div(),
            content: div(),
            animation: CollapsibleAnimation::Default,
            disabled: false,
            border: false,
            border_color: None,
            background: None,
            padding: None,
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = div().child(trigger);
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = div().child(content);
        self
    }

    pub fn animation(mut self, animation: CollapsibleAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }
}

impl RenderOnce for Collapsible {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let border_color = self.border_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        });
        let padding = self.padding.unwrap_or(12.0);

        let mut container = div()
            .id(self.id)
            .flex()
            .flex_col()
            .w_full()
            .overflow_hidden();

        if self.border {
            container = container.border_1().border_color(border_color).rounded_lg();
        }

        if let Some(bg) = self.background {
            container = container.bg(bg);
        }

        // Trigger section
        let trigger = div()
            .p(px(padding))
            .cursor_pointer()
            .when(self.disabled, |d| d.opacity(0.5).cursor_not_allowed())
            .child(self.trigger);

        // Content section (only shown when expanded)
        let content = if self.expanded && !self.disabled {
            div().p(px(padding)).pt_0().child(self.content)
        } else {
            div()
        };

        container.child(trigger).child(content)
    }
}
