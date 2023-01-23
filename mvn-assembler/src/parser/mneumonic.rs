use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use utils::error_or;

use crate::types::mneumonic::*;
use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for NormalMneumonic {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<Self> {
        let mneumonic = alt((
            value(Self::Jump, tag(Self::Jump.to_string().as_str())),
            value(Self::JumpIfZero, tag(Self::JumpIfZero.to_string().as_str())),
            value(
                Self::JumpIfNegative,
                tag(Self::JumpIfNegative.to_string().as_str()),
            ),
            value(Self::LoadValue, tag(Self::LoadValue.to_string().as_str())),
            value(Self::Add, tag(Self::Add.to_string().as_str())),
            value(Self::Subtract, tag(Self::Subtract.to_string().as_str())),
            value(Self::Multiply, tag(Self::Multiply.to_string().as_str())),
            value(Self::Divide, tag(Self::Divide.to_string().as_str())),
            value(Self::Load, tag(Self::Load.to_string().as_str())),
            value(Self::Memory, tag(Self::Memory.to_string().as_str())),
            value(Self::Subroutine, tag(Self::Subroutine.to_string().as_str())),
            value(
                Self::ReturnFromSubrotine,
                tag(Self::ReturnFromSubrotine.to_string().as_str()),
            ),
            value(
                Self::HaltMachine,
                tag(Self::HaltMachine.to_string().as_str()),
            ),
            value(Self::GetData, tag(Self::GetData.to_string().as_str())),
            value(Self::PutData, tag(Self::PutData.to_string().as_str())),
            value(
                Self::OperatingSystem,
                tag(Self::OperatingSystem.to_string().as_str()),
            ),
            value(
                Self::SetConstant,
                tag(Self::SetConstant.to_string().as_str()),
            ),
        ))(input);

        error_or!(
            mneumonic,
            input,
            "mneumonic does not match that of any known instruction"
        )
    }
}

impl<'a> Parse<'a> for PositionalMneumonic {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let mneumonic = alt((
            value(
                Self::SetAbsoluteOrigin,
                tag(Self::SetAbsoluteOrigin.to_string().as_str()),
            ),
            value(
                Self::ReserveMemory,
                tag(Self::ReserveMemory.to_string().as_str()),
            ),
            value(Self::SetEnd, tag(Self::SetEnd.to_string().as_str())),
            value(
                Self::SetRelocatableOrigin,
                tag(Self::SetRelocatableOrigin.to_string().as_str()),
            ),
        ))(input);

        error_or!(
            mneumonic,
            input,
            "mneumonic does not match that of any known pseudo-instruction"
        )
    }
}

impl<'a> Parse<'a> for RelationalMneumonic {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let mneumonic = alt((
            value(Self::Export, tag(Self::Export.to_string().as_str())),
            value(Self::Import, tag(Self::Import.to_string().as_str())),
        ))(input);

        error_or!(
            mneumonic,
            input,
            "mneumonic does not match that of any known pseudo-instruction"
        )
    }
}

#[cfg(test)]
mod tests {
    use dotenv_codegen::dotenv;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn should_parse_mneumonic() {
        let inputs_outputs = [
            (dotenv!("MNEUMONIC_JUMP"), NormalMneumonic::Jump),
            (
                dotenv!("MNEUMONIC_JUMP_IF_ZERO"),
                NormalMneumonic::JumpIfZero,
            ),
            (
                dotenv!("MNEUMONIC_JUMP_IF_NEGATIVE"),
                NormalMneumonic::JumpIfNegative,
            ),
            (dotenv!("MNEUMONIC_LOAD_VALUE"), NormalMneumonic::LoadValue),
            (dotenv!("MNEUMONIC_ADD"), NormalMneumonic::Add),
            (dotenv!("MNEUMONIC_SUBTRACT"), NormalMneumonic::Subtract),
            (dotenv!("MNEUMONIC_MULTIPLY"), NormalMneumonic::Multiply),
            (dotenv!("MNEUMONIC_DIVIDE"), NormalMneumonic::Divide),
            (dotenv!("MNEUMONIC_LOAD"), NormalMneumonic::Load),
            (dotenv!("MNEUMONIC_MEMORY"), NormalMneumonic::Memory),
            (dotenv!("MNEUMONIC_SUBROUTINE"), NormalMneumonic::Subroutine),
            (
                dotenv!("MNEUMONIC_RETURN_FROM_SUBROTINE"),
                NormalMneumonic::ReturnFromSubrotine,
            ),
            (
                dotenv!("MNEUMONIC_HALT_MACHINE"),
                NormalMneumonic::HaltMachine,
            ),
            (dotenv!("MNEUMONIC_GET_DATA"), NormalMneumonic::GetData),
            (dotenv!("MNEUMONIC_PUT_DATA"), NormalMneumonic::PutData),
            (
                dotenv!("MNEUMONIC_OPERATING_SYSTEM"),
                NormalMneumonic::OperatingSystem,
            ),
        ];

        for (input, output) in inputs_outputs {
            assert_eq!(NormalMneumonic::parse_assembler(Span::new(input)).unwrap().1, output,);
        }
    }

    #[test]
    fn should_parse_constant_assignment() {
        assert_eq!(
            NormalMneumonic::parse_assembler(Span::new("K")).unwrap().1,
            NormalMneumonic::SetConstant
        );
    }

    #[test]
    fn should_parse_positional_pseudo_mneumonic() {
        assert_eq!(
            PositionalMneumonic::parse_assembler(Span::new("@")).unwrap().1,
            PositionalMneumonic::SetAbsoluteOrigin
        );
        assert_eq!(
            PositionalMneumonic::parse_assembler(Span::new("&")).unwrap().1,
            PositionalMneumonic::SetRelocatableOrigin
        );
        assert_eq!(
            PositionalMneumonic::parse_assembler(Span::new("$")).unwrap().1,
            PositionalMneumonic::ReserveMemory
        );
        assert_eq!(
            PositionalMneumonic::parse_assembler(Span::new("#")).unwrap().1,
            PositionalMneumonic::SetEnd
        );
    }

    #[test]
    fn should_parse_relational_pseudo_mneumonic() {
        assert_eq!(
            RelationalMneumonic::parse_assembler(Span::new(">")).unwrap().1,
            RelationalMneumonic::Export
        );
        assert_eq!(
            RelationalMneumonic::parse_assembler(Span::new("<")).unwrap().1,
            RelationalMneumonic::Import
        );
    }
}
