use std::env;

fn char_count(s: String) -> [u8; 128] {
    let mut chars: [u8; 128] = [0; 128];    // Initialize an array of 0 for counting chars appearance

    // Count char occurency
    for c in s.into_bytes() {
        chars[c as usize] = chars[c as usize] + 1;
    }

    chars
}

fn print_char_count(t: [u8; 128]) -> () {
    // Print char count
    for i in 0..t.len() {
        if t[i] == 0 {
            continue
        }
        println!("{0} appears {1} time.", i as u8 as char, t[i]);
    }
}

fn main() {
    let mut it = env::args();
    it.next();                          // Ignore the program name

    match it.next() {
        Some(string) => print_char_count(char_count(string)),
        None            => println!("Too few arguments to run this program.")
    }
}
