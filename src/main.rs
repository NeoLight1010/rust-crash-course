mod print;
mod vars;
mod types;
mod strings;
mod tuples;

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
}
