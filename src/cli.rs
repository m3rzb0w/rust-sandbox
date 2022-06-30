//cli

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Toto";

    println!("Args: {:?}", command);

    if command == "hello" {
        println!("Hi {}, whats up?", name);
    } else {
        println!("No args found!");
    }
}

//cargo run hello