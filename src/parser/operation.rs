use nom::{IResult, sequence::separated_pair, character::complete::space1, combinator::map};

use super::{operand::Operand, intruction::Instruction};

#[derive(Debug, PartialEq)]
pub struct Operation<'a>{
    mneumonic: Instruction,
    operand: Operand<'a>
}

impl<'a> Operation<'a> {
    pub fn new(mneumonic: Instruction, operand: Operand<'a>) -> Self {
        Self{mneumonic, operand}
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            separated_pair(Instruction::parse, space1, Operand::parse),
            |(mneumonic, operand)| Self::new(mneumonic, operand)
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{label::Label, mneumonic::Mneumonic, pseudo_instruction::PseudoInstruction};

    use super::*;

    #[test]
    fn should_parse_operation() {
        assert_eq!(Operation::parse("JP  /0"), Ok(("", Operation::new(Instruction::Real(Mneumonic::Jump), Operand::new_numeric(0)))));
        assert_eq!(Operation::parse("AD VAR"), Ok(("", Operation::new(Instruction::Real(Mneumonic::Add), Operand::new_simbolic(Label::new("VAR"))))));

        assert_eq!(Operation::parse("K /0"), Ok(("", Operation::new(Instruction::Pseudo(PseudoInstruction::Constant), Operand::new_numeric(0)))));
        assert_eq!(Operation::parse("# MAIN"), Ok(("", Operation::new(Instruction::Pseudo(PseudoInstruction::FontCodeEnd), Operand::new_simbolic(Label::new("MAIN"))))));
    }
}

