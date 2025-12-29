use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn usage() -> String {
    if let Ok(file) = File::open("/proc/meminfo") {
        let mut total = 0;
        let mut avail = 0;
        for line in BufReader::new(file).lines().flatten() {
            if line.starts_with("MemTotal:") {
                total = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0);
            }
            if line.starts_with("MemAvailable:") {
                avail = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0);
            }
        }
        if total > 0 {
            let used = (total - avail) as f64 / 1024.0 / 1024.0;
            let total_gb = total as f64 / 1024.0 / 1024.0;
            return format!("{:.1} / {:.1} GB", used, total_gb);
        }
    }
    "N/A".to_string()
}
