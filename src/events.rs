#[derive(Debug, PartialEq, strum_macros::Display, strum_macros::EnumString)]
pub enum EventType {
    VolumeUp,
    VolumeDown,
    MuteToggle,
}

pub enum DaemonMessage {
    Event(EventType),
    ClearColor,
}
