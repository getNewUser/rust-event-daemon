use crate::controller::AudioError;
use std::{fmt, process::Command};

pub fn run_audio_command(program: AudioProgramType, args: &[&str]) -> Result<String, AudioError> {
    let output = Command::new(program.to_string())
        .args(args)
        .output()
        .map_err(AudioError::Io)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(AudioError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

pub enum AudioProgramType {
    Pactl,
    Amixer,
}

impl fmt::Display for AudioProgramType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pactl => write!(f, "pactl"),
            Self::Amixer => write!(f, "amixer"),
        }
    }
}
