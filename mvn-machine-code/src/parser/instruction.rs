use dotenv_codegen::dotenv;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;

use crate::types::{mneumonic::NormalMneumonic, Instruction};

use super::error;
use super::Parse;

impl<'a> Parse<'a> for Instruction {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (rest, mneumonic) = alt((
            value(NormalMneumonic::Jump, tag(dotenv!("VALUE_JUMP"))),
            value(
                NormalMneumonic::JumpIfZero,
                tag(dotenv!("VALUE_JUMP_IF_ZERO")),
            ),
            value(
                NormalMneumonic::JumpIfNegative,
                tag(dotenv!("VALUE_JUMP_IF_NEGATIVE")),
            ),
            value(NormalMneumonic::LoadValue, tag(dotenv!("VALUE_LOAD_VALUE"))),
            value(NormalMneumonic::Add, tag(dotenv!("VALUE_ADD"))),
            value(NormalMneumonic::Subtract, tag(dotenv!("VALUE_SUBTRACT"))),
            value(NormalMneumonic::Multiply, tag(dotenv!("VALUE_MULTIPLY"))),
            value(NormalMneumonic::Divide, tag(dotenv!("VALUE_DIVIDE"))),
            value(NormalMneumonic::Load, tag(dotenv!("VALUE_LOAD"))),
            value(NormalMneumonic::Memory, tag(dotenv!("VALUE_MEMORY"))),
            value(
                NormalMneumonic::Subroutine,
                tag(dotenv!("VALUE_SUBROUTINE")),
            ),
            value(
                NormalMneumonic::ReturnFromSubrotine,
                tag(dotenv!("VALUE_RETURN_FROM_SUBROTINE")),
            ),
            value(
                NormalMneumonic::HaltMachine,
                tag(dotenv!("VALUE_HALT_MACHINE")),
            ),
            value(NormalMneumonic::GetData, tag(dotenv!("VALUE_GET_DATA"))),
            value(NormalMneumonic::PutData, tag(dotenv!("VALUE_PUT_DATA"))),
            value(
                NormalMneumonic::OperatingSystem,
                tag(dotenv!("VALUE_OPERATING_SYSTEM")),
            ),
        ))(input)?;
        Ok((rest, Self::Normal(mneumonic)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_instruction() {
        let inputs_outputs = vec![
            ("0", NormalMneumonic::Jump),
            ("1", NormalMneumonic::JumpIfZero),
            ("2", NormalMneumonic::JumpIfNegative),
            ("3", NormalMneumonic::LoadValue),
            ("4", NormalMneumonic::Add),
            ("5", NormalMneumonic::Subtract),
            ("6", NormalMneumonic::Multiply),
            ("7", NormalMneumonic::Divide),
            ("8", NormalMneumonic::Load),
            ("9", NormalMneumonic::Memory),
            ("A", NormalMneumonic::Subroutine),
            ("B", NormalMneumonic::ReturnFromSubrotine),
            ("C", NormalMneumonic::HaltMachine),
            ("D", NormalMneumonic::GetData),
            ("E", NormalMneumonic::PutData),
            ("F", NormalMneumonic::OperatingSystem),
        ];
        for (input, output) in inputs_outputs {
            assert_eq!(
                Instruction::parse_machine_code(input.into()).unwrap().1,
                Instruction::Normal(output),
            )
        }
    }

    #[test]
    fn should_reject_invalid_instruction() {
        assert!(Instruction::parse_machine_code(";".into()).is_err());
        assert!(Instruction::parse_machine_code("@".into()).is_err());
        assert!(Instruction::parse_machine_code(">".into()).is_err());
        assert!(Instruction::parse_machine_code("<".into()).is_err());
        assert!(Instruction::parse_machine_code("G".into()).is_err());
    }
}
