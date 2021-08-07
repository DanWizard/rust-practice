extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
}

fn main() {
    let json_str = r#"
        {
            "name" : "Daniel",
            "age" : 22,
            "is_male" : true
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        // let p: JsonValue = res.unwrap();
        let p: Person = res.unwrap();
        println!("name {}", p.name)
    } else {
        println!("sorry couldd not pass")
    }
}
