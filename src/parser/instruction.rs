use super::mneumonic::{Mneumonic, NormalMneumonic, PositionalMneumonic, RelationalMneumonic};
use nom::{combinator::map, branch::alt, IResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Normal(NormalMneumonic),
    Positional(PositionalMneumonic),
    Relational(RelationalMneumonic),
}

impl Instruction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
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
        assert_eq!(Instruction::parse("JP"), Ok(("", Instruction::Normal(NormalMneumonic::Jump))));
        assert_eq!(Instruction::parse("@"), Ok(("", Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin))));
        assert_eq!(Instruction::parse(">"), Ok(("", Instruction::Relational(RelationalMneumonic::Export))));
    }
}

