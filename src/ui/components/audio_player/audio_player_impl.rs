//! AudioPlayer RenderOnce implementation

use super::player::AudioPlayer;
use super::render::{render_compact, render_full, render_minimal};
use super::types::AudioPlayerVariant;
use gpui::prelude::*;
use gpui::*;

impl RenderOnce for AudioPlayer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = self.id.clone();

        let content = match self.variant {
            AudioPlayerVariant::Minimal => render_minimal(&self).into_any_element(),
            AudioPlayerVariant::Full => render_full(&self).into_any_element(),
            _ => render_compact(&self).into_any_element(),
        };

        div().id(id).child(content)
    }
}
