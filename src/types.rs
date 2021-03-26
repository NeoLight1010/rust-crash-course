pub fn run() {
    // Int32
    let x = 1;

    // Float64
    let y = 3.5;

    // Explicit type
    let z: i8 = 20;

    // Max sizes
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    let is_greater = 2 > 5;

    // Char
    let c1 = 'c';
    let c2 = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, c1, c2));
}