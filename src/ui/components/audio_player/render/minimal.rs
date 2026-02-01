//! Minimal variant rendering

use gpui::*;
use gpui::prelude::*;
use crate::ui::components::audio_player::player::AudioPlayer;
use super::AudioColors;

pub(crate) fn render_minimal(player: &AudioPlayer) -> impl IntoElement {
    let colors = AudioColors::default();
    let button_size = player.size.button_size();
    let font_size = player.size.font_size();

    let play_icon = match player.state {
        super::super::types::PlaybackState::Playing => "⏸",
        super::super::types::PlaybackState::Loading => "⏳",
        super::super::types::PlaybackState::Error => "⚠️",
        _ => "▶",
    };

    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(button_size))
                .h(px(button_size))
                .bg(colors.primary_bg)
                .rounded_full()
                .cursor_pointer()
                .child(
                    div()
                        .text_size(px(button_size * 0.4))
                        .text_color(colors.primary_fg)
                        .child(play_icon)
                )
        )
        .when(player.show_time, |el| {
            el.child(
                div()
                    .text_size(px(font_size))
                    .text_color(colors.text_secondary)
                    .child(format!(
                        "{} / {}",
                        AudioPlayer::format_time(player.current_time),
                        AudioPlayer::format_time(player.duration)
                    ))
            )
        })
}
