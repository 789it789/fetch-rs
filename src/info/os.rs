use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn name() -> String {
    if let Ok(file) = File::open("/etc/os-release") {
        for line in BufReader::new(file).lines().flatten() {
            if line.starts_with("PRETTY_NAME=") {
                let val = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
                return val.to_string();
            }
        }
    }
    "Unknown Linux".to_string()
}

pub fn kernel() -> String {
    std::process::Command::new("uname")
    .arg("-r")
    .output()
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .unwrap_or_else(|_| "Unknown Kernel".to_string())
}
