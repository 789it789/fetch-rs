use std::process::Command;
use std::fs;
use std::path::Path;

pub fn get() -> String {

    if let Ok(output) = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "font-name"])
        .output()
        {
            if !output.stdout.is_empty() {
                let s = String::from_utf8_lossy(&output.stdout);
                return s.trim().trim_matches('\'').to_string(); // <-- fix
            }
        }

        if let Some(home) = std::env::var_os("HOME") {
            let kde_config = Path::new(&home).join(".config/kdeglobals");
            if kde_config.exists() {
                if let Ok(contents) = fs::read_to_string(&kde_config) {
                    for line in contents.lines() {
                        if line.starts_with("font=") {
                            return line["font=".len()..].trim().to_string();
                        }
                    }
                }
            }
        }

        if let Ok(output) = Command::new("fc-list").arg(":family").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.lines().next().unwrap_or("N/A").trim().to_string();
        }

        "N/A".to_string()
}
