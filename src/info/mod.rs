pub mod cpu;
pub mod memory;
pub mod os;
pub mod disk;
pub mod packages;
pub mod gpu;
pub mod de;
pub mod resolution;
pub mod systemfont;
pub mod uptime;


use std::process::{Command, Stdio};
use std::io::Write;

pub fn all() -> Vec<String> {
    let lines = vec![
        format!("\n\n\n\nOS: {}", os::name()),
        format!("Kernel: {}", os::kernel()),
        format!("CPU: {}", cpu::model()),
        format!("GPU: {}", gpu::model()),
        format!("Memory: {}", memory::usage()),
        format!("Disk: {}", disk::usage()),
        format!("Packages: {}", packages::count()),
        format!("DE: {}", de::name()),
        format!("Resolution: {}", resolution::current()),
        format!("Font: {}", systemfont::get()),
        format!("Uptime: {}", uptime::get())
    ];

    colorize(&lines)
}

fn colorize(lines: &[String]) -> Vec<String> {
    let lolcat_available = Command::new("which")
    .arg("lolcat-c")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .map(|s| s.success())
    .unwrap_or(false);

    if lolcat_available {
        if let Ok(mut child) = Command::new("lolcat-c")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(lines.join("\n").as_bytes());
                    let _ = stdin.flush();
                }
                if let Some(stdout) = child.stdout.take() {
                    use std::io::{BufRead, BufReader};
                    let reader = BufReader::new(stdout);
                    return reader.lines().filter_map(Result::ok).collect();
                }
            }
    }

    lines.to_vec()
}
