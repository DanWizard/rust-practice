fn main() {
    print_numbers_to(10);

    if is_even(5) {
        println!("is even")
    } else {
        println!("not even")
    }
}

fn print_numbers_to(num: u32) {
    for n in 1..=num {
        println!("print {}", n)
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
