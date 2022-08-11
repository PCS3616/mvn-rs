use std::collections::BTreeMap;

use nom::{IResult, sequence::separated_pair, character::complete::space1, combinator::map};

use super::{mneumonic::Mneumonic, operand::Operand, label::Label};

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

    pub fn value<'b>(&self, label_value: &BTreeMap<Label, u16>) -> Result<u16, String> {
        let operand_value = self.operand.value(&label_value)?;
        Ok(combine(self.mneumonic.value(), operand_value))
    }
}

fn combine(mneumonic_value: u8, label_value: u16) -> u16 {
    ((mneumonic_value as u16) << 12) | label_value
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

    #[test]
    fn should_generate_value() {
        assert_eq!(combine(0xF, 0x0DA), 0xF0DA)
    }

    #[test]
    fn should_return_label_value() {
        let labels = BTreeMap::from([(Label::new("label"), 13)]);
        let operation = Operation::new(Mneumonic::Add, Operand::new_simbolic(Label::new("label")));

        assert_eq!(
            operation.value(&labels),
            Ok(0x400D)
        );
    }
}

