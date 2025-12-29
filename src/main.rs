mod ascii;
mod info;
mod render;

fn main() {
    let ascii = ascii::load_ascii("/usr/local/bin//proto");
    let info_lines = info::all();

    render::print(&ascii, &info_lines);
}
