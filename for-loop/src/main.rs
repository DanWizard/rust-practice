fn main() {
    let range = 1..=100;

    let vector = vec!["string", "that", "other"];
    for (index, v) in vector.iter().enumerate() {
        println!("number is {}, animal is {}", index, v)
    }
}
