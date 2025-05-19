use event_daemon::events::EventType;
use std::{fs::File, io::Write};

pub fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/volume-events.fifo")?;
    let event = EventType::VolumeDown;
    let event = format!("{}\n", event);
    let event = event.as_bytes();
    file.write_all(event)
}
