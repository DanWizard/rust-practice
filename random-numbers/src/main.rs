extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11); // 1 - 10

    println!("Random number is {}", random_number);

    // Flip a coin
    let random_bool = rand::thread_rng().gen_weighted_bool(5);

    println!("Random Bool is {}", random_bool)
}
