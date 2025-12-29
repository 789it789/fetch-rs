use std::process::Command;

pub fn usage() -> String {
    Command::new("df")
    .arg("-h")
    .arg("/")
    .output()
    .map(|o| {
        let out = String::from_utf8_lossy(&o.stdout);
        out.lines().nth(1)
        .map(|line| line.split_whitespace().nth(2).unwrap_or("N/A").to_string() + " / " +
        line.split_whitespace().nth(1).unwrap_or("N/A"))
        .unwrap_or("N/A".to_string())
    })
    .unwrap_or("N/A".to_string())
}
