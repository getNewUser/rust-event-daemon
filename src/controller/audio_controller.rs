pub trait AudioController {
    fn adjust_volume(&self, volume: &str) -> Result<String, AudioError>;
    fn toggle_mute(&self) -> Result<String, AudioError>;
    fn get_current_volume(&self) -> Result<String, AudioError>;
}

#[derive(Debug)]
pub enum AudioError {
    Io(std::io::Error),
    CommandFailed(String),
    ParseError(String),
    BackendUnavailable,
    BothFailed {
        primary: Box<AudioError>,
        secondary: Box<AudioError>,
    },
}
