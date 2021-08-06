fn main() {
    let tup1 = (20, "RUST", 3.2, false, (1, 2, 3));

    let (a, b, c, d, e) = tup1;

    println!("print element {}", (tup1.4).2);

    println!("a is {} b is {} c is {}", a, b, c);
}
