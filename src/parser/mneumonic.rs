use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;

#[derive(Debug, PartialEq, Clone)]
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
    pub fn parse(input: &str) -> IResult<&str, Mneumonic> {
        alt((
            value(Mneumonic::Jump, tag("JP")),
            value(Mneumonic::JumpIfZero, tag("JZ")),
            value(Mneumonic::JumpIfNegative, tag("JN")),
            value(Mneumonic::LoadValue, tag("LV")),
            value(Mneumonic::Add, tag("AD")),
            value(Mneumonic::Subtract, tag("SB")),
            value(Mneumonic::Multiply, tag("ML")),
            value(Mneumonic::Divide, tag("DV")),
            value(Mneumonic::Load, tag("LD")),
            value(Mneumonic::Memory, tag("MM")),
            value(Mneumonic::Subroutine, tag("SC")),
            value(Mneumonic::ReturnFromSubrotine, tag("RS")),
            value(Mneumonic::HaltMachine, tag("HM")),
            value(Mneumonic::GetData, tag("GD")),
            value(Mneumonic::PutData, tag("PD")),
            value(Mneumonic::OperatingSystem, tag("OS")),
        ))(input)
    }
    
    pub fn value(&self) -> u8 {
        match self {
            Mneumonic::Jump => 0x0,
            Mneumonic::JumpIfZero => 0x1,
            Mneumonic::JumpIfNegative => 0x2,
            Mneumonic::LoadValue => 0x3,
            Mneumonic::Add => 0x4,
            Mneumonic::Subtract => 0x5,
            Mneumonic::Multiply => 0x6,
            Mneumonic::Divide => 0x7,
            Mneumonic::Load => 0x8,
            Mneumonic::Memory => 0x9,
            Mneumonic::Subroutine => 0xA,
            Mneumonic::ReturnFromSubrotine => 0xB,
            Mneumonic::HaltMachine => 0xC,
            Mneumonic::GetData => 0xD,
            Mneumonic::PutData => 0xE,
            Mneumonic::OperatingSystem => 0xF
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

