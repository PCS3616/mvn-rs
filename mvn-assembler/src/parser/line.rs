use nom::character::complete::{space0, space1};
use nom::combinator::opt;
use nom::sequence::{delimited, tuple, terminated};
use utils::{comment_or_space, types::Token};

use crate::types::{Line, Label, Operation};
use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for Line<'a> {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        delimited(
            space0,
            tuple((
                opt(
                    terminated(Token::<Label>::parse_assembler, space1)
                ),
                Operation::parse_assembler,
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
    use utils::types::Position;
    use crate::types::{*, mneumonic::*};
    use super::*;

    #[test]
    fn should_parse_without_label() {
        let inputs_outputs = [
            ("JP /0", 1, 4),
            ("       JP /0", 8, 11),
            ("       JP /0      ", 8, 11),
        ];
        for (input, instruction_column, operand_column) in inputs_outputs.into_iter() {
            assert_eq!(
                Line::parse_assembler(input.into()).unwrap().1,
                Line::new(
                    None,
                    Operation::new(
                        Token::new(Position::new(1, instruction_column), Instruction::Normal(NormalMneumonic::Jump)),
                        Token::new(Position::new(1, operand_column), Operand::new_numeric(0)),
                    )
                )
            );
        }
    }

    #[test]
    fn should_parse_with_label() {
        let inputs_outputs = [
            ("LOOP JP /0", 1, 6, 9),
            ("   LOOP JP /0", 4, 9, 12),
            ("   LOOP JP /0      ", 4, 9, 12),
        ];
        for (input, label_column, instruction_column, operand_column) in inputs_outputs.into_iter() {
            assert_eq!(
                Line::parse_assembler(input.into()).unwrap().1,
                Line::new(
                    Some(Token::new(Position::new(1, label_column), "LOOP".into())),
                    Operation::new(
                        Token::new(Position::new(1, instruction_column), Instruction::Normal(NormalMneumonic::Jump)),
                        Token::new(Position::new(1, operand_column), Operand::new_numeric(0)),
                    )
                )
            );
        }
    }

    #[test]
    fn should_parse_with_comment() {
        let inputs_outputs = [
            ("JP /0 ; Foo", None, 1, 4),
            ("LOOP JP /0 ; Bar", Some(1), 6, 9),
            ("   LOOP JP /0 ; Foobar", Some(4), 9, 12),
        ];
        for (input, label_column, instruction_column, operand_column) in inputs_outputs.into_iter() {
            let label = label_column.map(
                |column| Token::new(Position::new(1, column), "LOOP".into()),
            );
            assert_eq!(
                Line::parse_assembler(input.into()).unwrap().1,
                Line::new(
                    label,
                    Operation::new(
                        Token::new(Position::new(1, instruction_column), Instruction::Normal(NormalMneumonic::Jump)),
                        Token::new(Position::new(1, operand_column), Operand::new_numeric(0)),
                    )
                )
            );
        }
    }
}
