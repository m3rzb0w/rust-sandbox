/*
Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - use when you need to modify or own string data

*/

pub fn run() {
    let hello: &str = "Hello"; //primitive
    let mut bonjour: String = String::from("Bonjour");

    //Get length 
    println!("Length ==> {}", hello.len());
    println!("{}", hello);
    bonjour.push('!');
    bonjour.push_str(" Ca va ?");
    println!("Length ==> {}", bonjour.len());
    println!("{}", bonjour);
    //Capacity in bytes
    println!("Capacity in bytes  ==> {}",bonjour.capacity());
    //Contains
    println!("Contains 'va'  ==> {}",bonjour.contains("va"));
    //Check if empty
    println!("Is empty  ==> {}",bonjour.is_empty());
    //Replace
    println!("Replace  ==> {}",bonjour.replace("Bonjour", "Hey"));
    //Loop through string by whitespace
    for word in bonjour.split_whitespace() {
        println!("{}", word);
    }
    //Create string with capcacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    
}