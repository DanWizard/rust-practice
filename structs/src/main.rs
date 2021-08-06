struct Color {
    red: u8, // 0 - 255
    green: u8,
    blue: u8,
}

fn main() {
    //  color: red, gree, and blue
    // is immutable unless define as mutable
    let bg = Color {
        red: 255,
        green: 100,
        blue: 15,
    };

    println!("{} {} {}", bg.red, bg.green, bg.blue)
}
