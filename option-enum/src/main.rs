fn main() {
    let name = String::from("danielwq");

    println!(
        "Character at 7, {}",
        match name.chars().nth(7) {
            Some(c) => c.to_string(),
            None => "nothing".to_string(),
        }
    );

    match option("Daniel") {
        Some(c) => println!("{}", c),
        None => println!("no match"),
    }
}

fn option(name: &str) -> Option<&str> {
    match name {
        "Daniel" => Some("daniel"),
        "Michael" => Some("other"),
        _ => None,
    }
}
