mod ascii;
mod info;
mod render;
mod acecat;
mod lolcat;

fn main() {
    let ascii = ascii::load_ascii("/usr/local/bin/proto");

    let info_lines = lolcat::render_lines(&info::all());

    render::print(&ascii, &info_lines);

}
