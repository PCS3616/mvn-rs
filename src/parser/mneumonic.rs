use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;

use super::util::{LocatedIResult, Span};

pub trait Mneumonic: Sized {
    fn parse(input: Span) -> LocatedIResult<Self>;
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
    fn parse(input: Span) -> LocatedIResult<Self> {
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
    fn parse(input: Span) -> LocatedIResult<Self> {
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
    fn parse(input: Span) -> LocatedIResult<Self> {
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
        let inputs_outputs = [
            ("JP", NormalMneumonic::Jump),
            ("JZ", NormalMneumonic::JumpIfZero),
            ("JN", NormalMneumonic::JumpIfNegative),
            ("LV", NormalMneumonic::LoadValue),
            ("AD", NormalMneumonic::Add),
            ("SB", NormalMneumonic::Subtract),
            ("ML", NormalMneumonic::Multiply),
            ("DV", NormalMneumonic::Divide),
            ("LD", NormalMneumonic::Load),
            ("MM", NormalMneumonic::Memory),
            ("SC", NormalMneumonic::Subroutine),
            ("RS", NormalMneumonic::ReturnFromSubrotine),
            ("HM", NormalMneumonic::HaltMachine),
            ("GD", NormalMneumonic::GetData),
            ("PD", NormalMneumonic::PutData),
            ("OS", NormalMneumonic::OperatingSystem),
        ];

        for (input, output) in inputs_outputs {
            assert_eq!(
                NormalMneumonic::parse(Span::new(input)).unwrap().1,
                output,
            );
        }
    }

    #[test]
    fn should_return_value() {
        let inputs_outputs = [
            (NormalMneumonic::Jump, 0x0),
            (NormalMneumonic::JumpIfZero, 0x1),
            (NormalMneumonic::JumpIfNegative, 0x2),
            (NormalMneumonic::LoadValue, 0x3),
            (NormalMneumonic::Add, 0x4),
            (NormalMneumonic::Subtract, 0x5),
            (NormalMneumonic::Multiply, 0x6),
            (NormalMneumonic::Divide, 0x7),
            (NormalMneumonic::Load, 0x8),
            (NormalMneumonic::Memory, 0x9),
            (NormalMneumonic::Subroutine, 0xA),
            (NormalMneumonic::ReturnFromSubrotine, 0xB),
            (NormalMneumonic::HaltMachine, 0xC),
            (NormalMneumonic::GetData, 0xD),
            (NormalMneumonic::PutData, 0xE),
            (NormalMneumonic::OperatingSystem, 0xF),
        ];

        for (input, output) in inputs_outputs.into_iter() {
            assert_eq!(input.value(), output);
        }
    }

    #[test]
    fn should_parse_constant_assignment() {
        assert_eq!(NormalMneumonic::parse(Span::new("K")).unwrap().1, NormalMneumonic::SetConstant);
    }

    #[test]
    fn should_parse_positional_pseudo_mneumonic() {
        assert_eq!(PositionalMneumonic::parse(Span::new("@")).unwrap().1, PositionalMneumonic::SetAbsoluteOrigin);
        assert_eq!(PositionalMneumonic::parse(Span::new("&")).unwrap().1, PositionalMneumonic::SetRelocatableOrigin);
        assert_eq!(PositionalMneumonic::parse(Span::new("$")).unwrap().1, PositionalMneumonic::ReserveMemory);
        assert_eq!(PositionalMneumonic::parse(Span::new("#")).unwrap().1, PositionalMneumonic::SetEnd);
    }

    #[test]
    fn should_parse_relational_pseudo_mneumonic() {
        assert_eq!(RelationalMneumonic::parse(Span::new(">")).unwrap().1, RelationalMneumonic::Export);
        assert_eq!(RelationalMneumonic::parse(Span::new("<")).unwrap().1, RelationalMneumonic::Import);
    }
}
