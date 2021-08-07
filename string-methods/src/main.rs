fn main() {
    // replace
    {
        let my_string = String::from("Rust is fantastic");
        println!("{}", my_string.replace("fantastic", "great"))
    }
    // lines
    {
        let my_string = String::from("Rust is fantastic\n yo\n dude");
        for line in my_string.lines() {
            println!("{}", line)
        }
    }
    // split
    {
        let my_string = String::from("wasssup");
        let tokens: Vec<&str> = my_string.split("s").collect();
        println!("{}", tokens[0])
    }
    // trim
    {
        let my_string = String::from("wasssup my gu ys                ");
        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim())
    }
    // char
    {
        let my_string = String::from("this is good");

        match my_string.chars().nth(3) {
            Some(c) => println!("at index 4: {}", c),
            None => println!("not character at 4"),
        }
    }
}
