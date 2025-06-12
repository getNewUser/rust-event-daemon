use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::{Router, response::IntoResponse, response::Response, routing::get, routing::post};
use event_daemon::contracts::requests::event_request::EventRequest;
use event_daemon::controller::{AmixerController, FallbackController, PactlController};
use event_daemon::core::audio_handler::handle_audio_event;
use event_daemon::core::state::{ColorState, DaemonState, VolumeState};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::Instant;

type SharedState = Arc<Mutex<DaemonState>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(DaemonState {
        volume_state: VolumeState {
            color: ColorState::Default,
            last_event_time: None,
            volume: None,
        },
    }));

    let app = Router::new()
        .route("/volume", get(get_volume))
        .route("/event", post(post_event))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let state_for_reset = state.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let mut locked_state = state_for_reset.lock().unwrap();

            if let Some(last) = locked_state.volume_state.last_event_time {
                if last.elapsed() > Duration::from_secs(1) {
                    eprintln!("reseting color");
                    locked_state.volume_state.color = ColorState::Default;
                    locked_state.volume_state.last_event_time = None;
                }
            }
        }
    });
    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn get_volume(State(state): State<SharedState>) -> Response {
    let state = state.lock().unwrap();
    let volume = state
        .volume_state
        .volume
        .clone()
        .unwrap_or("NA".to_string());
    let volume = state.volume_state.color.apply_color(&volume);
    (StatusCode::OK, volume).into_response()
}

async fn post_event(
    State(state): State<SharedState>,
    Json(payload): Json<EventRequest>,
) -> StatusCode {
    let audio_controller = FallbackController {
        primary: PactlController,
        fallback: AmixerController,
    };
    let mut state = state.lock().unwrap();
    handle_audio_event(payload.event, &audio_controller, &mut state);
    state.volume_state.last_event_time = Some(Instant::now());
    StatusCode::OK
}
