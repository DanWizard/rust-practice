struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height)
    }

    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

fn main() {
    let my_rect = Rectangle {
        width: 10,
        height: 5,
    };

    my_rect.print_description();

    if my_rect.is_square() {
        println!("is square");
    } else {
        println!("not square");
    }
}
