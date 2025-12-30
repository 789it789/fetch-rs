use std::io::{self};

const COLORS: [[u8; 3]; 4] = [
    [0, 0, 0],
[164, 164, 164],
[255, 255, 255],
[129, 0, 129],
];

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn interpolate_color(t: f64) -> (u8, u8, u8) {
    if t <= 0.0 {
        return (COLORS[0][0], COLORS[0][1], COLORS[0][2]);
    }
    if t >= 1.0 {
        return (COLORS[3][0], COLORS[3][1], COLORS[3][2]);
    }
    let scaled = t * 3.0;
    let idx = scaled.floor() as usize;
    let local_t = scaled - idx as f64;

    (
        lerp(COLORS[idx][0] as f64, COLORS[idx + 1][0] as f64, local_t) as u8,
     lerp(COLORS[idx][1] as f64, COLORS[idx + 1][1] as f64, local_t) as u8,
     lerp(COLORS[idx][2] as f64, COLORS[idx + 1][2] as f64, local_t) as u8,
    )
}

pub fn display_acecat(raw_ascii: &str) -> io::Result<Vec<String>> {
    let lines: Vec<&str> = raw_ascii.lines().collect();
    let lines_count = lines.len();
    let mut colored_lines = Vec::new();

    for (i, &line) in lines.iter().enumerate() {
        let t = if lines_count == 1 { 0.0 } else { i as f64 / (lines_count - 1) as f64 };
        let (r, g, b) = interpolate_color(t);
        let colored_line = format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, line);
        colored_lines.push(colored_line);
    }

    Ok(colored_lines)
}
