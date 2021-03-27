// Tuples can have a maximum of 12 elements!!
pub fn run() {
    let person: (&str, &str, i8) = ("Anthony", "Ecuador", 18);
    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}
