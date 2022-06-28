//conditionals
//used to check the condition of something and act on the result

use crate::print;

pub fn run() {
    let age: u8 = 30;
    let check_id: bool = true;
    let knows_person: bool = true;
    //if//else
    if age >= 21 && check_id || knows_person && age >= 21 {
        println!("What would you like to drink ?");
    } else if age < 21 && check_id {
        println!("Sorry you have to leave");
    } else {
        println!("Show me your id");
    }

    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age {}", is_of_age);
}