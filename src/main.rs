mod ascii;
mod info;
mod render;

fn main() {
    let ascii = ascii::load_ascii("src/proto");
    let info_lines = info::all();

    render::print(&ascii, &info_lines);
}
