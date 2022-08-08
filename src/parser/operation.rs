use nom::{IResult, sequence::separated_pair, character::complete::space1, combinator::map};

use super::{mneumonic::Mneumonic, operand::Operand};

#[derive(Debug, PartialEq)]
pub struct Operation<'a>(Mneumonic, Operand<'a>);

impl<'a> Operation<'a> {
    pub fn new(mneumonic: Mneumonic, operand: Operand<'a>) -> Self {
        Operation(mneumonic, operand)
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
        assert_eq!(Operation::parse("JP  /0"), Ok(("", Operation(Mneumonic::Jump, Operand::Numeric(0)))));
        assert_eq!(Operation::parse("AD VAR"), Ok(("", Operation(Mneumonic::Add, Operand::Simbolic(Label::new("VAR"))))));
    }
}

