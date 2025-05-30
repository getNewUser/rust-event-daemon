use crate::audio_backend_trait::{AudioBackend, AudioError};

pub struct FallbackBackend<B1, B2> {
    pub primary: B1,
    pub fallback: B2,
}

impl<B1, B2> AudioBackend for FallbackBackend<B1, B2>
where
    B1: AudioBackend,
    B2: AudioBackend,
{
    fn adjust_volume(
        &self,
        volume: &str,
    ) -> Result<String, crate::audio_backend_trait::AudioError> {
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
