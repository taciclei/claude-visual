//! Full variant rendering

use gpui::*;
use gpui::prelude::*;
use crate::ui::components::audio_player::player::AudioPlayer;
use crate::ui::pct;
use super::AudioColors;

pub(crate) fn render_full(player: &AudioPlayer) -> impl IntoElement {
    let colors = AudioColors::default();

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
        .flex_col()
        .gap(px(8.0))
        .p(px(12.0))
        .bg(colors.container_bg)
        .border_1()
        .border_color(colors.border)
        .rounded(px(12.0))
        .when(player.title.is_some() || player.artist.is_some(), |el| {
            el.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .when_some(player.title.clone(), |el, title| {
                        el.child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .text_color(colors.text_primary)
                                .child(title)
                        )
                    })
                    .when_some(player.artist.clone(), |el, artist| {
                        el.child(
                            div()
                                .text_size(px(12.0))
                                .text_color(colors.text_secondary)
                                .child(artist)
                        )
                    })
            )
        })
        .child(
            div()
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
                                .rounded_full()
                        )
                )
                .when(player.show_time, |el| {
                    el.child(
                        div()
                            .flex()
                            .justify_between()
                            .text_size(px(11.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child(AudioPlayer::format_time(player.current_time))
                            .child(AudioPlayer::format_time(player.duration))
                    )
                })
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(16.0))
                                .cursor_pointer()
                                .child("⏮")
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(40.0))
                                .h(px(40.0))
                                .bg(colors.primary_bg)
                                .rounded_full()
                                .cursor_pointer()
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .text_color(colors.primary_fg)
                                        .child(play_icon)
                                )
                        )
                        .child(
                            div()
                                .text_size(px(16.0))
                                .cursor_pointer()
                                .child("⏭")
                        )
                )
                .when(player.show_volume, |el| {
                    el.child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .cursor_pointer()
                                    .child(volume_icon)
                            )
                            .child(
                                div()
                                    .w(px(60.0))
                                    .h(px(4.0))
                                    .bg(colors.track_bg)
                                    .rounded_full()
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .h_full()
                                            .w(pct(if player.muted { 0.0 } else { player.volume * 100.0 }))
                                            .bg(hsla(0.0, 0.0, 0.5, 1.0))
                                            .rounded_full()
                                    )
                            )
                    )
                })
        )
}
