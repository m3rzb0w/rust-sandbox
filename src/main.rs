mod print;
mod var;
mod types;
mod strings;
mod tuples;
mod arrays;

fn main() {
    println!("Hello, world!");
    println!("Hi Mom!");
    print::run();
    var::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
}
