struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and my age is {}", self.name, self.age);
    }
}

fn main() {
    let daniel = Person {
        name: String::from("daniel"),
        age: 22,
    };

    println!("{}", daniel.to_string())
}
