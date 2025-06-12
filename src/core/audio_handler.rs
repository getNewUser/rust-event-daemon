use crate::controller::{AudioController, AudioError};
use crate::core::events::AudioEventType;
use crate::core::state::{ColorState, DaemonState};

use std::time::Instant;

pub fn handle_audio_event(
    event: AudioEventType,
    controller: &impl AudioController,
    state: &mut DaemonState,
) {
    eprintln!("🔔 Handling event: {:?}", event);
    match event {
        AudioEventType::VolumeUp => apply_event(
            state,
            controller.adjust_volume("+5%"),
            ColorState::Increased,
        ),
        AudioEventType::VolumeDown => apply_event(
            state,
            controller.adjust_volume("-5%"),
            ColorState::Decreased,
        ),
        AudioEventType::MuteToggle => {
            apply_event(state, controller.toggle_mute(), ColorState::Default)
        }
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
}
