mod print;
mod vars;
mod types;

fn main() {
    print::run();
    println!();
    vars::run();
    println!();
    types::run();
}
