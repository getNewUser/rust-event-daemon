use crate::core::events::AudioEventType;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventRequest {
    pub event: AudioEventType,
}
