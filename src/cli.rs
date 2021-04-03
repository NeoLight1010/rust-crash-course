use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let status = "100%";
    
    println!("Command: {:?}", command);
    
    if command == "hello" {
        let name = &args[2];
        println!("Hi {}. What's up?", name);
    }

    if command == "status" {
        println!("Status is: {}", status);
    }
}