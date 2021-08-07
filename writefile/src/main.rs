use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("could not create file");

    file.write_all(b"welcome to the repo")
        .expect("could not write");
}
