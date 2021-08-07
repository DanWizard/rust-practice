fn main() {
    let number = 2;

    match number {
        1 => println!("It is 1!"),
        2 => println!("It is 2!"),
        3 => println!("It is 3!"),
        7 | 8 => println!("it is 7 or 8!"),
        _ => println!("none"),
    };
}
