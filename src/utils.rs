use plist::Error;
use plist::Value;
use std::fs;
use std::process::Command;
use terminal_size::terminal_size;
use terminal_size::Height;
use terminal_size::Width;

pub fn get_terminal_size() -> (usize, usize) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w as usize, h as usize)
    } else {
        (0, 0)
    }
}

// get_powermetrics_value(duration, samplers)
pub fn get_powermetrics_value(duration: u32) -> Result<Value, Error> {
    let output = Command::new("powermetrics")
        .args(["-n", "1", "-i", &duration.to_string(), "-f", "plist"])
        .output()
        .expect("powermetrics execute failed");

    Value::from_reader_xml(&*output.stdout)
}
// separar en get_powermetrics(duration) y parse_powermetrics(powermetrics)

pub fn get_os_version() -> String {
    let file = fs::read_to_string("/System/Library/CoreServices/SystemVersion.plist")
        .expect("SystemVersion.plist missing");

    let v = Value::from_reader_xml(file.as_bytes()).expect("SystemVersion.plist not plist");

    let version = v
        .as_dictionary()
        .expect("SystemVersion.plist not dictionary")
        .get("ProductVersion")
        .expect("ProductVersion missing");

    version
        .as_string()
        .expect("ProductVersion not string")
        .to_string()
}
