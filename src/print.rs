pub fn run() {
    // Print to console
    println!("Hi from the print.rs file!");

    // Basic formatting
    println!("Numbers: {}, {}, {}", 1, 2, 3);

    // Positional formatting
    println!("One: {0}, three: {2}, two: {1}", 1, 2, 3);

    // Named parameters
    println!("Jorge: {jorge}, Michael: {michael}", michael="Michael", jorge="Jorge");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Math
    println!("1+2: {}", 1+2);
}