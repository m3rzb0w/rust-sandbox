//strutcs - used to create custom data types

//Traditionnal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Rgb(u8, u8, u8);

// with function
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    //construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


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

    //person
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Nanashi");
    println!("Full name  {}", p.full_name());

    println!("Person tuple  {:?}", p.to_tuple());


}