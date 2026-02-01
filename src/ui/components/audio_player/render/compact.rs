//! Compact/Default variant rendering

use super::AudioColors;
use crate::ui::components::audio_player::player::AudioPlayer;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

pub(crate) fn render_compact(player: &AudioPlayer) -> impl IntoElement {
    let colors = AudioColors::default();
    let height = player.size.height();
    let button_size = player.size.button_size();
    let font_size = player.size.font_size();

    let progress = if player.duration > 0.0 {
        (player.current_time / player.duration).clamp(0.0, 1.0) as f32
    } else {
        0.0
    };

    let play_icon = match player.state {
        super::super::types::PlaybackState::Playing => "⏸",
        super::super::types::PlaybackState::Loading => "⏳",
        super::super::types::PlaybackState::Error => "⚠️",
        _ => "▶",
    };

    let volume_icon = if player.muted || player.volume == 0.0 {
        "🔇"
    } else if player.volume < 0.5 {
        "🔉"
    } else {
        "🔊"
    };

    div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .h(px(height))
        .px(px(12.0))
        .bg(colors.container_bg)
        .border_1()
        .border_color(colors.border)
        .rounded(px(8.0))
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
                        .child(play_icon),
                ),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .h(px(4.0))
                        .bg(colors.track_bg)
                        .rounded_full()
                        .cursor_pointer()
                        .child(
                            div()
                                .h_full()
                                .w(pct(progress * 100.0))
                                .bg(colors.track_fg)
                                .rounded_full(),
                        ),
                )
                .when(player.show_time, |el| {
                    el.child(
                        div()
                            .flex()
                            .justify_between()
                            .text_size(px(font_size))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child(AudioPlayer::format_time(player.current_time))
                            .child(AudioPlayer::format_time(player.duration)),
                    )
                }),
        )
        .when(player.show_volume, |el| {
            el.child(
                div()
                    .text_size(px(14.0))
                    .cursor_pointer()
                    .child(volume_icon),
            )
        })
}
