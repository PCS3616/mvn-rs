use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::sequence::preceded;
use utils::error_or;
use utils::{ascii, hexadecimal};

use super::error::{LocatedIResult, Span};
use super::Parse;
use crate::types::{Label, Operand};

impl<'a> Parse<'a> for Operand<'a> {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let numeric_operand = map(
            alt((
                // Numeric: hexadecimal
                preceded(tag("/"), hexadecimal),
                // Numeric: decimal
                preceded(tag("="), complete::u32),
                // ASCII
                preceded(tag("\""), ascii),
            )),
            Self::new_numeric,
        )(input);
        let numeric_operand =
            error_or!(numeric_operand, input, "could not parse numeric immediate");

        let symbolic_operand = map(Label::parse_assembler, Self::new_symbolic)(input);
        // `types::Label::parse` already returns a custom error

        if numeric_operand.is_err() {
            symbolic_operand
        } else {
            numeric_operand
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_parse_hexadecimal() {
        assert_eq!(
            Operand::parse_assembler(Span::new("/000F")).unwrap().1,
            Operand::new_numeric(15)
        );
        assert_eq!(
            Operand::parse_assembler(Span::new("/F")).unwrap().1,
            Operand::new_numeric(15)
        );
        assert_eq!(
            Operand::parse_assembler(Span::new("/10")).unwrap().1,
            Operand::new_numeric(16)
        );
    }

    #[test]
    fn should_parse_decimal() {
        assert_eq!(
            Operand::parse_assembler(Span::new("=0")).unwrap().1,
            Operand::new_numeric(0)
        );
        assert_eq!(
            Operand::parse_assembler(Span::new("=00")).unwrap().1,
            Operand::new_numeric(0)
        );
        assert_eq!(
            Operand::parse_assembler(Span::new("=10")).unwrap().1,
            Operand::new_numeric(10)
        );
    }

    #[test]
    fn should_parse_ascii() {
        assert_eq!(
            Operand::parse_assembler(Span::new("\"0")).unwrap().1,
            Operand::new_numeric(0x30)
        );
        assert_eq!(
            Operand::parse_assembler(Span::new("\"00")).unwrap().1,
            Operand::new_numeric(0x3030)
        );
    }

    #[test]
    fn should_parse_symbolic() {
        assert_eq!(
            Operand::parse_assembler(Span::new("label")).unwrap().1,
            Operand::from(Label::from("label"))
        );
        assert!(Operand::parse_assembler(Span::new("1label")).is_err());
    }
}
