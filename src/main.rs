use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use event_daemon::events::EventType;
use event_daemon::handler::handle_event;

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
fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
