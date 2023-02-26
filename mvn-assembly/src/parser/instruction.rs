use nom::{branch::alt, combinator::map};
use utils::error_or;

use super::error::{LocatedIResult, Span};
use super::Parse;
use crate::types::{mneumonic, Instruction};

impl<'a> Parse<'a> for Instruction {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let mneumonic = alt((
            map(mneumonic::NormalMneumonic::parse_assembler, |o| {
                Self::Normal(o)
            }),
            map(mneumonic::PositionalMneumonic::parse_assembler, |o| {
                Self::Positional(o)
            }),
            map(mneumonic::RelationalMneumonic::parse_assembler, |o| {
                Self::Relational(o)
            }),
        ))(input);

        error_or!(mneumonic, input, "invalid mneumonic")
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{mneumonic::*, Instruction};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_parse_instructions() {
        let inputs_outputs = [
            ("JP", Instruction::Normal(NormalMneumonic::Jump)),
            (
                "@",
                Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin),
            ),
            (">", Instruction::Relational(RelationalMneumonic::Export)),
        ];
        for (input, output) in inputs_outputs.into_iter() {
            assert_eq!(
                Instruction::parse_assembler(Span::new(input)).unwrap().1,
                output,
            );
        }
    }
}
