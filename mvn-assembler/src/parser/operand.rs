use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use types;

use super::hexadecimal;
use super::Parse;

impl<'a> Parse<'a> for types::Operand<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            // Numeric
            map(alt((preceded(tag("/"), hexadecimal),)), |value: u16| {
                Self::new_numeric(value)
            }),
            // Symbolic
            map(types::Label::parse, |label| Self::new_symbolic(label)),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::*;

    use super::*;

    #[test]
    fn should_parse_numeric() {
        assert_eq!(Operand::parse("/000F"), Ok(("", Operand::new_numeric(15))));
        assert_eq!(Operand::parse("/F"), Ok(("", Operand::new_numeric(15))));
    }

    #[test]
    fn should_parse_symbolic() {
        assert_eq!(
            Operand::parse("label"),
            Ok(("", Operand::new_symbolic(Label::new("label"))))
        );
        assert!(Operand::parse("1label").is_err());
    }
}
