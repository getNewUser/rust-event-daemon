use std::io::Error;
use std::process::Command;
use std::process::Output;

use crate::events::EventType;

pub fn handle_event(event: EventType) {
    let result = match event {
        EventType::VolumeUp => adjust_volume("+5%", || fallback_amixer("+5%")),
        EventType::VolumeDown => adjust_volume("-5%", || fallback_amixer("-5%")), 
        EventType::MuteToggle => toggle_mute(toggle_mute_amixer_fallback),
    };

    println!("{}", result);
}

fn adjust_volume(volume: &str, fallback: impl FnOnce()) -> String {
    let result = Command::new("pactl")
        .args(["set-sink-volume", "@DEFAULT_SINK@", volume])
        .output();

    handle_output("pactl", &result, fallback);

    get_current_volume().unwrap_or("NA".to_string())
}

fn get_current_volume() -> Option<String> {
    if let Some(volume) = get_volume_from_pactl() {
        Some(volume)
    } else {
        get_volume_from_amixer()
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
                    "MUTED".to_string()
                } else {
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
    let result = Command::new("pactl")
    .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
    .output();

    handle_output("amixer", &result,|| {});
}

fn fallback_amixer(volume: &str){
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
