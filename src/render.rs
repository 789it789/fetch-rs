use unicode_width::UnicodeWidthStr;

pub fn print(ascii_lines: &[String], info_lines: &[String]) {
    let ascii_width = ascii_lines
    .iter()
    .map(|line| UnicodeWidthStr::width(line.as_str()))
    .max()
    .unwrap_or(0);

    let gap = 5;

    let max_lines = ascii_lines.len().max(info_lines.len());

    for i in 0..max_lines {
        let ascii_line = if i < ascii_lines.len() {
            &ascii_lines[i]
        } else {
            ""
        };

        let mut ascii_display = ascii_line.to_string();
        let ascii_len = UnicodeWidthStr::width(ascii_display.as_str());

        if ascii_len < ascii_width {
            ascii_display.push_str(&" ".repeat(ascii_width - ascii_len));
        }

        print!("{}", ascii_display);

        print!("{}", " ".repeat(gap));

        if i < info_lines.len() {
            println!("{}", info_lines[i].trim_start());
        } else {
            println!();
        }
    }
}
