<<<<<<< HEAD
use std::fs::File;
use std::io::BufReader;

mod code;
mod file;
mod prelude;

fn main() {
    let mut fh = BufReader::new(File::open("test.mp4").unwrap());

    while let Ok(b) = file::RawBox::read(&mut fh) {
        println!("{b:?}");
    }
}
