use nom::branch::alt;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use types;

use super::comment_or_space;
use super::Parse;

impl<'a> Parse<'a> for types::Line<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        delimited(
            space0,
            alt((
                // FIXME Can probably be replaced by alt
                map(
                    separated_pair(types::Label::parse, space1, types::Operation::parse),
                    |(label, operation)| Self::new(Some(label), operation),
                ),
                map(types::Operation::parse, |operation| {
                    Self::new(None, operation)
                }),
            )),
            comment_or_space,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::mneumonic::*;
    use types::*;

    use super::*;

    #[test]
    fn should_parse() {
        assert_eq!(
            Line::parse("JP /0"),
            Ok((
                "",
                Line::new(
                    None,
                    Operation::new(
                        Instruction::Normal(NormalMneumonic::Jump),
                        Operand::new_numeric(0)
                    )
                )
            ))
        );
        assert_eq!(
            Line::parse("     JP /0"),
            Ok((
                "",
                Line::new(
                    None,
                    Operation::new(
                        Instruction::Normal(NormalMneumonic::Jump),
                        Operand::new_numeric(0)
                    )
                )
            ))
        );
        assert_eq!(
            Line::parse("LOOP JP /0"),
            Ok((
                "",
                Line::new(
                    Some(Label::new("LOOP")),
                    Operation::new(
                        Instruction::Normal(NormalMneumonic::Jump),
                        Operand::new_numeric(0)
                    )
                )
            ))
        );
        assert_eq!(
            Line::parse("  LOOP JP /0"),
            Ok((
                "",
                Line::new(
                    Some(Label::new("LOOP")),
                    Operation::new(
                        Instruction::Normal(NormalMneumonic::Jump),
                        Operand::new_numeric(0)
                    )
                )
            ))
        );
        assert_eq!(
            Line::parse("  LOOP JP /0   "),
            Ok((
                "",
                Line::new(
                    Some(Label::new("LOOP")),
                    Operation::new(
                        Instruction::Normal(NormalMneumonic::Jump),
                        Operand::new_numeric(0)
                    )
                )
            ))
        );
        assert_eq!(
            Line::parse("  LOOP JP /0; comment"),
            Ok((
                "",
                Line::new(
                    Some(Label::new("LOOP")),
                    Operation::new(
                        Instruction::Normal(NormalMneumonic::Jump),
                        Operand::new_numeric(0)
                    )
                )
            ))
        );
    }
}
