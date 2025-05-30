use crate::audio_backend_trait::{AudioBackend, AudioError};
use crate::events::EventType;
use crate::fifo_helper;
use crate::fifo_helper::FifoFile;
use crate::state::{ColorState, DaemonState};
use std::time::Instant;

pub fn handle_event(event: EventType, backend: &impl AudioBackend, state: &mut DaemonState) {
    eprintln!("🔔 Handling event: {:?}", event);
    match event {
        EventType::VolumeUp => {
            apply_event(state, backend.adjust_volume("+5%"), ColorState::Increased)
        }
        EventType::VolumeDown => {
            apply_event(state, backend.adjust_volume("-5%"), ColorState::Decreased)
        }
        EventType::MuteToggle => apply_event(state, backend.toggle_mute(), ColorState::Default),
    };

    eprintln!("✅ Wrote status to FIFO");
}

fn apply_event(state: &mut DaemonState, result: Result<String, AudioError>, color: ColorState) {
    match result {
        Ok(vol) => {
            state.volume_state.volume = Some(vol);
            state.volume_state.color = color;
            state.volume_state.last_event_time = Some(Instant::now());
        }
        Err(e) => {
            eprintln!("❌ Audio backend failed: {e:?}");
            state.volume_state.volume = Some("NA".into());
            state.volume_state.color = ColorState::Default;
            state.volume_state.last_event_time = None;
        }
    };
    let colored_volume = state
        .volume_state
        .color
        .apply_color(&state.volume_state.volume.clone().unwrap());

    fifo_helper::write(&colored_volume, FifoFile::VolumeStatus);
}
