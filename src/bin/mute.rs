use std::fs::File;
use event_daemon::events::EventType;
use std::io::Write;

pub fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/volume-events.fifo")?;
    let event = EventType::MuteToggle;
    let event = format!("{}\n", event);
    let event = event.as_bytes();
    file.write_all(event)
}
