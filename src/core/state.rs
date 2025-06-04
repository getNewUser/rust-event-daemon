use std::time::Instant;
use strum_macros::Display;

#[derive(Debug, Clone, Display, Copy, PartialEq, Eq)]
pub enum ColorState {
    Default,
    Increased,
    Decreased,
}

#[derive(Debug, Clone)]
pub struct VolumeState {
    pub color: ColorState,
    pub last_event_time: Option<Instant>,
    pub volume: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DaemonState {
    pub volume_state: VolumeState,
}

impl ColorState {
    pub fn apply_color(&self, text: &str) -> String {
        match self {
            ColorState::Default => text.to_string(),
            ColorState::Increased => format!("%{{F#6b9c9a}}{}%{{F-}}", text), // muted teal
            ColorState::Decreased => format!("%{{F#a54242}}{}%{{F-}}", text), // matches alert
        }
    }
}
