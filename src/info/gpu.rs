use std::process::Command;

pub fn model() -> String {
    Command::new("sh")
    .arg("-c")
    .arg("lspci | grep -i 'vga\\|3d\\|2d' | head -n1 | cut -d ':' -f3-")
    .output()
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .unwrap_or("Unknown GPU".to_string())
}
