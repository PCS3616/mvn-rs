pub mod error;

use error::{Span, MvnParseError};
use nom::IResult;
use nom::character::complete::hex_digit1;
use nom::combinator::{map, recognize};
use num_traits::Num;

/*
 * Adapted from https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#hexadecimal
 */
pub fn hexadecimal<T: Num>(input: Span) -> IResult<Span<'_>, T, MvnParseError> {
    map(
        recognize(hex_digit1),
        /* Unwrapping is not allowed since `<T as Num>::FromStrRadixErr`
         * doesn't implement `Debug`. We use `unwrap_or` as a workaround,
         * knowing the default value will never be used since nom
         * will return an `Err` if the conversion is not possible
         */
        |out: Span| T::from_str_radix(&out, 16).unwrap_or(T::zero())
    )(input)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_valid_hexadecimals() {
        assert_eq!(hexadecimal::<u8>(Span::new("0")).unwrap().1, 0x0);
        assert_eq!(hexadecimal::<u8>(Span::new("9")).unwrap().1, 0x9);
        assert_eq!(hexadecimal::<u8>(Span::new("A")).unwrap().1, 0xA);
        assert_eq!(hexadecimal::<u8>(Span::new("F")).unwrap().1, 0xF);
        assert_eq!(hexadecimal::<u8>(Span::new("10")).unwrap().1, 0x10);
        assert_eq!(hexadecimal::<u8>(Span::new("FF")).unwrap().1, 0xFF);
        assert_eq!(hexadecimal::<u16>(Span::new("FFFF")).unwrap().1, 0xFFFF);
    }
}
