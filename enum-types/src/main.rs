enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("it is up"),
        Direction::Down => println!("it is down"),
        Direction::Left => println!("it is left"),
        Direction::Right => println!("it is right"),
    }
}
