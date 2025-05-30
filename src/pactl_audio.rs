use crate::audio_backend_trait::AudioBackend;
use crate::audio_backend_trait::AudioError;
use crate::command_helper::AudioProgramType;
use crate::command_helper::run_audio_command;

pub struct PactlAudio;

const PROGRAM: AudioProgramType = AudioProgramType::Pactl;

impl AudioBackend for PactlAudio {
    fn adjust_volume(&self, volume: &str) -> Result<String, AudioError> {
        eprintln!("🔧 Adjusting volume by {}", volume);

        run_audio_command(PROGRAM, &["set-sink-volume", "@DEFAULT_SINK@", volume])?;
        self.get_current_volume()
    }

    fn get_current_volume(&self) -> Result<String, AudioError> {
        let output = run_audio_command(PROGRAM, &["get-sink-volume", "@DEFAULT_SINK@"])?;
        output
            .split_whitespace()
            .find(|s| s.ends_with("%"))
            .map(|x| x.to_string())
            .ok_or(AudioError::ParseError(
                "Could not find volume percentage".into(),
            ))
    }

    fn toggle_mute(&self) -> Result<String, AudioError> {
        run_audio_command(PROGRAM, &["set-sink-mute", "@DEFAULT_SINK@", "toggle"])?;
        let output = run_audio_command(PROGRAM, &["get-sink-mute", "@DEFAULT_SINK@"])?;

        if output.contains("yes") {
            eprintln!("Sink is muted");
            Ok("MUTED".to_string())
        } else {
            eprintln!("Sink is unmuted, getting volume");
            self.get_current_volume()
        }
    }
}
