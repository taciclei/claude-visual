//! Streaming metrics and animation

use gpui::*;

use crate::ui::chat::view::ChatView;

impl ChatView {
    /// Update streaming speed metrics
    pub(crate) fn update_streaming_speed(&mut self, new_tokens: usize) {
        self.streaming.token_count += new_tokens;

        if let Some(start) = self.streaming.response_start_time {
            let elapsed = chrono::Utc::now().signed_duration_since(start);
            let seconds = elapsed.num_milliseconds() as f64 / 1000.0;
            if seconds > 0.1 {
                let speed = self.streaming.token_count as f64 / seconds;
                self.streaming.last_speed = speed;
                if speed > self.streaming.peak_speed {
                    self.streaming.peak_speed = speed;
                }
            }
        }
    }

    /// Reset streaming metrics for new response
    pub(crate) fn reset_streaming_metrics(&mut self) {
        self.streaming.token_count = 0;
        self.streaming.last_speed = 0.0;
    }

    /// Update streaming animation
    pub(crate) fn start_streaming_animation(&mut self, cx: &mut Context<Self>) {
        // Start animation timer
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(std::time::Duration::from_millis(300)).await;
                let should_continue = this.update(cx, |view, cx| {
                    // Continue animation if streaming or has active tasks
                    if view.streaming.is_streaming || !view.active_tasks.is_empty() {
                        view.streaming.streaming_dots = (view.streaming.streaming_dots + 1) % 4;
                        cx.notify();
                        true
                    } else {
                        view.streaming.streaming_dots = 0;
                        false
                    }
                }).unwrap_or(false);

                if !should_continue {
                    break;
                }
            }
        }).detach();
    }
}
