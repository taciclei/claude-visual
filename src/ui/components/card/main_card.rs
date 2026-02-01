//! Main card component for content containers

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

/// Card component for content containers
pub struct Card {
    pub(crate) app_state: Arc<AppState>,
    /// Style variant
    pub(crate) variant: CardVariant,
    /// Padding size
    pub(crate) padding: CardPadding,
    /// Header title
    pub(crate) header_title: Option<String>,
    /// Header subtitle/description
    pub(crate) header_subtitle: Option<String>,
    /// Header icon
    pub(crate) header_icon: Option<String>,
    /// Footer content
    pub(crate) footer: Option<String>,
    /// Whether card is selected
    pub(crate) selected: bool,
    /// Whether card is disabled
    pub(crate) disabled: bool,
    /// Custom border radius
    pub(crate) border_radius: f32,
}

impl Card {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            variant: CardVariant::default(),
            padding: CardPadding::default(),
            header_title: None,
            header_subtitle: None,
            header_icon: None,
            footer: None,
            selected: false,
            disabled: false,
            border_radius: 8.0,
        }
    }

    /// Create a card with a header
    pub fn with_header(app_state: Arc<AppState>, title: impl Into<String>, cx: &mut Context<Self>) -> Self {
        let mut card = Self::new(app_state, cx);
        card.header_title = Some(title.into());
        card
    }

    /// Set the variant
    pub fn set_variant(&mut self, variant: CardVariant, cx: &mut Context<Self>) {
        self.variant = variant;
        cx.notify();
    }

    /// Set the padding
    pub fn set_padding(&mut self, padding: CardPadding, cx: &mut Context<Self>) {
        self.padding = padding;
        cx.notify();
    }

    /// Set the header title
    pub fn set_header_title(&mut self, title: Option<String>, cx: &mut Context<Self>) {
        self.header_title = title;
        cx.notify();
    }

    /// Set the header subtitle
    pub fn set_header_subtitle(&mut self, subtitle: Option<String>, cx: &mut Context<Self>) {
        self.header_subtitle = subtitle;
        cx.notify();
    }

    /// Set the header icon
    pub fn set_header_icon(&mut self, icon: Option<String>, cx: &mut Context<Self>) {
        self.header_icon = icon;
        cx.notify();
    }

    /// Set the footer
    pub fn set_footer(&mut self, footer: Option<String>, cx: &mut Context<Self>) {
        self.footer = footer;
        cx.notify();
    }

    /// Set selected state
    pub fn set_selected(&mut self, selected: bool, cx: &mut Context<Self>) {
        self.selected = selected;
        cx.notify();
    }

    /// Set disabled state
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set border radius
    pub fn set_border_radius(&mut self, radius: f32, cx: &mut Context<Self>) {
        self.border_radius = radius;
        cx.notify();
    }

    /// Check if card has a header
    fn has_header(&self) -> bool {
        self.header_title.is_some() || self.header_icon.is_some()
    }
}

impl EventEmitter<CardEvent> for Card {}

impl Render for Card {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let padding = self.padding.pixels();
        let is_interactive = matches!(self.variant, CardVariant::Interactive);

        // Determine styles based on variant
        let (bg_color, border_color, shadow) = match self.variant {
            CardVariant::Default => (
                theme.colors.surface,
                Some(theme.colors.border),
                false,
            ),
            CardVariant::Elevated => (
                theme.colors.surface,
                None,
                true,
            ),
            CardVariant::Outlined => (
                theme.colors.background,
                Some(theme.colors.border),
                false,
            ),
            CardVariant::Ghost => (
                gpui::transparent_black(),
                None,
                false,
            ),
            CardVariant::Interactive => (
                theme.colors.surface,
                Some(theme.colors.border),
                false,
            ),
        };

        // Selected state overrides
        let (final_bg, final_border) = if self.selected {
            (theme.colors.accent.opacity(0.1), Some(theme.colors.accent))
        } else {
            (bg_color, border_color)
        };

        // Disabled state
        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id("card")
            .w_full()
            .rounded(px(self.border_radius))
            .bg(final_bg)
            .when_some(final_border, |d, color| d.border_1().border_color(color))
            .when(shadow, |d| {
                // Simulate shadow with darker border
                d.border_1()
                    .border_color(theme.colors.border.opacity(0.5))
            })
            .opacity(opacity)
            .when(is_interactive && !self.disabled, |d| {
                d.cursor_pointer()
                    .hover(|s| s.bg(theme.colors.surface_hover).border_color(theme.colors.accent.opacity(0.5)))
            })
            .when(is_interactive && !self.disabled, |d| {
                d.on_click(cx.listener(|_this, _, _window, cx| {
                    cx.emit(CardEvent::Clicked);
                }))
            })
            .overflow_hidden()
            .flex()
            .flex_col()
            // Header
            .when(self.has_header(), |d| {
                d.child(
                    div()
                        .p(px(padding))
                        .when(self.has_header() && padding > 0.0, |d| d.border_b_1().border_color(theme.colors.border))
                        .flex()
                        .items_center()
                        .gap_3()
                        // Icon
                        .when_some(self.header_icon.clone(), |d, icon| {
                            d.child(
                                div()
                                    .size(px(32.0))
                                    .rounded(px(6.0))
                                    .bg(theme.colors.surface_hover)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.colors.text_muted)
                                    .child(icon)
                            )
                        })
                        // Title/Subtitle
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .when_some(self.header_title.clone(), |d, title| {
                                    d.child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child(title)
                                    )
                                })
                                .when_some(self.header_subtitle.clone(), |d, subtitle| {
                                    d.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(subtitle)
                                    )
                                })
                        )
                )
            })
            // Body (where children would go in a real implementation)
            .child(
                div()
                    .when(padding > 0.0, |d| d.p(px(padding)))
                    .flex_1()
            )
            // Footer
            .when_some(self.footer.clone(), |d, footer| {
                d.child(
                    div()
                        .p(px(padding))
                        .border_t_1()
                        .border_color(theme.colors.border)
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(footer)
                )
            })
    }
}
