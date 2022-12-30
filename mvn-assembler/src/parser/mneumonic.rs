use dotenv_codegen::dotenv;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;
use types;

use super::Parse;

impl Parse<'_> for types::mneumonic::NormalMneumonic {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
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
        ))(input)
    }
}

impl Parse<'_> for types::mneumonic::PositionalMneumonic {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
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
        ))(input)
    }
}

impl Parse<'_> for types::mneumonic::RelationalMneumonic {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Export, tag(Self::Export.to_string().as_str())),
            value(Self::Import, tag(Self::Import.to_string().as_str())),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::mneumonic::*;

    use super::*;

    #[test]
    fn should_parse_mneumonic() {
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_JUMP")),
            Ok(("", NormalMneumonic::Jump))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_JUMP_IF_ZERO")),
            Ok(("", NormalMneumonic::JumpIfZero))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_JUMP_IF_NEGATIVE")),
            Ok(("", NormalMneumonic::JumpIfNegative))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_LOAD_VALUE")),
            Ok(("", NormalMneumonic::LoadValue))
        );
        assert_eq!(NormalMneumonic::parse(dotenv!("MNEUMONIC_ADD")), Ok(("", NormalMneumonic::Add)));
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_SUBTRACT")),
            Ok(("", NormalMneumonic::Subtract))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_MULTIPLY")),
            Ok(("", NormalMneumonic::Multiply))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_DIVIDE")),
            Ok(("", NormalMneumonic::Divide))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_LOAD")),
            Ok(("", NormalMneumonic::Load))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_MEMORY")),
            Ok(("", NormalMneumonic::Memory))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_SUBROUTINE")),
            Ok(("", NormalMneumonic::Subroutine))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_RETURN_FROM_SUBROTINE")),
            Ok(("", NormalMneumonic::ReturnFromSubrotine))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_HALT_MACHINE")),
            Ok(("", NormalMneumonic::HaltMachine))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_GET_DATA")),
            Ok(("", NormalMneumonic::GetData))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_PUT_DATA")),
            Ok(("", NormalMneumonic::PutData))
        );
        assert_eq!(
            NormalMneumonic::parse(dotenv!("MNEUMONIC_OPERATING_SYSTEM")),
            Ok(("", NormalMneumonic::OperatingSystem))
        );
    }

    #[test]
    fn should_parse_constant_assignment() {
        assert_eq!(
            NormalMneumonic::parse("K"),
            Ok(("", NormalMneumonic::SetConstant))
        );
    }

    #[test]
    fn should_parse_positional_pseudo_mneumonic() {
        assert_eq!(
            PositionalMneumonic::parse("@"),
            Ok(("", PositionalMneumonic::SetAbsoluteOrigin))
        );
        assert_eq!(
            PositionalMneumonic::parse("&"),
            Ok(("", PositionalMneumonic::SetRelocatableOrigin))
        );
        assert_eq!(
            PositionalMneumonic::parse("$"),
            Ok(("", PositionalMneumonic::ReserveMemory))
        );
        assert_eq!(
            PositionalMneumonic::parse("#"),
            Ok(("", PositionalMneumonic::SetEnd))
        );
    }

    #[test]
    fn should_parse_relational_pseudo_mneumonic() {
        assert_eq!(
            RelationalMneumonic::parse(">"),
            Ok(("", RelationalMneumonic::Export))
        );
        assert_eq!(
            RelationalMneumonic::parse("<"),
            Ok(("", RelationalMneumonic::Import))
        );
    }
}
