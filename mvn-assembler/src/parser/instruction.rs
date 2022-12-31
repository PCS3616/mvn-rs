use nom::{branch::alt, combinator::map};
use types;
use types::mneumonic;
use utils::error_or;

use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for types::Instruction {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let mneumonic = alt((
            map(mneumonic::NormalMneumonic::parse, |o| Self::Normal(o)),
            map(mneumonic::PositionalMneumonic::parse, |o| {
                Self::Positional(o)
            }),
            map(mneumonic::RelationalMneumonic::parse, |o| {
                Self::Relational(o)
            }),
        ))(input);

        error_or!(mneumonic, input, "invalid mneumonic")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::mneumonic::*;
    use types::*;

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
            assert_eq!(Instruction::parse(Span::new(input)).unwrap().1, output,);
        }
    }
}
