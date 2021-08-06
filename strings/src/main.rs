fn main() {
    // let my_string = String::from("How is it going? My name is Daniel");
    let my_string = "How is it going? My name is Daniel";

    println!("Length: {}", my_string.len());

    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token)
    }
}
