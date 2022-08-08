use nom::branch::alt;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{preceded, separated_pair};

use super::label::Label;
use super::operation::Operation;

#[derive(Debug, PartialEq)]
pub struct Line<'a>(Option<Label<'a>>, Operation<'a>);

impl<'a> Line<'a> {
    pub fn new(label: Option<Label<'a>>, operation: Operation<'a>) -> Self {
        Line(label, operation)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        preceded(
            space0,
            alt((
                map(
                    separated_pair(Label::parse, space1, Operation::parse),
                    |(label, operation)| Self::new(Some(label), operation)
                ),
                map(
                    Operation::parse,
                    |operation| Self::new(None, operation)
                )
            ))
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{mneumonic::Mneumonic, operand::Operand, operation::Operation};

    use super::*;

    #[test]
    fn should_parse() {
        assert_eq!(
            Line::parse("JP /0"),
            Ok(("", Line(None, Operation::new(Mneumonic::Jump, Operand::Numeric(0)))))
        );
        assert_eq!(
            Line::parse("     JP /0"),
            Ok(("", Line(None, Operation::new(Mneumonic::Jump, Operand::Numeric(0)))))
        );
        assert_eq!(
            Line::parse("LOOP JP /0"),
            Ok(("", Line(Some(Label::new("LOOP")), Operation::new(Mneumonic::Jump, Operand::Numeric(0)))))
        );
        assert_eq!(
            Line::parse("  LOOP JP /0"),
            Ok(("", Line(Some(Label::new("LOOP")), Operation::new(Mneumonic::Jump, Operand::Numeric(0)))))
        );
    }
}
