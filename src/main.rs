use std::env;

mod char_count;

fn main() {
    let mut it = env::args();
    it.next();  // Ignore the program name

    // If it.next is None, there is not enough argument for this program
    match it.next() {
        Some(string) => char_count::print(char_count::count(string)),
        None            => println!("Too few arguments to run this program.")
    }
}
