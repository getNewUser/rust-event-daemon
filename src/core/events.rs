use serde::Deserialize;

#[derive(Debug, PartialEq, strum_macros::Display, strum_macros::EnumString, Deserialize)]
pub enum AudioEventType {
    VolumeUp,
    VolumeDown,
    MuteToggle,
}
