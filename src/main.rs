mod print;
mod vars;
mod types;
mod strings;

fn main() {
    print::run();
    println!();
    vars::run();
    println!();
    types::run();
    println!();
    strings::run();
}
