// Vectors are resizable arrays.
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    numbers[3] = 10;
    println!("{:?}", numbers);

    println!("Slice: {:?}", &numbers[0..2]);

    // Add to vector
    numbers.push(6);
    println!("{:?}", numbers);

    // Remove
    numbers.pop();
    println!("Popped: {:?}", numbers);

    // Loop
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}