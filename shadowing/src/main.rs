fn main() {
    let mut x = 10;

    {
        let x = 15;
    }

    println!("{}", x)
}
