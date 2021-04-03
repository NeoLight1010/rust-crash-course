pub fn run() {
    let mut count = 0;

    // Simple loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 5 { break }
    }

    // While loop
    count = 0;
    while count <= 50 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    println!();
    
    // For loop in range
    for x in 0..25 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}