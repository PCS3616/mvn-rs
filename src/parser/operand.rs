use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;

use super::label::Label;
use super::util::{hexadecimal, LocatedIResult, Span};

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Simbolic(Label<'a>),
    Numeric(u16),
}

impl<'a> Operand<'a> {
    pub fn parse(input: Span<'a>) -> LocatedIResult<Self> {
        alt((
            // numeric
            map(alt((preceded(tag("/"), hexadecimal),)), |value: u16| {
                Self::new_numeric(value)
            }),
            // simbolic
            map(Label::parse, |label| Self::new_simbolic(label)),
        ))(input)
    }

    pub fn new_numeric(value: u16) -> Self {
        Self::Numeric(value)
    }

    pub fn new_simbolic(label: Label<'a>) -> Self {
        Self::Simbolic(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_numeric() {
        assert_eq!(Operand::parse(Span::new("/000F")).unwrap().1, Operand::new_numeric(15));
        assert_eq!(Operand::parse(Span::new("/F")).unwrap().1, Operand::new_numeric(15));
    }

    #[test]
    fn should_parse_simbolic() {
        assert_eq!(
            Operand::parse(Span::new("label")).unwrap().1,
            Operand::new_simbolic(Label::new("label"))
        );
        assert!(Operand::parse(Span::new("1label")).is_err());
    }
}
