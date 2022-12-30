use std::convert::From;
use std::fmt;

use dotenv_codegen::dotenv;

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

impl fmt::Display for NormalMneumonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mneumonic: &str = match &self {
            Self::Jump  => dotenv!("MNEUMONIC_JUMP"),
            Self::JumpIfZero  => dotenv!("MNEUMONIC_JUMP_IF_ZERO"),
            Self::JumpIfNegative  => dotenv!("MNEUMONIC_JUMP_IF_NEGATIVE"),
            Self::LoadValue  => dotenv!("MNEUMONIC_LOAD_VALUE"),
            Self::Add  => dotenv!("MNEUMONIC_ADD"),
            Self::Subtract  => dotenv!("MNEUMONIC_SUBTRACT"),
            Self::Multiply  => dotenv!("MNEUMONIC_MULTIPLY"),
            Self::Divide  => dotenv!("MNEUMONIC_DIVIDE"),
            Self::Load  => dotenv!("MNEUMONIC_LOAD"),
            Self::Memory  => dotenv!("MNEUMONIC_MEMORY"),
            Self::Subroutine  => dotenv!("MNEUMONIC_SUBROUTINE"),
            Self::ReturnFromSubrotine  => dotenv!("MNEUMONIC_RETURN_FROM_SUBROTINE"),
            Self::HaltMachine  => dotenv!("MNEUMONIC_HALT_MACHINE"),
            Self::GetData  => dotenv!("MNEUMONIC_GET_DATA"),
            Self::PutData  => dotenv!("MNEUMONIC_PUT_DATA"),
            Self::OperatingSystem  => dotenv!("MNEUMONIC_OPERATING_SYSTEM"),
            Self::SetConstant  => dotenv!("MNEUMONIC_SET_CONSTANT"),
        };
        write!(f, "{mneumonic}")
    }
}

impl From<NormalMneumonic> for u8 {
    fn from(value: NormalMneumonic) -> Self {
        match value {
            NormalMneumonic::Jump  => hex_char_to_u8(dotenv!("VALUE_JUMP")),
            NormalMneumonic::JumpIfZero => hex_char_to_u8(dotenv!("VALUE_JUMP_IF_ZERO")),
            NormalMneumonic::JumpIfNegative => hex_char_to_u8(dotenv!("VALUE_JUMP_IF_NEGATIVE")),
            NormalMneumonic::LoadValue => hex_char_to_u8(dotenv!("VALUE_LOAD_VALUE")),
            NormalMneumonic::Add => hex_char_to_u8(dotenv!("VALUE_ADD")),
            NormalMneumonic::Subtract => hex_char_to_u8(dotenv!("VALUE_SUBTRACT")),
            NormalMneumonic::Multiply => hex_char_to_u8(dotenv!("VALUE_MULTIPLY")),
            NormalMneumonic::Divide => hex_char_to_u8(dotenv!("VALUE_DIVIDE")),
            NormalMneumonic::Load => hex_char_to_u8(dotenv!("VALUE_LOAD")),
            NormalMneumonic::Memory => hex_char_to_u8(dotenv!("VALUE_MEMORY")),
            NormalMneumonic::Subroutine => hex_char_to_u8(dotenv!("VALUE_SUBROUTINE")),
            NormalMneumonic::ReturnFromSubrotine => hex_char_to_u8(dotenv!("VALUE_RETURN_FROM_SUBROTINE")),
            NormalMneumonic::HaltMachine => hex_char_to_u8(dotenv!("VALUE_HALT_MACHINE")),
            NormalMneumonic::GetData => hex_char_to_u8(dotenv!("VALUE_GET_DATA")),
            NormalMneumonic::PutData => hex_char_to_u8(dotenv!("VALUE_PUT_DATA")),
            NormalMneumonic::OperatingSystem => hex_char_to_u8(dotenv!("VALUE_OPERATING_SYSTEM")),
            NormalMneumonic::SetConstant => 0,
        }
    }
}

fn hex_char_to_u8(string: &str) -> u8 {
    match string {
        "0" => 0x0,
        "1" => 0x1,
        "2" => 0x2,
        "3" => 0x3,
        "4" => 0x4,
        "5" => 0x5,
        "6" => 0x6,
        "7" => 0x7,
        "8" => 0x8,
        "9" => 0x9,
        "A" => 0xA,
        "B" => 0xB,
        "C" => 0xC,
        "D" => 0xD,
        "E" => 0xE,
        "F" => 0xF,
        _ => panic!("Input string does not represent hexadecimal character.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn normal_menumonic_should_convert_to_u8() {
        assert_eq!(u8::from(NormalMneumonic::Jump ), hex_char_to_u8(dotenv!("VALUE_JUMP")));
        assert_eq!(u8::from(NormalMneumonic::JumpIfZero), hex_char_to_u8(dotenv!("VALUE_JUMP_IF_ZERO")));
        assert_eq!(u8::from(NormalMneumonic::JumpIfNegative), hex_char_to_u8(dotenv!("VALUE_JUMP_IF_NEGATIVE")));
        assert_eq!(u8::from(NormalMneumonic::LoadValue), hex_char_to_u8(dotenv!("VALUE_LOAD_VALUE")));
        assert_eq!(u8::from(NormalMneumonic::Add), hex_char_to_u8(dotenv!("VALUE_ADD")));
        assert_eq!(u8::from(NormalMneumonic::Subtract), hex_char_to_u8(dotenv!("VALUE_SUBTRACT")));
        assert_eq!(u8::from(NormalMneumonic::Multiply), hex_char_to_u8(dotenv!("VALUE_MULTIPLY")));
        assert_eq!(u8::from(NormalMneumonic::Divide), hex_char_to_u8(dotenv!("VALUE_DIVIDE")));
        assert_eq!(u8::from(NormalMneumonic::Load), hex_char_to_u8(dotenv!("VALUE_LOAD")));
        assert_eq!(u8::from(NormalMneumonic::Memory), hex_char_to_u8(dotenv!("VALUE_MEMORY")));
        assert_eq!(u8::from(NormalMneumonic::Subroutine), hex_char_to_u8(dotenv!("VALUE_SUBROUTINE")));
        assert_eq!(u8::from(NormalMneumonic::ReturnFromSubrotine), hex_char_to_u8(dotenv!("VALUE_RETURN_FROM_SUBROTINE")));
        assert_eq!(u8::from(NormalMneumonic::HaltMachine), hex_char_to_u8(dotenv!("VALUE_HALT_MACHINE")));
        assert_eq!(u8::from(NormalMneumonic::GetData), hex_char_to_u8(dotenv!("VALUE_GET_DATA")));
        assert_eq!(u8::from(NormalMneumonic::PutData), hex_char_to_u8(dotenv!("VALUE_PUT_DATA")));
        assert_eq!(u8::from(NormalMneumonic::OperatingSystem), hex_char_to_u8(dotenv!("VALUE_OPERATING_SYSTEM")));
        assert_eq!(u8::from(NormalMneumonic::SetConstant), 0);
    }

    #[test]
    fn instruction_menumonic_should_convert_to_string() {
        assert_eq!(NormalMneumonic::Jump.to_string(), dotenv!("MNEUMONIC_JUMP"));
        assert_eq!(NormalMneumonic::JumpIfZero.to_string(), dotenv!("MNEUMONIC_JUMP_IF_ZERO"));
        assert_eq!(NormalMneumonic::JumpIfNegative.to_string(), dotenv!("MNEUMONIC_JUMP_IF_NEGATIVE"));
        assert_eq!(NormalMneumonic::LoadValue.to_string(), dotenv!("MNEUMONIC_LOAD_VALUE"));
        assert_eq!(NormalMneumonic::Add.to_string(), dotenv!("MNEUMONIC_ADD"));
        assert_eq!(NormalMneumonic::Subtract.to_string(), dotenv!("MNEUMONIC_SUBTRACT"));
        assert_eq!(NormalMneumonic::Multiply.to_string(), dotenv!("MNEUMONIC_MULTIPLY"));
        assert_eq!(NormalMneumonic::Divide.to_string(), dotenv!("MNEUMONIC_DIVIDE"));
        assert_eq!(NormalMneumonic::Load.to_string(), dotenv!("MNEUMONIC_LOAD"));
        assert_eq!(NormalMneumonic::Memory.to_string(), dotenv!("MNEUMONIC_MEMORY"));
        assert_eq!(NormalMneumonic::Subroutine.to_string(), dotenv!("MNEUMONIC_SUBROUTINE"));
        assert_eq!(NormalMneumonic::ReturnFromSubrotine.to_string(), dotenv!("MNEUMONIC_RETURN_FROM_SUBROTINE"));
        assert_eq!(NormalMneumonic::HaltMachine.to_string(), dotenv!("MNEUMONIC_HALT_MACHINE"));
        assert_eq!(NormalMneumonic::GetData.to_string(), dotenv!("MNEUMONIC_GET_DATA"));
        assert_eq!(NormalMneumonic::PutData.to_string(), dotenv!("MNEUMONIC_PUT_DATA"));
        assert_eq!(NormalMneumonic::OperatingSystem.to_string(), dotenv!("MNEUMONIC_OPERATING_SYSTEM"));
        assert_eq!(NormalMneumonic::SetConstant.to_string(), dotenv!("MNEUMONIC_SET_CONSTANT"));

    }

    #[test]
    fn constant_mneumonic_should_convert_to_string() {
        assert_eq!(NormalMneumonic::SetConstant.to_string(), "K");
    }
}
