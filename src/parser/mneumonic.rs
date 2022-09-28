use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use dotenv_codegen::dotenv;

use super::util::hexadecimal;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mneumonic {
    // Instructions
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
    // Pseudo-instructions
    SetConstant,
    SetEnd,
    ReserveMemory,
    SetAbsoluteOrigin,
    SetRelocatableOrigin,
    // Relations
    Import,
    Export,
}

impl Mneumonic {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Jump,                   tag(dotenv!("MNEUMONIC_JUMP"))),
            value(Self::JumpIfZero,             tag(dotenv!("MNEUMONIC_JUMP_IF_ZERO"))),
            value(Self::JumpIfNegative,         tag(dotenv!("MNEUMONIC_JUMP_IF_NEGATIVE"))),
            value(Self::LoadValue,              tag(dotenv!("MNEUMONIC_LOAD_VALUE"))),
            value(Self::Add,                    tag(dotenv!("MNEUMONIC_ADD"))),
            value(Self::Subtract,               tag(dotenv!("MNEUMONIC_SUBTRACT"))),
            value(Self::Multiply,               tag(dotenv!("MNEUMONIC_MULTIPLY"))),
            value(Self::Divide,                 tag(dotenv!("MNEUMONIC_DIVIDE"))),
            value(Self::Load,                   tag(dotenv!("MNEUMONIC_LOAD"))),
            value(Self::Memory,                 tag(dotenv!("MNEUMONIC_MEMORY"))),
            value(Self::Subroutine,             tag(dotenv!("MNEUMONIC_SUBROUTINE"))),
            value(Self::ReturnFromSubrotine,    tag(dotenv!("MNEUMONIC_RETURN_FROM_SUBROTINE"))),
            value(Self::HaltMachine,            tag(dotenv!("MNEUMONIC_HALT_MACHINE"))),
            value(Self::GetData,                tag(dotenv!("MNEUMONIC_GET_DATA"))),
            value(Self::PutData,                tag(dotenv!("MNEUMONIC_PUT_DATA"))),
            value(Self::OperatingSystem,        tag(dotenv!("MNEUMONIC_OPERATING_SYSTEM"))),
            value(Self::SetConstant,            tag(dotenv!("MNEUMONIC_SET_CONSTANT"))),
            value(Self::SetEnd,                 tag(dotenv!("MNEUMONIC_SET_END"))),
            value(Self::ReserveMemory,          tag(dotenv!("MNEUMONIC_RESERVE_MEMORY"))),
            value(Self::SetAbsoluteOrigin,      tag(dotenv!("MNEUMONIC_SET_ABSOLUTE_ORIGIN"))),
            value(Self::SetRelocatableOrigin,   tag(dotenv!("MNEUMONIC_SET_RELOCATABLE_ORIGIN"))),
            // TODO Discover why adding these optag(dotenv!("MNEUMONIC_IMPORT"))),tions causes an erro
            // value(Self::Import, tag("<"))    tag(dotenv!("MNEUMONIC_EXPORT"))),
            // value(Self::Export, tag(">")),
        ))(input)
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::Jump                  => hexadecimal(dotenv!("VALUE_JUMP")).unwrap().1,
            Self::JumpIfZero            => hexadecimal(dotenv!("VALUE_JUMP_IF_ZERO")).unwrap().1,
            Self::JumpIfNegative        => hexadecimal(dotenv!("VALUE_JUMP_IF_NEGATIVE")).unwrap().1,
            Self::LoadValue             => hexadecimal(dotenv!("VALUE_LOAD_VALUE")).unwrap().1,
            Self::Add                   => hexadecimal(dotenv!("VALUE_ADD")).unwrap().1,
            Self::Subtract              => hexadecimal(dotenv!("VALUE_SUBTRACT")).unwrap().1,
            Self::Multiply              => hexadecimal(dotenv!("VALUE_MULTIPLY")).unwrap().1,
            Self::Divide                => hexadecimal(dotenv!("VALUE_DIVIDE")).unwrap().1,
            Self::Load                  => hexadecimal(dotenv!("VALUE_LOAD")).unwrap().1,
            Self::Memory                => hexadecimal(dotenv!("VALUE_MEMORY")).unwrap().1,
            Self::Subroutine            => hexadecimal(dotenv!("VALUE_SUBROUTINE")).unwrap().1,
            Self::ReturnFromSubrotine   => hexadecimal(dotenv!("VALUE_RETURN_FROM_SUBROTINE")).unwrap().1,
            Self::HaltMachine           => hexadecimal(dotenv!("VALUE_HALT_MACHINE")).unwrap().1,
            Self::GetData               => hexadecimal(dotenv!("VALUE_GET_DATA")).unwrap().1,
            Self::PutData               => hexadecimal(dotenv!("VALUE_PUT_DATA")).unwrap().1,
            Self::OperatingSystem       => hexadecimal(dotenv!("VALUE_OPERATING_SYSTEM")).unwrap().1,
            _ => 0x0
        }
    }

}

    pub fn is_normal(mneumonic: &Mneumonic) -> bool {
        [
            Mneumonic::Jump,
            Mneumonic::JumpIfZero,
            Mneumonic::JumpIfNegative,
            Mneumonic::LoadValue,
            Mneumonic::Add,
            Mneumonic::Subtract,
            Mneumonic::Multiply,
            Mneumonic::Divide,
            Mneumonic::Load,
            Mneumonic::Memory,
            Mneumonic::Subroutine,
            Mneumonic::ReturnFromSubrotine,
            Mneumonic::HaltMachine,
            Mneumonic::GetData,
            Mneumonic::PutData,
            Mneumonic::OperatingSystem,
            Mneumonic::SetConstant
        ].contains(mneumonic)
    }

    pub fn is_positional(mneumonic: &Mneumonic) -> bool {
        [
            Mneumonic::ReserveMemory,
            Mneumonic::SetAbsoluteOrigin,
            Mneumonic::SetRelocatableOrigin
        ].contains(mneumonic)
    }

    pub fn is_relational(mneumonic: &Mneumonic) -> bool {
        [
            Mneumonic::Import,
            Mneumonic::Export
        ].contains(mneumonic)
    }

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_instruction_mneumonic() {
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
    fn should_parse_pseudo_instruction_mneumonic(){
        assert_eq!(Mneumonic::parse("K"), Ok(("", Mneumonic::SetConstant)));
        assert_eq!(Mneumonic::parse("#"), Ok(("", Mneumonic::SetEnd)));
        assert_eq!(Mneumonic::parse("$"), Ok(("", Mneumonic::ReserveMemory)));
        assert_eq!(Mneumonic::parse("@"), Ok(("", Mneumonic::SetAbsoluteOrigin)));
        assert_eq!(Mneumonic::parse("&"), Ok(("", Mneumonic::SetRelocatableOrigin)));
        // assert_eq!(Mneumonic::parse(">"), Ok(("", Mneumonic::Export)));
        // assert_eq!(Mneumonic::parse("<"), Ok(("", Mneumonic::Import)));
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

    #[test]
    fn should_detect_normal_mneumonics(){
        for mneumonic in [
            Mneumonic::Jump,
            Mneumonic::JumpIfZero,
            Mneumonic::JumpIfNegative,
            Mneumonic::LoadValue,
            Mneumonic::Add,
            Mneumonic::Subtract,
            Mneumonic::Multiply,
            Mneumonic::Divide,
            Mneumonic::Load,
            Mneumonic::Memory,
            Mneumonic::Subroutine,
            Mneumonic::ReturnFromSubrotine,
            Mneumonic::HaltMachine,
            Mneumonic::GetData,
            Mneumonic::PutData,
            Mneumonic::OperatingSystem,
            Mneumonic::SetConstant,
        ] {
            assert_eq!(is_normal(&mneumonic), true);
            assert_eq!(is_positional(&mneumonic), false);
            assert_eq!(is_relational(&mneumonic), false);
        }

    }

    #[test]
    fn should_detect_positional_mneumonics(){
        for mneumonic in [
            Mneumonic::ReserveMemory,
            Mneumonic::SetAbsoluteOrigin,
            Mneumonic::SetRelocatableOrigin
        ] {
            assert_eq!(is_normal(&mneumonic), false);
            assert_eq!(is_positional(&mneumonic), true);
            assert_eq!(is_relational(&mneumonic), false);
        }

    }

    #[test]
    fn should_detect_relational_mneumonics(){
        for mneumonic in [
            Mneumonic::Import,
            Mneumonic::Export,
        ] {
            assert_eq!(is_normal(&mneumonic), false);
            assert_eq!(is_positional(&mneumonic), false);
            assert_eq!(is_relational(&mneumonic), true);
        }

    }

}
