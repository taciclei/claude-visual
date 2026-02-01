//! Audio player components
//!
//! Provides audio playback controls and visualization.

mod types;
mod player;
mod render;
mod audio_player_impl;
mod voice_message;
mod record_button;
mod podcast_player;

pub use types::{PlaybackState, AudioPlayerSize, AudioPlayerVariant};
pub use player::AudioPlayer;
pub use voice_message::VoiceMessage;
pub use record_button::AudioRecordButton;
pub use podcast_player::PodcastPlayer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_player_sizes() {
        let sm = AudioPlayerSize::Sm;
        let lg = AudioPlayerSize::Lg;

        assert!(sm.height() < lg.height());
        assert!(sm.button_size() < lg.button_size());
    }

    #[test]
    fn test_audio_player() {
        let player = AudioPlayer::new("ap")
            .title("My Song")
            .artist("Artist Name")
            .duration(180.0)
            .current_time(45.0)
            .state(PlaybackState::Playing)
            .volume(0.8);

        assert_eq!(player.duration, 180.0);
        assert_eq!(player.current_time, 45.0);
        assert_eq!(player.state, PlaybackState::Playing);
    }

    #[test]
    fn test_voice_message() {
        let waveform: Vec<f32> = vec![0.3, 0.5, 0.8, 0.4, 0.6];
        let msg = VoiceMessage::new("vm")
            .duration(12.5)
            .current_time(5.0)
            .waveform(waveform)
            .is_outgoing(true);

        assert_eq!(msg.duration, 12.5);
        assert!(msg.is_outgoing);
    }

    #[test]
    fn test_record_button() {
        let btn = AudioRecordButton::new("rb")
            .is_recording(true)
            .recording_time(30.5);

        assert!(btn.is_recording);
        assert_eq!(btn.recording_time, 30.5);
    }

    #[test]
    fn test_podcast_player() {
        let player = PodcastPlayer::new("pp", "Episode 1", "My Podcast")
            .duration(3600.0)
            .playback_speed(1.5);

        assert_eq!(player.duration, 3600.0);
        assert_eq!(player.playback_speed, 1.5);
    }

    #[test]
    fn test_time_format() {
        assert_eq!(AudioPlayer::format_time(65.0), "1:05");
        assert_eq!(AudioPlayer::format_time(125.0), "2:05");
        assert_eq!(AudioPlayer::format_time(0.0), "0:00");
    }
}
