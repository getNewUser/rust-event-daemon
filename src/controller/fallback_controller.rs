use crate::controller::{AudioController, AudioError, audio_controller};
pub struct FallbackController<B1, B2> {
    pub primary: B1,
    pub fallback: B2,
}

impl<B1, B2> AudioController for FallbackController<B1, B2>
where
    B1: AudioController,
    B2: AudioController,
{
    fn adjust_volume(&self, volume: &str) -> Result<String, audio_controller::AudioError> {
        self.primary.adjust_volume(volume).or_else(|primary_err| {
            self.fallback
                .adjust_volume(volume)
                .map_err(|fallback_err| AudioError::BothFailed {
                    primary: Box::new(primary_err),
                    secondary: Box::new(fallback_err),
                })
        })
    }

    fn toggle_mute(&self) -> Result<String, AudioError> {
        self.primary.toggle_mute().or_else(|primary_err| {
            self.fallback
                .toggle_mute()
                .map_err(|fallback_err| AudioError::BothFailed {
                    primary: Box::new(primary_err),
                    secondary: Box::new(fallback_err),
                })
        })
    }

    fn get_current_volume(&self) -> Result<String, AudioError> {
        self.primary.get_current_volume().or_else(|primary_err| {
            self.fallback
                .get_current_volume()
                .map_err(|fallback_err| AudioError::BothFailed {
                    primary: Box::new(primary_err),
                    secondary: Box::new(fallback_err),
                })
        })
    }
}
