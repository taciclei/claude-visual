use gpui::*;
use gpui::prelude::*;

/// Description list label
#[derive(IntoElement)]
pub struct DescriptionLabel {
    term: SharedString,
    description: SharedString,
    term_color: Option<Hsla>,
    description_color: Option<Hsla>,
    inline: bool,
}

impl DescriptionLabel {
    pub fn new(term: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            term: term.into(),
            description: description.into(),
            term_color: None,
            description_color: None,
            inline: false,
        }
    }

    pub fn term_color(mut self, color: Hsla) -> Self {
        self.term_color = Some(color);
        self
    }

    pub fn description_color(mut self, color: Hsla) -> Self {
        self.description_color = Some(color);
        self
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
}

impl RenderOnce for DescriptionLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let term_color = self.term_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 1.0,
        });
        let description_color = self.description_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        });

        let term = div()
            .text_size(px(12.0))
            .text_color(term_color)
            .child(self.term);

        let description = div()
            .text_size(px(14.0))
            .text_color(description_color)
            .child(self.description);

        if self.inline {
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(term)
                .child(description)
        } else {
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(term)
                .child(description)
        }
    }
}
