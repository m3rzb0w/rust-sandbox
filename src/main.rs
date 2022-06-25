mod print;
mod var;
mod types;
mod strings;

fn main() {
    println!("Hello, world!");
    println!("Hi Mom!");
    print::run();
    var::run();
    types::run();
    strings::run();
}
