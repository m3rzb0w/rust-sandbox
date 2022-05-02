pub fn run() {
    // Print to console
    println!("Hey from the print.rs file! ;)");
    
    // Basic formatting
    println!("Number {}", 1);
    println!("{} means {}", "こんにちは", "Hello");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}.", "John", "Mexico", "Code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "Erlich", activity = "VC");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debut trait
    println!("{:?}", (12, true, "hey!"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}