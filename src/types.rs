/*
--Primitive Types--

Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
- Number of bits they take in memory.
i = signed
u = unisgned, no negative value

Floats: f32, f64
Characters (char) = one character
Boolean
Tuples
Arrays = fix length
*/

// Rust is a statically typed language, it must know the types of all variables at compile time.

pub fn run() {
    //Default is i32
    let x = 1;
    //Default is f64
    let y = 2.5;
    //Add explicit type
    let z: i64 = 50004404333;

    //find max size
    println!("Max i32 => {}", std::i32::MAX);
    println!("Max i64 => {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    // Get Boolean from expression
    let is_greater: bool = 10 > 2;

    //Char single char, unicode ==> use single quote ''
    let a1 = 'a';
    let face = '\u{1F600}';


    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

}