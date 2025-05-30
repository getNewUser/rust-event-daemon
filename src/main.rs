use event_daemon::amixer_audio::AmixerAudio;
use event_daemon::events::DaemonMessage;
use event_daemon::events::EventType;
use event_daemon::fallback_backend::FallbackBackend;
use event_daemon::fifo_helper;
use event_daemon::fifo_helper::FifoFile;
use event_daemon::fifo_helper::read_lines;
use event_daemon::handler::handle_event;
use event_daemon::pactl_audio::PactlAudio;
use event_daemon::state::ColorState;
use event_daemon::state::DaemonState;
use event_daemon::state::VolumeState;
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

    let audio_backend = FallbackBackend {
        primary: PactlAudio,
        fallback: AmixerAudio,
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
