use std::process::Command;

pub fn count() -> String {
    let managers = [
        ("pacman", "pacman -Qq | wc -l"),
        ("dpkg", "dpkg -l | wc -l"),
        ("rpm", "rpm -qa | wc -l"),
        ("eopkg", "eopkg list-installed | wc -l"),
        ("xbps-query", "xbps-query -l | wc -l"),
    ];

    for (check, cmd) in &managers {
        if Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", check))
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
            {
                return Command::new("sh")
                .arg("-c")
                .arg(*cmd)
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "N/A".to_string());
            }
    }

    "N/A".to_string()
}
