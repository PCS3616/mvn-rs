use nom::bytes::complete::take;
use nom_locate::position;

use crate::types::{Instruction, Operand, Operation, AddressPosition};
use utils::{hexadecimal, types::*};

use super::error;
use super::{Parse, Relocate};

impl<'a> Parse<'a> for Operation<'a> {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (operand, instruction) = take(1usize)(input)?;

        let (instruction, instruction_position) = position(instruction)?;
        let instruction_position = Position::from(instruction_position);
        let (_, instruction) = Instruction::parse_machine_code(instruction)?;
        let instruction = Token::new(instruction_position, instruction);

        let (operand, operand_position) = position(operand)?;
        let operand_position = Position::from(operand_position);
        let (rest, operand) = hexadecimal::<u32>(operand)?;
        let operand = Operand::new_numeric(operand);
        let operand = Token::new(operand_position, operand);

        Ok((rest, Self::new(instruction, operand)))
    }
}

impl Relocate for Operation<'_> {
    fn relocate(self, base: AddressPosition) -> Self {
        let operand = if let Operand::Numeric(operand) = self.operand.value {
            operand
        } else {
            // FIXME Add proper error treatment
            panic!("operand is not numeric")
        };

        Self::new(
            self.instruction,
            Token::new(self.operand.position, (base + operand).into()),
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use utils::types::*;
    use crate::types::{*, mneumonic::NormalMneumonic};
    use super::*;

    #[test]
    fn should_parse_instruction() {
        let inputs_outputs = vec![
            ("0000", (NormalMneumonic::Jump, 0x0)),
            ("4FFF", (NormalMneumonic::Add, 0xFFF)),
            ("8002", (NormalMneumonic::Load, 0x2)),
            ("9010", (NormalMneumonic::Memory, 0x10)),
        ];
        for (input, output) in inputs_outputs {
            let (mneumonic, operand)= output;
            assert_eq!(
                Operation::parse_machine_code(input.into()).unwrap().1,
                Operation::new(
                    Token::new(Position::new(1, 1), Instruction::Normal(mneumonic)),
                    Token::new(Position::new(1, 2), Operand::new_numeric(operand)),
                ),
            )
        }
    }

    #[test]
    fn should_reject_invalid_instruction() {
        assert!(Operation::parse_machine_code("G000".into()).is_err());
    }
}
