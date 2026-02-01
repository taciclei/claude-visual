//! Video player tests

use super::controls::{format_time, format_views};
use super::*;

#[test]
fn test_video_player_sizes() {
    let sm = VideoPlayerSize::Sm;
    let lg = VideoPlayerSize::Lg;

    let (sm_w, sm_h) = sm.dimensions();
    let (lg_w, lg_h) = lg.dimensions();

    assert!(sm_w < lg_w);
    assert!(sm_h < lg_h);
}

#[test]
fn test_aspect_ratios() {
    let wide = VideoAspectRatio::Widescreen;
    let standard = VideoAspectRatio::Standard;

    assert!(wide.ratio() > standard.ratio());
}

#[test]
fn test_video_player() {
    let player = VideoPlayer::new("vp")
        .title("My Video")
        .duration(120.0)
        .current_time(30.0)
        .state(PlaybackState::Playing)
        .quality(VideoQuality::Q1080p);

    assert_eq!(player.duration, 120.0);
    assert_eq!(player.quality, VideoQuality::Q1080p);
}

#[test]
fn test_video_thumbnail() {
    let thumb = VideoThumbnail::new("vt")
        .duration(180.0)
        .title("Video Title")
        .progress(0.5);

    assert_eq!(thumb.duration, 180.0);
    assert_eq!(thumb.progress, Some(0.5));
}

#[test]
fn test_video_card() {
    let card = VideoCard::new("vc", "Amazing Video", "Channel Name")
        .views(1_500_000)
        .duration(600.0);

    assert_eq!(card.views, 1_500_000);
    assert_eq!(format_views(1_500_000), "1M views");
}

#[test]
fn test_time_format() {
    assert_eq!(format_time(65.0), "1:05");
    assert_eq!(format_time(3665.0), "1:01:05");
}

#[test]
fn test_mini_player() {
    let player = MiniPlayer::new("mp", "Playing Video")
        .state(PlaybackState::Playing)
        .progress(0.75);

    assert_eq!(player.state, PlaybackState::Playing);
    assert_eq!(player.progress, 0.75);
}
