//arrays
//Fixed list where elements are the same data types

use::std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 9, 4, 12];
    println!("{:?}", numbers);
    
    //get single val
    println!("Single value {}", numbers[0]);

    let mut pokemon_names: [&str; 3] = ["pikachu", "charizard", "bulbasaur"];

    println!("{:?}", pokemon_names);

    pokemon_names[1] = "mew";

    println!("{:?}", pokemon_names);

    //array length
    println!("pokemon name array length is {}", pokemon_names.len());

    //stack allowed
    println!("Pokemon Array occupies {} bytes", mem::size_of_val(&pokemon_names));
    println!("Numbers Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);


}