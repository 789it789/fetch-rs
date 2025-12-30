use std::time::{SystemTime, UNIX_EPOCH};

const NUM_COLORS: usize = 30;

static COLORS: [&str; NUM_COLORS] = [
    "\x1b[38;5;39m",  "\x1b[38;5;38m",  "\x1b[38;5;44m",  "\x1b[38;5;43m",
"\x1b[38;5;49m",  "\x1b[38;5;48m",  "\x1b[38;5;84m",  "\x1b[38;5;83m",
"\x1b[38;5;119m", "\x1b[38;5;118m", "\x1b[38;5;154m", "\x1b[38;5;148m",
"\x1b[38;5;184m", "\x1b[38;5;178m", "\x1b[38;5;214m", "\x1b[38;5;208m",
"\x1b[38;5;209m", "\x1b[38;5;203m", "\x1b[38;5;204m", "\x1b[38;5;198m",
"\x1b[38;5;199m", "\x1b[38;5;163m", "\x1b[38;5;164m", "\x1b[38;5;128m",
"\x1b[38;5;129m", "\x1b[38;5;93m",  "\x1b[38;5;99m",  "\x1b[38;5;63m",
"\x1b[38;5;69m",  "\x1b[38;5;33m",
];

const RESET: &str = "\x1b[0m";

#[inline]
fn utf8_len(b: u8) -> usize {
    if b < 0x80 { 1 }
    else if b < 0xE0 { 2 }
    else if b < 0xF0 { 3 }
    else { 4 }
}

#[inline]
fn skip_ansi(bytes: &[u8], mut i: usize) -> usize {
    i += 2;
    while i < bytes.len() && !(bytes[i] >= 0x40 && bytes[i] <= 0x7E) {
        i += 1;
    }
    i + 1
}

pub fn render_lines(lines: &[String]) -> Vec<String> {
    let seed = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as f32;

    let mut gradient = 0.4 + (seed % 100.0) * 0.002;
    let mut line_offset = (seed as i32) % NUM_COLORS as i32;

    let mut out = Vec::with_capacity(lines.len());

    for line in lines {
        let bytes = line.as_bytes();
        let mut i = 0usize;
        let mut idx = 0i32;

        let mut buf = String::with_capacity(bytes.len() * 8);

        while i < bytes.len() {
            if bytes[i] == 0x1B && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
                let start = i;
                i = skip_ansi(bytes, i);
                buf.push_str(&line[start..i]);
                continue;
            }

            let len = utf8_len(bytes[i]);
            let mut c =
            ((line_offset as f32 + idx as f32 * gradient) as i32)
            % NUM_COLORS as i32;
            if c < 0 {
                c += NUM_COLORS as i32;
            }

            buf.push_str(COLORS[c as usize]);
            buf.push_str(&line[i..i + len]);

            i += len;
            idx += 1;
        }

        buf.push_str(RESET);
        out.push(buf);

        gradient += 0.015;
        if gradient > 1.2 {
            gradient = 0.3;
        }

        line_offset += 1;
    }

    out
}
