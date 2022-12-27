use super::mneumonic::{Mneumonic, NormalMneumonic, PositionalMneumonic, RelationalMneumonic};
use nom::{combinator::map, branch::alt};

use super::util::{LocatedIResult, Span};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Normal(NormalMneumonic),
    Positional(PositionalMneumonic),
    Relational(RelationalMneumonic),
}

impl Instruction {
    pub fn parse(input: Span) -> LocatedIResult<Self> {
        alt((
            map(NormalMneumonic::parse, |o| Self::Normal(o)),
            map(PositionalMneumonic::parse, |o| Self::Positional(o)),
            map(RelationalMneumonic::parse, |o| Self::Relational(o)),
        ))(input)
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Normal(mneumonic) => mneumonic.to_str(),
            Self::Positional(mneumonic) => mneumonic.to_str(),
            Self::Relational(mneumonic) => mneumonic.to_str(),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_instructions() {
        let inputs_outputs = [
            ("JP", Instruction::Normal(NormalMneumonic::Jump)),
            ("@", Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin)),
            (">", Instruction::Relational(RelationalMneumonic::Export)),
        ];
        for (input, output) in inputs_outputs.into_iter() {
            assert_eq!(
                Instruction::parse(Span::new(input)).unwrap().1,
                output,
            );
        }
    }
}
