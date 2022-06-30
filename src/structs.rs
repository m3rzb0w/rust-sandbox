//strutcs - used to create custom data types

//Traditionnal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Rgb(u8, u8, u8);



pub fn run() {
    let mut c: Color = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    println!("Color : {} {} {}", c.red, c.green, c.blue);
    c.blue = 89;
    println!("Color : {} {} {}", c.red, c.green, c.blue);

    let mut col = Rgb(255, 10, 3);
    println!("Color rgb struct : {} {} {}", col.0, col.1, col.2);
    col.2 = 95;
    println!("Color rgb struct : {} {} {}", col.0, col.1, col.2);

}