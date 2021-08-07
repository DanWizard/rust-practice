struct Person {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    // speak
    fn speak(&self);
    // can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("hello my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        return self.age > 0;
    }
}

fn main() {
    let person = Person {
        name: String::from("Daniel"),
        age: 2,
    };

    if person.can_speak() {
        person.speak()
    }
}
