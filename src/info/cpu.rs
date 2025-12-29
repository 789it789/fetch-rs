use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn model() -> String {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("model name") {
                if let Some(colon) = line.find(':') {
                    return line[colon+1..].trim().to_string();
                }
            }
        }
    }
    "Unknown CPU".to_string()
}
