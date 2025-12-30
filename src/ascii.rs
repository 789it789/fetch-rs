use std::fs;
use crate::acecat;

pub fn load_ascii(path: &str) -> Vec<String> {
    let raw_ascii = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return vec![String::new()],
    };

    if let Ok(colored_lines) = acecat::display_acecat(&raw_ascii) {
        return colored_lines;
    }

    raw_ascii.lines().map(|l| l.to_string()).collect()
}
