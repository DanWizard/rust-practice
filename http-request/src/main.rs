extern crate reqwest;

// impl Display for Re

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    match reqwest::get("https://www.youtube.com/watch?v=xYoESR1aEQk&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=38"){
        Ok(mut response) => {
            // if &response.status() == reqwest::StatusCode::Ok{


            // }else {
            //     println!("bad request");
            // }
            // let mut other ;
            
            // match response.headers().get("Content Type"){
            //     Some(c) => other = c,
            //     None => {other = "ndad"}
            // }
            print_type_of(&response.headers());
            println!("{:#?}", &response.text())
        },
        Err(_) => println!("could not make reqwest")
    }
}
