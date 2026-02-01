//! Core lazy block implementation

use gpui::*;
use std::sync::Arc;
use super::config::LazyBlockConfig;
use super::types::{LazyState, LazyBlockEvent};

/// Lazy block wrapper that defers rendering
pub struct LazyBlock<T: IntoElement + Clone + 'static> {
    /// Unique ID for this block
    pub(crate) id: ElementId,
    /// Current loading state
    pub(crate) state: LazyState,
    /// Configuration
    pub(crate) config: LazyBlockConfig,
    /// Content builder (called when visible)
    pub(crate) content_builder: Option<Arc<dyn Fn() -> T + Send + Sync>>,
    /// Cached content (after first render)
    pub(crate) cached_content: Option<T>,
    /// Error message if loading failed
    pub(crate) error: Option<String>,
    /// Whether currently visible
    pub(crate) is_visible: bool,
}

impl<T: IntoElement + Clone + 'static> LazyBlock<T> {
    /// Create a new lazy block
    pub fn new(
        id: impl Into<ElementId>,
        config: LazyBlockConfig,
        content_builder: impl Fn() -> T + Send + Sync + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            state: LazyState::Pending,
            config,
            content_builder: Some(Arc::new(content_builder)),
            cached_content: None,
            error: None,
            is_visible: false,
        }
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool, cx: &mut Context<Self>) {
        if self.is_visible != visible {
            self.is_visible = visible;

            if visible && self.state == LazyState::Pending {
                self.load_content(cx);
            }

            cx.emit(if visible {
                LazyBlockEvent::BecameVisible
            } else {
                LazyBlockEvent::BecameHidden
            });

            cx.notify();
        }
    }

    /// Load the content
    fn load_content(&mut self, cx: &mut Context<Self>) {
        if let Some(builder) = &self.content_builder {
            self.state = LazyState::Loading;
            cx.notify();

            // Build content
            let content = builder();
            self.cached_content = Some(content);
            self.state = LazyState::Loaded;

            cx.emit(LazyBlockEvent::ContentLoaded);
            cx.notify();
        }
    }

    /// Force load (ignore visibility)
    pub fn force_load(&mut self, cx: &mut Context<Self>) {
        if self.state == LazyState::Pending {
            self.load_content(cx);
        }
    }

    /// Get current state
    pub fn state(&self) -> LazyState {
        self.state
    }

    /// Check if content is loaded
    pub fn is_loaded(&self) -> bool {
        self.state == LazyState::Loaded
    }

    /// Get estimated height
    pub fn estimated_height(&self) -> f32 {
        self.config.estimated_height
    }
}

impl<T: IntoElement + Clone + 'static> EventEmitter<LazyBlockEvent> for LazyBlock<T> {}
