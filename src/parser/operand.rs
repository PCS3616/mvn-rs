use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use nom::bytes::complete::tag;

use super::label::Label;
use super::util::hexadecimal;

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Simbolic(Label<'a>),
    Numeric(u16),
}

impl<'a> Operand<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            // numeric
            map(
                alt((
                    preceded(
                        tag("/"),
                        hexadecimal
                    ),
                )),
                |value: u16| Self::new_numeric(value)
            ),
            // simbolic
            map(
                Label::parse,
                |label| Self::new_simbolic(label)
            )
          ))(input)
    }

    fn new_numeric(value: u16) -> Self {
        Self::Numeric(value)
    }
    
    fn new_simbolic(label: Label<'a>) -> Self {
        Self::Simbolic(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_numeric() {
        assert_eq!(Operand::parse("/000F"), Ok(("", Operand::Numeric(15))));
        assert_eq!(Operand::parse("/F"), Ok(("", Operand::Numeric(15))));
    }

    #[test]
    fn should_parse_simbolic() {
        assert_eq!(Operand::parse("label"), Ok(("", Operand::Simbolic(Label::new("label")))));
        assert!(Operand::parse("1label").is_err());
    }
}

