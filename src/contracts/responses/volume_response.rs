use serde::Serialize;

#[derive(Serialize)]
pub struct VolumeResponse {
    pub volume: String,
}
