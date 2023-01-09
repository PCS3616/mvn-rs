pub mod error;

use error::{Span, LocatedIResult};
use nom::character::complete::{hex_digit1, satisfy};
use nom::combinator::{map, recognize};
use nom::multi::many_m_n;
use nom::character::complete::{space0, char, not_line_ending};
use nom::combinator::opt;
use nom::sequence::{preceded, tuple};
use num_traits::Num;

/*
 * Adapted from https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#hexadecimal
 */
pub fn hexadecimal<T: Num>(input: Span) -> LocatedIResult<'_, T> {
    map(
        recognize(hex_digit1),
        /* Unwrapping is not allowed since `<T as Num>::FromStrRadixErr`
         * doesn't implement `Debug`. We use `unwrap_or` as a workaround,
         * knowing the default value will never be used since nom
         * will return an `Err` if the conversion is not possible
         */
        |out: Span| T::from_str_radix(&out, 16).unwrap_or(T::zero()),
    )(input)
}

pub fn ascii(input: Span) -> LocatedIResult<'_, u32> {
    let (rest, bytes) = map(
        // ASCII immediates may contain at most two bytes
        // `many_m_n` ensures there are either 1 or 2 bytes
        many_m_n(1, 2, satisfy(|c: char| c.is_ascii())),
        |s: Vec<char>| s,
    )(input)?;
    let bytes = bytes.iter().map(|c| *c as u8);

    let mut result = 0;
    for (i, byte) in bytes.rev().enumerate() {
        result += (byte as u32) << (8 * i);
    }
    Ok((rest, result))
}

pub fn hex_char_to_u8(string: &str) -> u8 {
    let char = string
        .chars()
        .next()
        .expect("Input string should contain at least one character.");
    if !char.is_ascii_hexdigit() {
        panic!("Input is not a valid ASCII hex digit.");
    }
    // If the char is ASCII, we can safely interpret it as a byte
    let char = char as u8;
    if char <= 0x39 {
        // Numbers 0..=9
        char - 0x30
    } else if char <= 0x46 {
        // Upper case letters A..=F
        char - 0x41 + 10
    // We know the char is a hex digit, so no more checks are necessary
    } else {
        // Lower case letters a..=f
        char - 0x61 + 10
    }
}

pub fn comment_or_space(input: error::Span) -> error::LocatedIResult<Option<&str>> {
    let (rest, matched) = preceded(
        space0,
        opt(preceded(
            tuple((char(';'), space0)),
            not_line_ending
        )),
    )(input)?;
    let matched = match matched {
        Some(span) => Some(*span),
        None => None,
    };
    Ok((rest, matched))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
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

    #[test]
    fn should_parse_valid_ascii() {
        let inputs_outputs = vec![
            ("0", 0x0030),
            ("9", 0x0039),
            ("A", 0x0041),
            ("F", 0x0046),
            ("!", 0x0021),
            ("?", 0x003F),
            ("09", 0x3039),
        ];
        for (input, output) in inputs_outputs {
            assert_eq!(ascii(Span::new(input)).unwrap().1, output);
        }
        assert!(ascii(Span::new("ó")).is_err());
        assert!(ascii(Span::new("\u{80}")).is_err());
    }

    #[test]
    fn should_parse_no_comment() {
        assert!(comment_or_space(Span::new("")).is_ok())
    }

    #[test]
    fn should_parse_comment_with_varying_spacing() {
        let comment = Some("FOO");
        assert_eq!(comment_or_space(";FOO".into()).unwrap().1, comment);
        assert_eq!(comment_or_space(" ;FOO".into()).unwrap().1, comment);
        assert_eq!(comment_or_space("; FOO".into()).unwrap().1, comment);
        assert_eq!(comment_or_space(" ; FOO".into()).unwrap().1, comment);
    }

    #[test]
    fn should_parse_normal_comment() {
        let comments = vec![
            "Foo",
            "bar",
            "Um comentário",
            "Numb3rs s2",
            "Speci@l ch@racters!",
        ];
        for comment in comments {
            assert_eq!(
                comment_or_space(Span::new(format!(";{comment}").as_str())).unwrap().1,
                Some(comment),
            );
        }
    }

    #[test]
    fn should_parse_symbol_table_comment() {
        let comments = vec![
            "> EXPORT",
            "< IMPORT",
        ];
        for comment in comments {
            assert_eq!(
                comment_or_space(Span::new(format!(";{comment}").as_str())).unwrap().1,
                Some(comment),
            );
        }
    }
}
