pub fn run() {
    let mut age = 16;

    if age < 18 {
        println!("You are underage! Leave!");
    } else {
        println!("Nice! Take your beer!");
    }

    age = 20;
    let has_id = false;
    let knows_person = true;
    if (age >= 18 && has_id) || knows_person {
        println!("You can go in!");
    } else {
        println!("Sorry. You have to have an ID AND be at least 18 years old.");
    }

    // Shorthand if
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is of age?: {}", is_of_age);
}