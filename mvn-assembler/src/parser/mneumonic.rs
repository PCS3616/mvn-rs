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
            NormalMneumonic::parse("JP"),
            Ok(("", NormalMneumonic::Jump))
        );
        assert_eq!(
            NormalMneumonic::parse("JZ"),
            Ok(("", NormalMneumonic::JumpIfZero))
        );
        assert_eq!(
            NormalMneumonic::parse("JN"),
            Ok(("", NormalMneumonic::JumpIfNegative))
        );
        assert_eq!(
            NormalMneumonic::parse("LV"),
            Ok(("", NormalMneumonic::LoadValue))
        );
        assert_eq!(NormalMneumonic::parse("AD"), Ok(("", NormalMneumonic::Add)));
        assert_eq!(
            NormalMneumonic::parse("SB"),
            Ok(("", NormalMneumonic::Subtract))
        );
        assert_eq!(
            NormalMneumonic::parse("ML"),
            Ok(("", NormalMneumonic::Multiply))
        );
        assert_eq!(
            NormalMneumonic::parse("DV"),
            Ok(("", NormalMneumonic::Divide))
        );
        assert_eq!(
            NormalMneumonic::parse("LD"),
            Ok(("", NormalMneumonic::Load))
        );
        assert_eq!(
            NormalMneumonic::parse("MM"),
            Ok(("", NormalMneumonic::Memory))
        );
        assert_eq!(
            NormalMneumonic::parse("SC"),
            Ok(("", NormalMneumonic::Subroutine))
        );
        assert_eq!(
            NormalMneumonic::parse("RS"),
            Ok(("", NormalMneumonic::ReturnFromSubrotine))
        );
        assert_eq!(
            NormalMneumonic::parse("HM"),
            Ok(("", NormalMneumonic::HaltMachine))
        );
        assert_eq!(
            NormalMneumonic::parse("GD"),
            Ok(("", NormalMneumonic::GetData))
        );
        assert_eq!(
            NormalMneumonic::parse("PD"),
            Ok(("", NormalMneumonic::PutData))
        );
        assert_eq!(
            NormalMneumonic::parse("OS"),
            Ok(("", NormalMneumonic::OperatingSystem))
        );
    }

    #[test]
    fn should_return_value() {
        assert_eq!(u8::from(NormalMneumonic::Jump), 0x0);
        assert_eq!(u8::from(NormalMneumonic::JumpIfZero), 0x1);
        assert_eq!(u8::from(NormalMneumonic::JumpIfNegative), 0x2);
        assert_eq!(u8::from(NormalMneumonic::LoadValue), 0x3);
        assert_eq!(u8::from(NormalMneumonic::Add), 0x4);
        assert_eq!(u8::from(NormalMneumonic::Subtract), 0x5);
        assert_eq!(u8::from(NormalMneumonic::Multiply), 0x6);
        assert_eq!(u8::from(NormalMneumonic::Divide), 0x7);
        assert_eq!(u8::from(NormalMneumonic::Load), 0x8);
        assert_eq!(u8::from(NormalMneumonic::Memory), 0x9);
        assert_eq!(u8::from(NormalMneumonic::Subroutine), 0xA);
        assert_eq!(u8::from(NormalMneumonic::ReturnFromSubrotine), 0xB);
        assert_eq!(u8::from(NormalMneumonic::HaltMachine), 0xC);
        assert_eq!(u8::from(NormalMneumonic::GetData), 0xD);
        assert_eq!(u8::from(NormalMneumonic::PutData), 0xE);
        assert_eq!(u8::from(NormalMneumonic::OperatingSystem), 0xF);
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
