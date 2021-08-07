extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"(\w{5})").unwrap();

    let text = "danil";

    // println!("found match? {}", re.is_match(text))

    match re.captures(text) {
        Some(cap) => println!("Found Match: {}", &cap[0]),
        None => println!("no line"),
    }
}
