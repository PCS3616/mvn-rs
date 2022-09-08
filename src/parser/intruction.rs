use super::{mneumonic::Mneumonic, pseudo_instruction::PseudoInstruction};
use nom::{combinator::map, branch::alt, IResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Real(Mneumonic),
    Pseudo(PseudoInstruction)
}

impl Instruction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(Mneumonic::parse, |o| Self::Real(o)),
            map(PseudoInstruction::parse, |o| Self::Pseudo(o)),
        ))(input)
    }
    
    pub fn to_str(&self) -> &str {
        match self {
            Self::Real(intr) => intr.to_str(),
            Self::Pseudo(intr) => intr.to_str(),
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_instructions() {
        assert_eq!(Instruction::parse("JP"), Ok(("", Instruction::Real(Mneumonic::Jump))));

        assert_eq!(Instruction::parse("@"), Ok(("", Instruction::Pseudo(PseudoInstruction::AbsolutePosition))));
    }
}

