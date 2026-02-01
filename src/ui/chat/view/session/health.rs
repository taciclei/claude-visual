//! Session health tracking methods

use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    /// Calculate session health score based on various factors
    pub fn calculate_session_health(&mut self, cx: &mut Context<Self>) {
        let mut health = 1.0_f32;

        // Reduce health based on context usage
        let context_pct = self.context_usage_percentage();
        if context_pct > 0.9 {
            health -= 0.4;
        } else if context_pct > 0.7 {
            health -= 0.2;
        } else if context_pct > 0.5 {
            health -= 0.1;
        }

        // Reduce health if there are errors
        if self.last_error.is_some() {
            health -= 0.2;
        }

        // Reduce health if connection is unstable
        if self.stats.connection_retry_count > 0 {
            health -= 0.1 * (self.stats.connection_retry_count as f32).min(3.0);
        }

        // Reduce health if response latency is high
        if let Some(latency) = self.stats.last_response_latency_ms {
            if latency > 10000 {
                health -= 0.15;
            } else if latency > 5000 {
                health -= 0.1;
            }
        }

        self.stats.health = health.max(0.0).min(1.0);
        self.stats.last_health_check = Some(chrono::Utc::now());
        cx.notify();
    }

    /// Get session health value (0.0 to 1.0)
    pub fn get_session_health(&self) -> f32 {
        self.stats.health
    }

    /// Get session health status label
    pub fn session_health_label(&self) -> &'static str {
        if self.stats.health > 0.8 {
            "Healthy"
        } else if self.stats.health > 0.5 {
            "Fair"
        } else if self.stats.health > 0.2 {
            "Degraded"
        } else {
            "Critical"
        }
    }

    /// Get session health color
    pub fn session_health_color(&self, theme: &crate::app::theme::Theme) -> gpui::Hsla {
        if self.stats.health > 0.8 {
            theme.colors.success
        } else if self.stats.health > 0.5 {
            theme.colors.warning
        } else {
            theme.colors.error
        }
    }

    // ==================== Token Tracking ====================

    /// Get context usage as a percentage (0.0 to 1.0)
    pub fn context_usage_percentage(&self) -> f64 {
        if self.context_capacity == 0 {
            return 0.0;
        }
        (self.context_used as f64 / self.context_capacity as f64).min(1.0)
    }

    // ==================== Cost Tracking ====================

    /// Show cost breakdown
    pub fn show_cost(&mut self, cx: &mut Context<Self>) {
        self.send_slash_command("/cost", cx);
    }
}
