use event_daemon::events::EventType;
use event_daemon::fifo_helper;
use event_daemon::fifo_helper::FifoFile;
use event_daemon::fifo_helper::read_lines;
use event_daemon::handler::handle_event;
use event_daemon::state::ColorState;
use event_daemon::state::DaemonState;
use event_daemon::state::VolumeState;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(DaemonState {
        volume_state: VolumeState {
            color: ColorState::Default,
            last_event_time: None,
            volume: None,
        },
    }));

    let state_for_handler = Arc::clone(&state);
    thread::spawn(move || {
        loop {
            if let Ok(lines) = read_lines("/tmp/volume-events.fifo") {
                for line in lines.map_while(Result::ok) {
                    if let Ok(event) = line.parse::<EventType>() {
                        let mut state = state_for_handler.lock().unwrap();
                        handle_event(event, &mut state);
                    }
                }
            }
        }
    });

    let state_for_ticker = Arc::clone(&state);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            let mut state = state_for_ticker.lock().unwrap();
            if let Some(last_event_time) = state.volume_state.last_event_time {
                if last_event_time.elapsed() >= Duration::from_secs(1) {
                    state.volume_state.color = ColorState::Default;
                    state.volume_state.last_event_time = None;
                    let colored_volume = state
                        .volume_state
                        .color
                        .apply_color(&state.volume_state.volume.clone().unwrap());

                    fifo_helper::write(&colored_volume, FifoFile::VolumeStatus);
                }
            }
        }
    });

    std::thread::park();
    Ok(())
}
