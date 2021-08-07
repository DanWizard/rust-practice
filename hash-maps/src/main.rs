use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    marks.insert("Web", 87);
    marks.insert("DB", 65);
    marks.insert("Frontend", 23);

    println!("length of hashmap {}", marks.len());

    match marks.get("Rust Programming") {
        Some(mark) => println!("You got {} for web dev", mark),
        None => println!("you got nothing"),
    }

    // remove a value

    marks.remove("Web");

    // Loop through HashMap

    for (subject, mark) in &marks {
        println!("For {}, you got {}%", subject, mark);
    }

    // check for value

    println!("did you study c++? {}", marks.contains_key("c++"))
}
