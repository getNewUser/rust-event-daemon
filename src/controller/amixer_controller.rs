use crate::controller::{AudioController, AudioError};
use crate::util::command_helper::{AudioProgramType, run_audio_command};

pub struct AmixerController;

const PROGRAM: AudioProgramType = AudioProgramType::Amixer;

impl AudioController for AmixerController {
    fn adjust_volume(&self, volume: &str) -> Result<String, AudioError> {
        run_audio_command(PROGRAM, &["Master", volume])?;
        self.get_current_volume()
    }

    fn toggle_mute(&self) -> Result<String, AudioError> {
        run_audio_command(PROGRAM, &["set-sink-mute", "@DEFAULT_SINK@", "toggle"])?;
        let result = run_audio_command(PROGRAM, &["get-sink-mute", "@DEFAULT_SINK@"])?;

        if result.contains("yes") {
            eprintln!("sink is muted");
            Ok("MUTED".to_string())
        } else {
            eprintln!("sink is unmuted");
            self.get_current_volume()
        }
    }

    fn get_current_volume(&self) -> Result<String, AudioError> {
        let output = run_audio_command(PROGRAM, &["get", "Master"])?;
        output
            .lines()
            .find(|line| line.contains('%'))
            .and_then(|line| line.split_whitespace().find(|word| word.contains('%')))
            .map(|s| s.trim_matches(['[', ']']))
            .map(|x| x.to_string())
            .ok_or(AudioError::ParseError(
                "Could not find volume percentage".into(),
            ))
    }
}
