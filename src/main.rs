mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod ref_pointers;

fn main() {
    print::run();
    println!();
    vars::run();
    println!();
    types::run();
    println!();
    strings::run();
    println!();
    tuples::run();
    println!();
    arrays::run();
    println!();
    vectors::run();
    println!();
    conditionals::run();
    println!();
    loops::run();
    println!();
    functions::run();
    println!();
    ref_pointers::run();
}
