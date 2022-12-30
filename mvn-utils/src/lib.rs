pub fn hex_char_to_u8(string: &str) -> u8 {
    let char = string.chars().next().expect("Input string should contain at least one character.");
    if !char.is_ascii_hexdigit() { panic!("Input is not a valid ASCII hex digit."); }
    // If the char is ASCII, we can safely interpret it as a byte
    let char = char as u8;
    if char <= 0x39 { // Numbers 0..=9
        char - 0x30
    } else if char <= 0x46 { // Upper case letters A..=F
        char - 0x41 + 10
    // We know the char is a hex digit, so no more checks are necessary
    } else { // Lower case letters a..=f
        char - 0x61 + 10
    }
}
