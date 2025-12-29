use std::process::Command;

pub fn current() -> String {
    Command::new("sh")
    .arg("-c")
    .arg("xrandr --current | grep '*' | head -n1 | awk '{print $1}'")
    .output()
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .unwrap_or("N/A".to_string())
}
