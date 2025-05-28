use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorState {
    Default,
    Increased,
    Decreased
}

pub struct VolumeState {
    pub color: ColorState,
    pub last_event_time: Option<Instant>,
    pub volume: Option<u8>,
}

pub struct DaemonState {
    pub volume_state: VolumeState
}
