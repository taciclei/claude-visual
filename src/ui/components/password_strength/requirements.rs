//! Password requirements checklist component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Password requirements checklist
#[derive(IntoElement)]
pub struct PasswordRequirements {
    id: ElementId,
    pub(crate) requirements: Vec<PasswordRequirement>,
    show_all: bool,
}

impl PasswordRequirements {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            requirements: Vec::new(),
            show_all: true,
        }
    }

    pub fn requirements(mut self, requirements: Vec<PasswordRequirement>) -> Self {
        self.requirements = requirements;
        self
    }

    pub fn from_password(mut self, password: &str) -> Self {
        self.requirements = vec![
            PasswordRequirement::new("At least 8 characters", password.len() >= 8),
            PasswordRequirement::new(
                "Contains uppercase letter",
                password.chars().any(|c| c.is_uppercase()),
            ),
            PasswordRequirement::new(
                "Contains lowercase letter",
                password.chars().any(|c| c.is_lowercase()),
            ),
            PasswordRequirement::new(
                "Contains a number",
                password.chars().any(|c| c.is_numeric()),
            ),
            PasswordRequirement::new(
                "Contains special character",
                password.chars().any(|c| !c.is_alphanumeric()),
            ),
        ];
        self
    }

    pub fn show_all(mut self, show: bool) -> Self {
        self.show_all = show;
        self
    }
}

impl RenderOnce for PasswordRequirements {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let requirements: Vec<_> = if self.show_all {
            self.requirements
        } else {
            self.requirements.into_iter().filter(|r| !r.met).collect()
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(6.0))
            .children(requirements.iter().map(|req| {
                let (icon, color) = if req.met {
                    ("✓", hsla(0.35, 0.7, 0.45, 1.0))
                } else {
                    ("○", hsla(0.0, 0.0, 0.5, 1.0))
                };

                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(16.0))
                            .text_size(px(12.0))
                            .text_color(color)
                            .child(icon),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(if req.met {
                                hsla(0.0, 0.0, 0.6, 1.0)
                            } else {
                                hsla(0.0, 0.0, 0.8, 1.0)
                            })
                            .child(req.label.clone()),
                    )
            }))
    }
}
