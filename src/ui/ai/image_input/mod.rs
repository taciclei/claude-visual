//! Image Input Component
//!
//! Handles image drag & drop and attachment for AI context.

mod attachment;
mod colors;
mod events;

pub use attachment::ImageAttachment;
pub use events::ImageInputEvent;

use colors::default_colors;
use gpui::*;
use gpui::prelude::*;
use std::path::PathBuf;

/// Image input component with drag & drop
pub struct ImageInput {
    /// Attached images
    images: Vec<ImageAttachment>,
    /// Whether currently dragging over
    is_drag_over: bool,
    /// Maximum number of images
    max_images: usize,
    /// Maximum total size
    max_total_size: usize,
}

impl ImageInput {
    /// Create a new image input
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            images: Vec::new(),
            is_drag_over: false,
            max_images: 5,
            max_total_size: 50_000_000, // 50MB total
        }
    }

    /// Add images from paths
    pub fn add_images(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        let mut new_images = Vec::new();

        for path in paths {
            if self.images.len() + new_images.len() >= self.max_images {
                tracing::warn!("Maximum image count reached");
                break;
            }

            if let Some(attachment) = ImageAttachment::from_path(path) {
                if attachment.is_too_large() {
                    tracing::warn!("Image too large: {}", attachment.name);
                    continue;
                }
                new_images.push(attachment);
            }
        }

        if !new_images.is_empty() {
            cx.emit(ImageInputEvent::ImagesAttached(new_images.clone()));
            self.images.extend(new_images);
            cx.notify();
        }
    }

    /// Remove image at index
    pub fn remove_image(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.images.len() {
            self.images.remove(index);
            cx.emit(ImageInputEvent::ImageRemoved(index));
            cx.notify();
        }
    }

    /// Clear all images
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.images.clear();
        cx.emit(ImageInputEvent::ClearImages);
        cx.notify();
    }

    /// Get all images
    pub fn images(&self) -> &[ImageAttachment] {
        &self.images
    }

    /// Get total size
    pub fn total_size(&self) -> usize {
        self.images.iter().map(|i| i.size).sum()
    }

    /// Check if can add more images
    pub fn can_add(&self) -> bool {
        self.images.len() < self.max_images && self.total_size() < self.max_total_size
    }
}

impl EventEmitter<ImageInputEvent> for ImageInput {}

impl Render for ImageInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = default_colors();
        let has_images = !self.images.is_empty();
        let is_drag_over = self.is_drag_over;
        let can_add = self.can_add();

        // Copy colors for move closures
        let error_color = colors.error;
        let border_color = colors.border;
        let surface_color = colors.surface;
        let editor_background_color = colors.editor_background;
        let text_muted_color = colors.text_muted;
        let accent_color = colors.accent;
        let error_hover = colors.error.opacity(0.8);
        let accent_bg = colors.accent.opacity(0.1);

        // Extract listeners before div chains
        let on_drop_listener = cx.listener(|this, paths: &ExternalPaths, _window, cx| {
            this.is_drag_over = false;
            let image_paths: Vec<_> = paths
                .paths()
                .iter()
                .filter(|p| {
                    p.extension()
                        .map(|e| {
                            matches!(
                                e.to_string_lossy().to_lowercase().as_str(),
                                "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg"
                            )
                        })
                        .unwrap_or(false)
                })
                .cloned()
                .collect();
            this.add_images(image_paths, cx);
        });

        let on_drag_move_listener = cx.listener(|this, _, _window, cx| {
            this.is_drag_over = true;
            cx.notify();
        });

        div()
            .flex()
            .flex_col()
            .gap_2()
            // Image preview grid (if any images)
            .when(has_images, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_2()
                        .children(self.images.iter().enumerate().map(|(index, image)| {
                            let name = image.name.clone();
                            let size = image.format_size();
                            let is_too_large = image.is_too_large();

                            let on_remove_click = cx.listener(move |this, _, _window, cx| {
                                this.remove_image(index, cx);
                            });

                            div()
                                .id(ElementId::Name(format!("image-{}", index).into()))
                                .relative()
                                .w(px(80.0))
                                .h(px(80.0))
                                .rounded_md()
                                .overflow_hidden()
                                .border_1()
                                .border_color(if is_too_large {
                                    error_color
                                } else {
                                    border_color
                                })
                                .bg(surface_color)
                                .child(
                                    div()
                                        .size_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .bg(editor_background_color)
                                        .child(
                                            div()
                                                .text_2xl()
                                                .text_color(text_muted_color)
                                                .child("🖼"),
                                        ),
                                )
                                .child(
                                    div()
                                        .absolute()
                                        .bottom_0()
                                        .left_0()
                                        .right_0()
                                        .bg(hsla(0.0, 0.0, 0.0, 0.7))
                                        .px_1()
                                        .py_0p5()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                                        .text_ellipsis()
                                                        .child(name),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                                        .child(size),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .id(ElementId::Name(format!("remove-image-{}", index).into()))
                                        .absolute()
                                        .top_1()
                                        .right_1()
                                        .w(px(18.0))
                                        .h(px(18.0))
                                        .rounded_full()
                                        .bg(error_color)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .cursor_pointer()
                                        .hover(|style| style.bg(error_hover))
                                        .on_click(on_remove_click)
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                                .child("×"),
                                        ),
                                )
                        })),
                )
            })
            // Drop zone
            .when(can_add, |this| {
                this.child(
                    div()
                        .id("image-drop-zone")
                        .w_full()
                        .h(px(60.0))
                        .rounded_md()
                        .border_2()
                        .border_color(if is_drag_over {
                            accent_color
                        } else {
                            border_color
                        })
                        .border_dashed()
                        .bg(if is_drag_over {
                            accent_bg
                        } else {
                            surface_color
                        })
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .on_drop(on_drop_listener)
                        .on_drag_move::<ExternalPaths>(on_drag_move_listener)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_lg()
                                        .text_color(text_muted_color)
                                        .child(if is_drag_over { "📥" } else { "🖼" }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted_color)
                                        .child(if is_drag_over {
                                            "Drop images here"
                                        } else {
                                            "Drag & drop images"
                                        }),
                                ),
                        ),
                )
            })
    }
}
