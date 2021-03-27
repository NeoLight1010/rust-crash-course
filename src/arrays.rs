// Arrays have a fixed size and can only hold one data type.
pub fn run() {
    let mut numbers: [i8; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    // Change a number
    println!("Changing a number!");
    numbers[2] = 7;
    println!("{:?}", numbers);

    // Get length
    println!("Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Slice array
    let slice: &[i8] = &numbers[0..2];
    println!("Slice: {:?}", slice)
}