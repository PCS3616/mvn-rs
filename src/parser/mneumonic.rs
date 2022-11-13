use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;

pub trait Mneumonic: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;
    fn to_str(&self) -> &str;
    fn value(&self) -> u8 {
        0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NormalMneumonic {
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
    OperatingSystem,
    SetConstant,
}

impl Mneumonic for NormalMneumonic {
    fn parse(input: &str) -> IResult<&str, Self> {
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
            value(Self::SetConstant, tag(Self::SetConstant.to_str())),
        ))(input)
    }

    fn to_str(&self) -> &str {
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
            Self::OperatingSystem => "OS",
            Self::SetConstant => "K",
        }
    }

    fn value(&self) -> u8 {
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
            Self::OperatingSystem => 0xF,
            Self::SetConstant => 0,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionalMneumonic {
    SetAbsoluteOrigin,
    SetRelocatableOrigin,
    ReserveMemory,
    SetEnd,
}

impl Mneumonic for PositionalMneumonic {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::SetAbsoluteOrigin, tag(Self::SetAbsoluteOrigin.to_str())),
            value(Self::ReserveMemory, tag(Self::ReserveMemory.to_str())),
            value(Self::SetEnd, tag(Self::SetEnd.to_str())),
            value(Self::SetRelocatableOrigin, tag(Self::SetRelocatableOrigin.to_str())),
        ))(input)
    }

    fn to_str(&self) -> &str {
        match self {
            Self::SetAbsoluteOrigin => "@",
            Self::ReserveMemory => "$",
            Self::SetEnd => "#",
            Self::SetRelocatableOrigin => "&",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RelationalMneumonic {
    Export,
    Import,
}

impl Mneumonic for RelationalMneumonic {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Export, tag(Self::Export.to_str())),
            value(Self::Import, tag(Self::Import.to_str())),
        ))(input)
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Export => ">",
            Self::Import => "<",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_mneumonic() {
        assert_eq!(NormalMneumonic::parse("JP"), Ok(("", NormalMneumonic::Jump)));
        assert_eq!(NormalMneumonic::parse("JZ"), Ok(("",  NormalMneumonic::JumpIfZero)));
        assert_eq!(NormalMneumonic::parse("JN"), Ok(("",  NormalMneumonic::JumpIfNegative)));
        assert_eq!(NormalMneumonic::parse("LV"), Ok(("",  NormalMneumonic::LoadValue)));
        assert_eq!(NormalMneumonic::parse("AD"), Ok(("",  NormalMneumonic::Add)));
        assert_eq!(NormalMneumonic::parse("SB"), Ok(("",  NormalMneumonic::Subtract)));
        assert_eq!(NormalMneumonic::parse("ML"), Ok(("",  NormalMneumonic::Multiply)));
        assert_eq!(NormalMneumonic::parse("DV"), Ok(("",  NormalMneumonic::Divide)));
        assert_eq!(NormalMneumonic::parse("LD"), Ok(("",  NormalMneumonic::Load)));
        assert_eq!(NormalMneumonic::parse("MM"), Ok(("",  NormalMneumonic::Memory)));
        assert_eq!(NormalMneumonic::parse("SC"), Ok(("",  NormalMneumonic::Subroutine)));
        assert_eq!(NormalMneumonic::parse("RS"), Ok(("",  NormalMneumonic::ReturnFromSubrotine)));
        assert_eq!(NormalMneumonic::parse("HM"), Ok(("",  NormalMneumonic::HaltMachine)));
        assert_eq!(NormalMneumonic::parse("GD"), Ok(("",  NormalMneumonic::GetData)));
        assert_eq!(NormalMneumonic::parse("PD"), Ok(("",  NormalMneumonic::PutData)));
        assert_eq!(NormalMneumonic::parse("OS"), Ok(("",  NormalMneumonic::OperatingSystem)));
    }

    #[test]
    fn should_return_value() {
        assert_eq!(NormalMneumonic::Jump.value(), 0x0);
        assert_eq!(NormalMneumonic::JumpIfZero.value(), 0x1);
        assert_eq!(NormalMneumonic::JumpIfNegative.value(), 0x2);
        assert_eq!(NormalMneumonic::LoadValue.value(), 0x3);
        assert_eq!(NormalMneumonic::Add.value(), 0x4);
        assert_eq!(NormalMneumonic::Subtract.value(), 0x5);
        assert_eq!(NormalMneumonic::Multiply.value(), 0x6);
        assert_eq!(NormalMneumonic::Divide.value(), 0x7);
        assert_eq!(NormalMneumonic::Load.value(), 0x8);
        assert_eq!(NormalMneumonic::Memory.value(), 0x9);
        assert_eq!(NormalMneumonic::Subroutine.value(), 0xA);
        assert_eq!(NormalMneumonic::ReturnFromSubrotine.value(), 0xB);
        assert_eq!(NormalMneumonic::HaltMachine.value(), 0xC);
        assert_eq!(NormalMneumonic::GetData.value(), 0xD);
        assert_eq!(NormalMneumonic::PutData.value(), 0xE);
        assert_eq!(NormalMneumonic::OperatingSystem.value(), 0xF);
    }

    #[test]
    fn should_parse_constant_assignment() {
        assert_eq!(NormalMneumonic::parse("K"), Ok(("", NormalMneumonic::SetConstant)));
    }

    #[test]
    fn should_parse_positional_pseudo_mneumonic() {
        assert_eq!(PositionalMneumonic::parse("@"), Ok(("", PositionalMneumonic::SetAbsoluteOrigin)));
        assert_eq!(PositionalMneumonic::parse("&"), Ok(("", PositionalMneumonic::SetRelocatableOrigin)));
        assert_eq!(PositionalMneumonic::parse("$"), Ok(("", PositionalMneumonic::ReserveMemory)));
        assert_eq!(PositionalMneumonic::parse("#"), Ok(("", PositionalMneumonic::SetEnd)));
    }

    #[test]
    fn should_parse_relational_pseudo_mneumonic() {
        assert_eq!(RelationalMneumonic::parse(">"), Ok(("", RelationalMneumonic::Export)));
        assert_eq!(RelationalMneumonic::parse("<"), Ok(("", RelationalMneumonic::Import)));
    }
}

