use nom::bytes::complete::take;
use nom_locate::position;

use types;
use utils::hexadecimal;

use super::error;
use super::{Parse, Position, Relocate};

impl<'a> Parse<'a> for types::Operation<'a> {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (operand, instruction) = take(1usize)(input)?;

        let (instruction, instruction_position) = position(instruction)?;
        let instruction_position = types::Position::from(instruction_position);
        let (_, instruction) = types::Instruction::parse_machine_code(instruction)?;
        let instruction = types::Token::new(instruction_position, instruction);

        let (operand, operand_position) = position(operand)?;
        let operand_position = types::Position::from(operand_position);
        let (rest, operand) = hexadecimal::<u32>(operand)?;
        let operand = types::Operand::new_numeric(operand);
        let operand = types::Token::new(operand_position, operand);

        Ok((rest, Self::new(instruction, operand)))
    }
}

impl Relocate for types::Operation<'_> {
    fn relocate(self, base: Position) -> Self {
        let operand = if let types::Operand::Numeric(operand) = self.operand {
            operand
        } else {
            // FIXME Add proper error treatment
            panic!("operand is not numeric")
        };

        Self::new(
            self.instruction,
            types::Operand::new_numeric(base + operand),
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::{Instruction, mneumonic::NormalMneumonic, Operand, Operation, Token, Position};
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
}
