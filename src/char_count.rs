/// Build a 255 elements array containing the occurency number of each
/// character in the given String.
/// Each char is referenced in the array by his ASCII value.
pub fn count(s: String) -> [u8; 255] {
    let mut chars: [u8; 255] = [0; 255];

    for c in s.into_bytes() {
        chars[c as usize] = chars[c as usize] + 1;
    }

    chars
}

/// Print the content of the given array.
/// Will print a sentence like :
///     "{0} appears {1} times."
/// where {0} is the char corresponding to the u8 in array
/// and {1} is the occurency number of this char
///
/// This function will not print char that doesn't appear at least 1 time.
pub fn print(t: [u8; 255]) -> () {
    for i in 0..t.len() {
        if t[i] == 0 {
            continue
        }

        println!("{0} appears {1} times.", i as u8 as char, t[i]);
    }
}

