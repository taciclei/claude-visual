//! FAQ components with questions and answers

use gpui::prelude::*;
use gpui::*;

/// FAQ item with question and answer
#[derive(IntoElement)]
pub struct FaqItem {
    question: SharedString,
    answer: SharedString,
    open: bool,
    question_color: Option<Hsla>,
    answer_color: Option<Hsla>,
}

impl FaqItem {
    pub fn new(question: impl Into<SharedString>, answer: impl Into<SharedString>) -> Self {
        Self {
            question: question.into(),
            answer: answer.into(),
            open: false,
            question_color: None,
            answer_color: None,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn question_color(mut self, color: Hsla) -> Self {
        self.question_color = Some(color);
        self
    }

    pub fn answer_color(mut self, color: Hsla) -> Self {
        self.answer_color = Some(color);
        self
    }
}

impl RenderOnce for FaqItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let question_color = self.question_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.95,
            a: 1.0,
        });
        let answer_color = self.answer_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.7,
            a: 1.0,
        });

        let chevron = if self.open { "−" } else { "+" };

        let question = div()
            .flex()
            .items_center()
            .justify_between()
            .py_3()
            .cursor_pointer()
            .child(
                div()
                    .text_size(px(15.0))
                    .text_color(question_color)
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(self.question),
            )
            .child(
                div()
                    .text_size(px(18.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.5,
                        a: 1.0,
                    })
                    .child(chevron),
            );

        let answer = if self.open {
            div()
                .pb_3()
                .text_size(px(14.0))
                .text_color(answer_color)
                .line_height(px(22.0))
                .child(self.answer)
        } else {
            div()
        };

        div()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.2,
                a: 1.0,
            })
            .child(question)
            .child(answer)
    }
}

/// FAQ section with multiple items
#[derive(IntoElement)]
pub struct FaqSection {
    title: Option<SharedString>,
    items: Vec<FaqItem>,
    title_color: Option<Hsla>,
}

impl FaqSection {
    pub fn new() -> Self {
        Self {
            title: None,
            items: Vec::new(),
            title_color: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn item(mut self, item: FaqItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = FaqItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn title_color(mut self, color: Hsla) -> Self {
        self.title_color = Some(color);
        self
    }
}

impl Default for FaqSection {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FaqSection {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let title_color = self.title_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.95,
            a: 1.0,
        });

        let mut container = div().flex().flex_col().w_full();

        if let Some(title) = self.title {
            container = container.child(
                div()
                    .text_size(px(24.0))
                    .text_color(title_color)
                    .font_weight(gpui::FontWeight::BOLD)
                    .mb_4()
                    .child(title),
            );
        }

        for item in self.items {
            container = container.child(item);
        }

        container
    }
}
