mod test {
    fn dog() {
        println!("dog")
    }
    pub fn print_message() {
        println!("message");
        dog()
    }

    pub mod drink {
        pub fn print_message() {
            println!("nested mod")
        }
    }
}

fn main() {
    test::print_message();
    test::drink::print_message()
}
