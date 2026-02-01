use super::form_field::FormField;
use gpui::prelude::*;
use gpui::*;

/// Field group with title
#[derive(IntoElement)]
pub struct FieldGroup {
    title: Option<SharedString>,
    description: Option<SharedString>,
    fields: Vec<FormField>,
    gap: f32,
    title_color: Option<Hsla>,
}

impl FieldGroup {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            fields: Vec::new(),
            gap: 16.0,
            title_color: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn field(mut self, field: FormField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn fields(mut self, fields: impl IntoIterator<Item = FormField>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn title_color(mut self, color: Hsla) -> Self {
        self.title_color = Some(color);
        self
    }
}

impl Default for FieldGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FieldGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let title_color = self.title_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.95,
            a: 1.0,
        });

        let mut container = div().flex().flex_col();

        if self.title.is_some() || self.description.is_some() {
            let mut header = div().flex().flex_col().gap_1().mb_4();

            if let Some(title) = self.title {
                header = header.child(
                    div()
                        .text_size(px(16.0))
                        .text_color(title_color)
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child(title),
                );
            }

            if let Some(description) = self.description {
                header = header.child(
                    div()
                        .text_size(px(13.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child(description),
                );
            }

            container = container.child(header);
        }

        let mut fields_container = div().flex().flex_col().gap(px(self.gap));
        for field in self.fields {
            fields_container = fields_container.child(field);
        }

        container.child(fields_container)
    }
}
