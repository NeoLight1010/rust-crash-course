// Variables are immutable by default!!

pub fn run() {
    let name = "Anthony";
    let mut age = 18;

    println!("Hello, my name is {} and I am {} years old.", name, age);

    println!("1 year has passed");
    age = age + 1;
    println!("Now I am {} years old!", age);

    // You need to define a type to create a const
    const ID: i32 = 20004;
    println!("ID: {}", ID);

    // Assign multiple vars at once
    let (name2, age2) = ("Juan", 40);
    println!("He is {}, and he is {} years old.", name2, age2);
}
