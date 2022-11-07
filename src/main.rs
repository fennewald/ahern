mod code;
mod file;
mod prelude;
pub mod util;

fn main() {
    file::parse_file("test.mp4").unwrap();
}
