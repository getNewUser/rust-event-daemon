use event_daemon::events::EventType;
use event_daemon::handler::handle_event;
use event_daemon::fifo_helper::read_lines;

fn main()-> Result<(), Box<dyn std::error::Error>>{
    loop {
        eprintln!("opening pipe...");
        if let Ok(lines) = read_lines("/tmp/volume-events.fifo") {
            for line in lines.map_while(Result::ok) {
                let parsed: EventType = line.parse()?;
                handle_event(parsed);
            }
        }
    }
}

