use std::fs;
use std::process::{Command, Stdio};
use std::io::Write;

pub fn load_ascii(path: &str) -> Vec<String> {
    // Read the ASCII file
    let raw_ascii = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Failed to read ASCII file '{}': {}", path, err);
            return vec!["".to_string()];
        }
    };

    let acecat_available = Command::new("which")
    .arg("acecat-c")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .map(|s| s.success())
    .unwrap_or(false);

    if acecat_available {
        if let Ok(mut child) = Command::new("acecat-c")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(raw_ascii.as_bytes());
                    let _ = stdin.flush();
                }

                if let Some(stdout) = child.stdout.take() {
                    use std::io::{BufRead, BufReader};
                    let reader = BufReader::new(stdout);
                    return reader.lines().filter_map(Result::ok).collect();
                }
            }
    }

    raw_ascii.lines().map(|l| l.to_string()).collect()
}
