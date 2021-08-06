fn main() {
    let mut x = 10;
    let xr = &x;
    let xb = &x;

    println!("x is {}", xr);
    println!("x is {}", xb);

    let xmr = &mut x;
    *xmr = 5;
    println!("x is {}", x);
    println!("x is {}", x);
}
