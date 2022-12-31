use nom::branch::alt;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};
use types;

use super::comment_or_space;
use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for types::Line<'a> {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
        delimited(
            space0,
            alt((
                // FIXME Can probably be replaced by alt
                map(types::Operation::parse, |operation| {
                    Self::new(None, operation)
                }),
                map(
                    separated_pair(types::Label::parse, space1, types::Operation::parse),
                    |(label, operation)| Self::new(Some(label), operation),
                ),
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
        let inputs_outputs = [
            ("JP /0", None),
            ("       JP /0", None),
            ("LOOP JP /0", Some(Label::new("LOOP"))),
            ("   LOOP JP /0", Some(Label::new("LOOP"))),
            ("   LOOP JP /0      ", Some(Label::new("LOOP"))),
            ("   LOOP JP /0 ; comment", Some(Label::new("LOOP"))),
        ];
        for (input, output_label) in inputs_outputs.into_iter() {
            assert_eq!(
                Line::parse(Span::new(input)).unwrap().1,
                Line::new(
                    output_label,
                    Operation::new(
                        Instruction::Normal(NormalMneumonic::Jump),
                        Operand::new_numeric(0)
                    )
                )
            );
        }
    }
}
