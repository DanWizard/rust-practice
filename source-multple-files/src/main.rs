mod logic;
#[path = "./nest/nested.rs"]
mod nested;

fn main() {
    logic::print_message();
    nested::test();
}
