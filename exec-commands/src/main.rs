use std::process::Command;

fn main() {
    let mut cmd = Command::new("cargo");

    // Execute the command

    match cmd.output() {
        Ok(o) => unsafe { println!("Output: {}", String::from_utf8_unchecked(o.stdout)) },

        Err(_) => println!("this is it"),
    }
}
