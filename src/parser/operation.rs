use nom::{IResult, sequence::separated_pair, character::complete::space1, combinator::map};

use super::{mneumonic::Mneumonic, operand::Operand};

#[derive(Debug, PartialEq)]
pub struct Operation<'a>{
    mneumonic: Mneumonic,
    operand: Operand<'a>
}

impl<'a> Operation<'a> {
    pub fn new(mneumonic: Mneumonic, operand: Operand<'a>) -> Self {
        Self{mneumonic, operand}
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            separated_pair(Mneumonic::parse, space1, Operand::parse),
            |(mneumonic, operand)| Self::new(mneumonic, operand)
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::label::Label;

    use super::*;

    #[test]
    fn should_parse_intruction() {
        assert_eq!(Operation::parse("JP  /0"), Ok(("", Operation::new(Mneumonic::Jump, Operand::new_numeric(0)))));
        assert_eq!(Operation::parse("AD VAR"), Ok(("", Operation::new(Mneumonic::Add, Operand::new_simbolic(Label::new("VAR"))))));
    }
}

