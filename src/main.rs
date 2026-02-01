//! Claude Visual - A visual client for Claude Code
//!
//! This application provides a modern, GPU-accelerated interface for interacting
//! with Claude Code CLI, inspired by the Warp terminal.

mod actions;
mod agent;

// Re-export all actions for backward compatibility
pub use actions::*;
mod ai;
mod app;
mod claude;
mod cloud;
mod debug;
mod git;
mod i18n;
mod lsp;
mod markdown;
mod mcp;
mod plugins;
mod project;
mod storage;
mod syntax;
mod terminal;
mod ui;
mod update;

use anyhow::Result;
use gpui::*;
use gpui::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use app::state::AppState;
use ui::workspace::Workspace;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Claude Visual");

    // Initialize GPUI application
    Application::new().run(|cx: &mut App| {
        // Initialize app state
        let app_state = AppState::new(cx);

        // Register global actions
        actions::register_actions(cx);

        // Create main window
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Claude Visual".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(9.0), px(9.0))),
            }),
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
            is_resizable: true,
            is_minimizable: true,
            display_id: None,
            window_background: WindowBackgroundAppearance::Blurred,
            app_id: Some("com.claude-visual".to_string()),
            window_min_size: Some(size(px(800.0), px(600.0))),
            window_decorations: None,
            tabbing_identifier: None,
        };

        cx.open_window(window_options, |_window, cx| {
            cx.new(|cx| Workspace::new(app_state, cx))
        })
        .expect("Failed to open window");

        cx.activate(true);
    });

    Ok(())
}
