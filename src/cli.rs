//cli

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command: String = args[1].clone();
        let name: &str = "Toto";
        println!("Args: {:?}", command);
        if command == "hello" {
            println!("Hi {}, whats up?", name);
        } else {
            println!("What do you mean by {} ?", command);
        }
    } else {
        println!("No args found!");
    }
}

//cargo run hello