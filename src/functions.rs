pub fn run() {
    greeting("Hello", "Anthony");

    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // Closure
    let substract_nums = |n1: i32, n2:i32| n1 - n2;
    println!("Substract: {}", substract_nums(5, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(a: i32, b: i32) -> i32 {
    // Return values don't need a semicolon
    a + b
}