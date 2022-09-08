use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mneumonic {
    Jump,
    JumpIfZero,
    JumpIfNegative,
    LoadValue,
    Add,
    Subtract,
    Multiply,
    Divide,
    Load,
    Memory,
    Subroutine,
    ReturnFromSubrotine,
    HaltMachine,
    GetData,
    PutData,
    OperatingSystem
}

impl Mneumonic {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Jump, tag(Self::Jump.to_str())),
            value(Self::JumpIfZero, tag(Self::JumpIfZero.to_str())),
            value(Self::JumpIfNegative, tag(Self::JumpIfNegative.to_str())),
            value(Self::LoadValue, tag(Self::LoadValue.to_str())),
            value(Self::Add, tag(Self::Add.to_str())),
            value(Self::Subtract, tag(Self::Subtract.to_str())),
            value(Self::Multiply, tag(Self::Multiply.to_str())),
            value(Self::Divide, tag(Self::Divide.to_str())),
            value(Self::Load, tag(Self::Load.to_str())),
            value(Self::Memory, tag(Self::Memory.to_str())),
            value(Self::Subroutine, tag(Self::Subroutine.to_str())),
            value(Self::ReturnFromSubrotine, tag(Self::ReturnFromSubrotine.to_str())),
            value(Self::HaltMachine, tag(Self::HaltMachine.to_str())),
            value(Self::GetData, tag(Self::GetData.to_str())),
            value(Self::PutData, tag(Self::PutData.to_str())),
            value(Self::OperatingSystem, tag(Self::OperatingSystem.to_str())),
        ))(input)
    }
    
    pub fn to_str(&self) -> &str {
        match self {
            Self::Jump => "JP",
            Self::JumpIfZero => "JZ",
            Self::JumpIfNegative => "JN",
            Self::LoadValue => "LV",
            Self::Add => "AD",
            Self::Subtract => "SB",
            Self::Multiply => "ML",
            Self::Divide => "DV",
            Self::Load => "LD",
            Self::Memory => "MM",
            Self::Subroutine => "SC",
            Self::ReturnFromSubrotine => "RS",
            Self::HaltMachine => "HM",
            Self::GetData => "GD",
            Self::PutData => "PD",
            Self::OperatingSystem => "OS"
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::Jump => 0x0,
            Self::JumpIfZero => 0x1,
            Self::JumpIfNegative => 0x2,
            Self::LoadValue => 0x3,
            Self::Add => 0x4,
            Self::Subtract => 0x5,
            Self::Multiply => 0x6,
            Self::Divide => 0x7,
            Self::Load => 0x8,
            Self::Memory => 0x9,
            Self::Subroutine => 0xA,
            Self::ReturnFromSubrotine => 0xB,
            Self::HaltMachine => 0xC,
            Self::GetData => 0xD,
            Self::PutData => 0xE,
            Self::OperatingSystem => 0xF
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_mneumonic() {
        assert_eq!(Mneumonic::parse("JP"), Ok(("", Mneumonic::Jump)));
        assert_eq!(Mneumonic::parse("JZ"), Ok(("",  Mneumonic::JumpIfZero)));
        assert_eq!(Mneumonic::parse("JN"), Ok(("",  Mneumonic::JumpIfNegative)));
        assert_eq!(Mneumonic::parse("LV"), Ok(("",  Mneumonic::LoadValue)));
        assert_eq!(Mneumonic::parse("AD"), Ok(("",  Mneumonic::Add)));
        assert_eq!(Mneumonic::parse("SB"), Ok(("",  Mneumonic::Subtract)));
        assert_eq!(Mneumonic::parse("ML"), Ok(("",  Mneumonic::Multiply)));
        assert_eq!(Mneumonic::parse("DV"), Ok(("",  Mneumonic::Divide)));
        assert_eq!(Mneumonic::parse("LD"), Ok(("",  Mneumonic::Load)));
        assert_eq!(Mneumonic::parse("MM"), Ok(("",  Mneumonic::Memory)));
        assert_eq!(Mneumonic::parse("SC"), Ok(("",  Mneumonic::Subroutine)));
        assert_eq!(Mneumonic::parse("RS"), Ok(("",  Mneumonic::ReturnFromSubrotine)));
        assert_eq!(Mneumonic::parse("HM"), Ok(("",  Mneumonic::HaltMachine)));
        assert_eq!(Mneumonic::parse("GD"), Ok(("",  Mneumonic::GetData)));
        assert_eq!(Mneumonic::parse("PD"), Ok(("",  Mneumonic::PutData)));
        assert_eq!(Mneumonic::parse("OS"), Ok(("",  Mneumonic::OperatingSystem)));
    }

    #[test]
    fn should_return_value() {
            assert_eq!(Mneumonic::Jump.value(), 0x0);
            assert_eq!(Mneumonic::JumpIfZero.value(), 0x1);
            assert_eq!(Mneumonic::JumpIfNegative.value(), 0x2);
            assert_eq!(Mneumonic::LoadValue.value(), 0x3);
            assert_eq!(Mneumonic::Add.value(), 0x4);
            assert_eq!(Mneumonic::Subtract.value(), 0x5);
            assert_eq!(Mneumonic::Multiply.value(), 0x6);
            assert_eq!(Mneumonic::Divide.value(), 0x7);
            assert_eq!(Mneumonic::Load.value(), 0x8);
            assert_eq!(Mneumonic::Memory.value(), 0x9);
            assert_eq!(Mneumonic::Subroutine.value(), 0xA);
            assert_eq!(Mneumonic::ReturnFromSubrotine.value(), 0xB);
            assert_eq!(Mneumonic::HaltMachine.value(), 0xC);
            assert_eq!(Mneumonic::GetData.value(), 0xD);
            assert_eq!(Mneumonic::PutData.value(), 0xE);
            assert_eq!(Mneumonic::OperatingSystem.value(), 0xF);
    }
}

