fn main() {
    let x = 10;
    // can access data here
    {
        let y = 10;
        // isolated
        println!("x {} y {}", x, y)
    }
}
