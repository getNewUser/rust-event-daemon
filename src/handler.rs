use std::io::Error;
use std::process::Command;
use std::process::Output;
use crate::events::EventType;
use crate::fifo_helper;
use crate::fifo_helper::FifoFile;


pub fn handle_event(event: EventType) {
    eprintln!("🔔 Handling event: {:?}", event);

    let result = match event {
        EventType::VolumeUp => adjust_volume("+5%", || fallback_amixer("+5%")),
        EventType::VolumeDown => adjust_volume("-5%", || fallback_amixer("-5%")), 
        EventType::MuteToggle => toggle_mute(toggle_mute_amixer_fallback),
    };

    eprintln!("📤 Final output: {}", result);
    fifo_helper::write(&result, FifoFile::VolumeStatus);
    eprintln!("✅ Wrote status to FIFO");
}

fn adjust_volume(volume: &str, fallback: impl FnOnce()) -> String {
    eprintln!("🔧 Adjusting volume by {}", volume);

    let result = Command::new("pactl")
        .args(["set-sink-volume", "@DEFAULT_SINK@", volume])
        .output();

    handle_output("pactl", &result, fallback);

    let volume = get_current_volume().unwrap_or("NA".to_string());
    eprintln!("🔊 Volume after adjustment: {}", volume);
    volume 
}

fn get_current_volume() -> Option<String> {
    if let Some(volume) = get_volume_from_pactl() {
        eprintln!("ℹ️ Volume from pactl: {}", volume);
        Some(volume)
    } else {
        eprintln!("⚠️ pactl failed, falling back to amixer...");
        let fallback_result = get_volume_from_amixer();
        if let Some(vol) = &fallback_result {
            eprintln!("ℹ️ Volume from amixer: {}", vol);
        } else {
            eprintln!("🚫 amixer also failed to get volume");
        } 
        fallback_result
    }
}
fn get_volume_from_pactl() -> Option<String>{
    let result = Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&result.stdout);
    text.split_whitespace().find(|s| s.ends_with("%")).map(|x| x.to_string())
}

fn get_volume_from_amixer() -> Option<String>{
    let output = Command::new("amixer")
        .args(["get", "Master"])
        .output()
        .ok();

    let text = output
        .as_ref()
        .map(|x| String::from_utf8_lossy(&x.stdout));

    text
        .as_deref()
        .and_then(|x| x.lines().find(|line| line.contains("%")))
        .and_then(|line| line.split_whitespace().find(|word| word.contains("%")))
        .map(|s| s.trim_matches(['[', ']']))
        .map(|x| x.to_string())
}

fn toggle_mute(fallback: impl FnOnce()) -> String{
    eprintln!("🔁 Toggling mute");
    let result = Command::new("pactl")
    .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
    .output();

    handle_output("pactl", &result, fallback);

    let status_output = Command::new("pactl")
        .args(["get-sink-mute", "@DEFAULT_SINK@"])
        .output();

    match status_output {
        Ok(output) => {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if text.contains("yes") {
                    eprintln!("🔇 Sink is now muted");
                    "MUTED".to_string()
                } else {
                    eprintln!("🔈 Sink is unmuted, getting volume...");
                    get_current_volume().unwrap_or_else(|| "NA".to_string())
                }
            } else {
                eprintln!("⚠️ pactl mute status command exited with error");
                "NA".to_string()
            }
        }
        Err(e) => {
            eprintln!("🚫 Failed to execute pactl mute status command: {}", e);
            "NA".to_string()
        }
    }
}

fn toggle_mute_amixer_fallback(){
    eprintln!("⛑️  Fallback: toggling mute with amixer");
    let result = Command::new("pactl")
    .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
    .output();

    handle_output("amixer", &result,|| {});
}

fn fallback_amixer(volume: &str){
    eprintln!("⛑️  Fallback: adjusting volume with amixer {}", volume);
    let result = Command::new("amixer")
         .args(["set", "Master", volume])
         .output();

     handle_output("amixer", &result, || {})
}



fn handle_output(tool: &str, result: &Result<Output, Error>, fallback: impl FnOnce()){
    match result {
        Ok(output) if output.status.success() => {
            eprintln!("✅ {} succeeded", tool);
        }
        Ok(output) => {
            eprintln!("⚠️ {} failed: {}", tool, String::from_utf8_lossy(&output.stderr));
            fallback();
        }
        Err(err) => {
            eprintln!("🚫 Failed to execute {}: {}", tool, err);
            fallback();
        }
    }
}
