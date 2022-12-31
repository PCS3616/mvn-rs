use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::sequence::preceded;
use types;
use utils::error_or;
use utils::hexadecimal;

use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for types::Operand<'a> {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let numeric_operand = map(
            alt((
                // Numeric: hexadecimal
                preceded(tag("/"), hexadecimal),
                // Numeric: decimal
                preceded(tag("="), complete::u16),
            )),
            |value: u16| Self::new_numeric(value),
        )(input);
        let numeric_operand =
            error_or!(numeric_operand, input, "could not parse numeric immediate");

        let symbolic_operand = map(types::Label::parse, |label| Self::new_symbolic(label))(input);
        // `types::Label::parse` already returns a custom error

        if let Err(_) = numeric_operand {
            symbolic_operand
        } else {
            numeric_operand
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::*;

    use super::*;

    #[test]
    fn should_parse_hexadecimal() {
        assert_eq!(
            Operand::parse(Span::new("/000F")).unwrap().1,
            Operand::new_numeric(15)
        );
        assert_eq!(
            Operand::parse(Span::new("/F")).unwrap().1,
            Operand::new_numeric(15)
        );
        assert_eq!(
            Operand::parse(Span::new("/10")).unwrap().1,
            Operand::new_numeric(16)
        );
    }

    #[test]
    fn should_parse_decimal() {
        assert_eq!(
            Operand::parse(Span::new("=0")).unwrap().1,
            Operand::new_numeric(0)
        );
        assert_eq!(
            Operand::parse(Span::new("=00")).unwrap().1,
            Operand::new_numeric(0)
        );
        assert_eq!(
            Operand::parse(Span::new("=10")).unwrap().1,
            Operand::new_numeric(10)
        );
    }

    #[test]
    fn should_parse_symbolic() {
        assert_eq!(
            Operand::parse(Span::new("label")).unwrap().1,
            Operand::new_symbolic(Label::new("label"))
        );
        assert!(Operand::parse(Span::new("1label")).is_err());
    }
}
