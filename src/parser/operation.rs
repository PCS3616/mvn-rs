use nom::{IResult, sequence::separated_pair, character::complete::space1, combinator::map};

use super::{operand::Operand, instruction::Instruction};

#[derive(Debug, PartialEq)]
pub struct Operation<'a>{
    pub instruction: Instruction,
    pub operand: Operand<'a>
}

impl<'a> Operation<'a> {
    pub fn new(instruction: Instruction, operand: Operand<'a>) -> Self {
        Self{instruction, operand}
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            separated_pair(Instruction::parse, space1, Operand::parse),
            |(instruction, operand)| Self::new(instruction, operand)
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{label::Label, mneumonic::{NormalMneumonic, PositionalMneumonic, RelationalMneumonic}};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_parse_operation() {
        assert_eq!(Operation::parse("< FUNC"), Ok(("", Operation::new(Instruction::Relational(RelationalMneumonic::Import), Operand::new_simbolic(Label::new("FUNC"))))));
        assert_eq!(Operation::parse("JP  /0"), Ok(("", Operation::new(Instruction::Normal(NormalMneumonic::Jump), Operand::new_numeric(0)))));
        assert_eq!(Operation::parse("AD VAR"), Ok(("", Operation::new(Instruction::Normal(NormalMneumonic::Add), Operand::new_simbolic(Label::new("VAR"))))));
        assert_eq!(Operation::parse("K /0"), Ok(("", Operation::new(Instruction::Normal(NormalMneumonic::SetConstant), Operand::new_numeric(0)))));
        assert_eq!(Operation::parse("# MAIN"), Ok(("", Operation::new(Instruction::Positional(PositionalMneumonic::SetEnd), Operand::new_simbolic(Label::new("MAIN"))))));
    }
}

