//! Video player components
//!
//! Provides video playback controls and display.

mod card;
mod card_render;
mod controls;
mod mini_player;
mod mini_player_render;
mod overlay;
mod player;
mod player_render;
mod progress_bar;
mod thumbnail;
mod thumbnail_render;
mod types;

#[cfg(test)]
mod tests;

pub use card::VideoCard;
pub use mini_player::MiniPlayer;
pub use player::VideoPlayer;
pub use thumbnail::VideoThumbnail;
pub use types::{PlaybackState, VideoAspectRatio, VideoPlayerSize, VideoQuality};
