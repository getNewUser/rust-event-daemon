use event_daemon::controller::{AmixerController, FallbackController, PactlController};
use event_daemon::core::events::{DaemonMessage, EventType};
use event_daemon::core::fifo_helper::{self, FifoFile, read_lines};
use event_daemon::core::handler::handle_event;
use event_daemon::core::state::{ColorState, DaemonState, VolumeState};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = DaemonState {
        volume_state: VolumeState {
            color: ColorState::Default,
            last_event_time: None,
            volume: None,
        },
    };

    let (msg_tx, msg_rx) = std::sync::mpsc::channel::<DaemonMessage>();
    let (reset_tx, reset_rx) = std::sync::mpsc::channel::<()>();

    let handler_tx = msg_tx.clone();
    thread::spawn(move || {
        loop {
            if let Ok(lines) = read_lines("/tmp/volume-events.fifo") {
                for line in lines.map_while(Result::ok) {
                    if let Ok(event) = line.parse::<EventType>() {
                        let result = handler_tx.send(DaemonMessage::Event(event));
                        // handle result
                    }
                }
            }
        }
    });

    let ticker_msg_tx = msg_tx.clone();
    thread::spawn(move || {
        loop {
            if reset_rx.recv().is_err() {
                break;
            }
            loop {
                match reset_rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(()) => continue,
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        ticker_msg_tx.send(DaemonMessage::ClearColor).ok();
                        break;
                    }
                    Err(_) => break,
                }
            }
        }
    });

    let audio_backend = FallbackController {
        primary: PactlController,
        fallback: AmixerController,
    };

    for msg in msg_rx {
        match msg {
            DaemonMessage::Event(event) => {
                handle_event(event, &audio_backend, &mut state);
                reset_tx.send(()).ok();
            }
            DaemonMessage::ClearColor => {
                state.volume_state.color = ColorState::Default;
                let colored_volume = state.volume_state.color.apply_color(
                    &state
                        .volume_state
                        .volume
                        .clone()
                        .unwrap_or_else(|| "NA".to_string()),
                );

                fifo_helper::write(&colored_volume, FifoFile::VolumeStatus);
            }
        }
    }
    Ok(())
}
