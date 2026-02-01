//! Splash screen component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

impl EventEmitter<SplashEvent> for SplashScreen {}

/// Splash screen component
pub struct SplashScreen {
    /// Current startup phase
    phase: StartupPhase,
    /// Error message if startup failed
    error: Option<String>,
    /// Animation frame for spinner
    animation_frame: usize,
    /// Version string
    version: String,
}

impl SplashScreen {
    /// Create a new splash screen
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            phase: StartupPhase::Initializing,
            error: None,
            animation_frame: 0,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Update the current phase
    pub fn set_phase(&mut self, phase: StartupPhase, cx: &mut Context<Self>) {
        self.phase = phase;
        if phase == StartupPhase::Ready {
            cx.emit(SplashEvent::Complete);
        }
        cx.notify();
    }

    /// Advance to the next phase
    pub fn advance(&mut self, cx: &mut Context<Self>) {
        if let Some(next) = self.phase.next() {
            self.set_phase(next, cx);
        }
    }

    /// Set an error
    pub fn set_error(&mut self, error: String, cx: &mut Context<Self>) {
        self.error = Some(error.clone());
        cx.emit(SplashEvent::Error(error));
        cx.notify();
    }

    /// Check if startup is complete
    pub fn is_complete(&self) -> bool {
        self.phase == StartupPhase::Ready
    }

    /// Check if there's an error
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// Tick animation
    pub fn tick_animation(&mut self, cx: &mut Context<Self>) {
        self.animation_frame = (self.animation_frame + 1) % 8;
        cx.notify();
    }

    /// Get spinner character
    fn spinner_char(&self) -> &'static str {
        const SPINNER: [&str; 8] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
        SPINNER[self.animation_frame]
    }
}

impl Render for SplashScreen {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let progress = self.phase.progress();
        let status_text = self.phase.display_text();
        let spinner = self.spinner_char();
        let version = self.version.clone();
        let error = self.error.clone();

        // Colors (dark theme defaults for splash)
        let bg_color = rgb(0x1a1a2e);
        let surface_color = rgb(0x16213e);
        let text_color = rgb(0xe4e4e7);
        let muted_color = rgb(0x71717a);
        let accent_color = rgb(0x8b5cf6);
        let error_color = rgb(0xef4444);

        div()
            .w_full()
            .h_full()
            .bg(bg_color)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            // Logo / App name
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    // App icon placeholder
                    .child(
                        div()
                            .w(px(80.0))
                            .h(px(80.0))
                            .rounded(px(16.0))
                            .bg(surface_color)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().text_3xl().text_color(accent_color).child("C")),
                    )
                    // App name
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(text_color)
                            .child("Claude Visual"),
                    )
                    // Version
                    .child(
                        div()
                            .text_sm()
                            .text_color(muted_color)
                            .child(format!("v{}", version)),
                    ),
            )
            // Progress section
            .child(
                div()
                    .w(px(300.0))
                    .flex()
                    .flex_col()
                    .gap_3()
                    // Progress bar
                    .child(
                        div()
                            .w_full()
                            .h(px(4.0))
                            .rounded_full()
                            .bg(surface_color)
                            .child(
                                div()
                                    .h_full()
                                    .rounded_full()
                                    .bg(accent_color)
                                    .w(relative(progress)),
                            ),
                    )
                    // Status text
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .when(error.is_none(), |d| {
                                d.child(
                                    div()
                                        .text_sm()
                                        .text_color(accent_color)
                                        .child(spinner.to_string()),
                                )
                            })
                            .child(
                                div()
                                    .text_sm()
                                    .when(error.is_none(), |d| d.text_color(muted_color))
                                    .when(error.is_some(), |d| d.text_color(error_color))
                                    .child(if let Some(err) = error {
                                        err
                                    } else {
                                        status_text.to_string()
                                    }),
                            ),
                    ),
            )
            // Footer
            .child(
                div()
                    .absolute()
                    .bottom(px(20.0))
                    .text_xs()
                    .text_color(muted_color)
                    .child("Visual interface for Claude Code"),
            )
    }
}
