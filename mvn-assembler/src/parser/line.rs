use nom::character::complete::{space0, space1};
use nom::combinator::opt;
use nom::sequence::{delimited, tuple, terminated};
use types;
use types::Line;

use super::comment_or_space;
use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for Line<'a> {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
        delimited(
            space0,
            tuple((
                opt(
                    terminated(types::Label::parse, space1)
                ),
                types::Operation::parse,
            )),
            comment_or_space,
        )(input)
        .and_then(|(remainder, (label, operation))| Ok((remainder, Self::new(label, operation))))
        .map_err(
            |e| match e {
                nom::Err::Error(e) | nom::Err::Failure(e) => nom::Err::Failure(e),
                nom::Err::Incomplete(e) => nom::Err::Incomplete(e),
            }
        )
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
