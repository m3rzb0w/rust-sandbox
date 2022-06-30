mod print;
mod var;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditonals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

fn main() {
    println!("Hello, world!");
    println!("Hi Mom!");
    print::run();
    var::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditonals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();

}
