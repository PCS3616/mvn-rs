use nom::branch::alt;
use nom::character::complete::{space0, space1, char, not_line_ending};
use nom::combinator::{map, value, opt};
use nom::IResult;
use nom::sequence::{separated_pair, delimited, pair};

use super::label::Label;
use super::operation::Operation;

#[derive(Debug, PartialEq)]
pub struct Line<'a>(Option<Label<'a>>, Operation<'a>);

impl<'a> Line<'a> {
    pub fn new(label: Option<Label<'a>>, operation: Operation<'a>) -> Self {
        Line(label, operation)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        delimited(
          space0, 
            alt(( // probabily can be replaced by alt
                map(
                    separated_pair(Label::parse, space1, Operation::parse),
                    |(label, operation)| Self::new(Some(label), operation)
                ),
                map(
                    Operation::parse,
                    |operation| Self::new(None, operation)
                )
            )),
          comment_or_space
        )(input)
    }

    pub fn unwrap(self) -> (Option<Label<'a>>, Operation<'a>) {
        (self.0, self.1)
    }
}


fn comment_or_space<'a>(input: &'a str) -> IResult<&'a str, ()> {
    value(
        (), // Output is thrown away.
        pair(
            space0,
            opt(
                pair(char('#'), not_line_ending)
            )
        )
    )(input)
}


#[cfg(test)]
mod tests {
    use crate::parser::{mneumonic::Mneumonic, operand::Operand, operation::Operation};

    use super::*;

    #[test]
    fn should_parse() {
        assert_eq!(
            Line::parse("JP /0"),
            Ok(("", Line(None, Operation::new(Mneumonic::Jump, Operand::new_numeric(0)))))
        );
        assert_eq!(
            Line::parse("     JP /0"),
            Ok(("", Line(None, Operation::new(Mneumonic::Jump, Operand::new_numeric(0)))))
        );
        assert_eq!(
            Line::parse("LOOP JP /0"),
            Ok(("", Line(Some(Label::new("LOOP")), Operation::new(Mneumonic::Jump, Operand::new_numeric(0)))))
        );
        assert_eq!(
            Line::parse("  LOOP JP /0"),
            Ok(("", Line(Some(Label::new("LOOP")), Operation::new(Mneumonic::Jump, Operand::new_numeric(0)))))
        );
        assert_eq!(
            Line::parse("  LOOP JP /0   "),
            Ok(("", Line(Some(Label::new("LOOP")), Operation::new(Mneumonic::Jump, Operand::new_numeric(0)))))
        );
        assert_eq!(
            Line::parse("  LOOP JP /0# comment"),
            Ok(("", Line(Some(Label::new("LOOP")), Operation::new(Mneumonic::Jump, Operand::new_numeric(0)))))
        );
    }
}
