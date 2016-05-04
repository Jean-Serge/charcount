use std::env;

fn main() {
    let mut chars: [u8; 128] = [0; 128];        // Initialize an array of 0 for counting chars appearance
    let mut it = env::args();
    it.next();
    let string = it.next().unwrap();      // Get the given string from command line

    // Count char occurency
    for c in string.into_bytes() {
        chars[c as usize] = chars[c as usize] + 1;
    }

    // Print char count
    for i in 0..chars.len() {
        if chars[i] == 0 {
            continue
        }
        println!("{0} appeared {1} times in the string.", i as u8 as char, chars[i]);
    }
}
