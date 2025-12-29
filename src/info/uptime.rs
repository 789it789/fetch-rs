use std::fs;

pub fn get() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/uptime") {
        if let Some(first_field) = contents.split_whitespace().next() {
            if let Ok(seconds) = first_field.parse::<f64>() {
                let total_seconds = seconds as u64;
                let days = total_seconds / 86400;
                let hours = (total_seconds % 86400) / 3600;
                let minutes = (total_seconds % 3600) / 60;

                let mut parts = Vec::new();
                if days > 0 {
                    parts.push(format!("{}d", days));
                }
                if hours > 0 {
                    parts.push(format!("{}h", hours));
                }
                parts.push(format!("{}m", minutes));

                return parts.join(" ");
            }
        }
    }

    "N/A".to_string()
}
