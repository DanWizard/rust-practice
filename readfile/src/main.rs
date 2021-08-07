use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("info.txt").expect("cant open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Oops!, cannot read file...");

    println!("file contents:\n\n{}", contents)
}
