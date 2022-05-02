// Variables are imutable by default

pub fn run() {
    let name = "Big Head";
    let mut age = 30; //add mut to modify a variable
    println!("His name is {} and He is {} years old.", name, age);
    age += 13;
    println!("His name is {} and He is {} years old.", name, age);

    // Define constant
    const ID : i32 = 1007;
    println!("My id is {}", ID);

    // Asign multiple variables
    let (my_name, my_age) = ("Jared", 33);
    println!("{} is {}", my_name, my_age);
}