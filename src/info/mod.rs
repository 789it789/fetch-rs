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

const SEP: &str = "   ";

fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

fn item(icon: &str, value: &str) -> String {
    format!("{} {}{}", icon, SEP, value)
}

pub fn all() -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("".to_string());

    lines.push(bold("\tHardware Information"));
    lines.push("".to_string());
    lines.push(item("󰍛", &cpu::model()));
    lines.push(item("󰾲", &gpu::model()));
    lines.push(item("󰋊", &disk::usage()));
    lines.push(item("󰑭", &memory::usage()));
    lines.push(item("󰍹", &resolution::current()));

    lines.push("".to_string());
    lines.push(bold("\tSoftware Information"));
    lines.push("".to_string());
    lines.push(item("", &os::name()));
    lines.push(item("`", &os::kernel()));
    lines.push(item("󰧨", &de::name()));
    lines.push(item("", &systemfont::get()));
    lines.push(item("󰏖", &packages::count()));
    lines.push(item("󰅐", &uptime::get()));

    lines.push("".to_string());

    let mut colors = String::from("  ");
    for i in 30..38 {
        colors.push_str(&format!("\x1b[{}m● \x1b[0m", i));
    }
    lines.push(colors);

    lines
}

