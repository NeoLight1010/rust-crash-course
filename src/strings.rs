// Thee primitive str is immutable and fixed length.
// On the other hand, String is a data structure that is more manageable.

pub fn run() {
    let hello_world = String::from("Hello World!");

    println!("{}", hello_world);
    println!("Length: {}", hello_world.len());

    // You can push chars or strings to a mutable String
    let mut mutable = String::from("Hii ");
    mutable.push(' ');
    mutable.push_str("Anthony");
    println!("{}", mutable);
    println!("{:?}", (mutable.capacity(), mutable.is_empty(), mutable.contains("Hii")));

    // Replace in string
    mutable = mutable.replace("Anthony", "NeoLight");
    println!("{}", mutable);

   // Loop through split
   for word in mutable.split_whitespace() {
       println!("{}", word);
   }

   // Simple assertion
   let cap10 = String::with_capacity(10);
   assert_eq!(cap10.capacity(), 10);
}