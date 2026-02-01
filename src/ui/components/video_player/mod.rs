//! Video player components
//!
//! Provides video playback controls and display.

mod types;
mod controls;
mod overlay;
mod progress_bar;
mod player;
mod thumbnail;
mod card;
mod mini_player;
mod player_render;
mod thumbnail_render;
mod card_render;
mod mini_player_render;

#[cfg(test)]
mod tests;

pub use types::{VideoPlayerSize, VideoAspectRatio, VideoQuality, PlaybackState};
pub use player::VideoPlayer;
pub use thumbnail::VideoThumbnail;
pub use card::VideoCard;
pub use mini_player::MiniPlayer;
