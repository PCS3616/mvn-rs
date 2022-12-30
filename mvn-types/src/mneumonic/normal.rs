use std::convert::{From, TryFrom};
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
        let mneumonic = match &self {
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

impl TryFrom<u8> for NormalMneumonic {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::Jump),
            0x1 => Ok(Self::JumpIfZero),
            0x2 => Ok(Self::JumpIfNegative),
            0x3 => Ok(Self::LoadValue),
            0x4 => Ok(Self::Add),
            0x5 => Ok(Self::Subtract),
            0x6 => Ok(Self::Multiply),
            0x7 => Ok(Self::Divide),
            0x8 => Ok(Self::Load),
            0x9 => Ok(Self::Memory),
            0xA => Ok(Self::Subroutine),
            0xB => Ok(Self::ReturnFromSubrotine),
            0xC => Ok(Self::HaltMachine),
            0xD => Ok(Self::GetData),
            0xE => Ok(Self::PutData),
            0xF => Ok(Self::OperatingSystem),
            _ => Err("Value does not represent a valid instruction."),
        }
    }
}

impl From<NormalMneumonic> for u8 {
    fn from(value: NormalMneumonic) -> Self {
        match value {
            NormalMneumonic::Jump | NormalMneumonic::SetConstant => 0x0,
            NormalMneumonic::JumpIfZero => 0x1,
            NormalMneumonic::JumpIfNegative => 0x2,
            NormalMneumonic::LoadValue => 0x3,
            NormalMneumonic::Add => 0x4,
            NormalMneumonic::Subtract => 0x5,
            NormalMneumonic::Multiply => 0x6,
            NormalMneumonic::Divide => 0x7,
            NormalMneumonic::Load => 0x8,
            NormalMneumonic::Memory => 0x9,
            NormalMneumonic::Subroutine => 0xA,
            NormalMneumonic::ReturnFromSubrotine => 0xB,
            NormalMneumonic::HaltMachine => 0xC,
            NormalMneumonic::GetData => 0xD,
            NormalMneumonic::PutData => 0xE,
            NormalMneumonic::OperatingSystem => 0xF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn u8_should_convert_to_normal_menumonic() {
        assert_eq!(0x0.try_into(), Ok(NormalMneumonic::Jump));
        assert_eq!(0x1.try_into(), Ok(NormalMneumonic::JumpIfZero));
        assert_eq!(0x2.try_into(), Ok(NormalMneumonic::JumpIfNegative));
        assert_eq!(0x3.try_into(), Ok(NormalMneumonic::LoadValue));
        assert_eq!(0x4.try_into(), Ok(NormalMneumonic::Add));
        assert_eq!(0x5.try_into(), Ok(NormalMneumonic::Subtract));
        assert_eq!(0x6.try_into(), Ok(NormalMneumonic::Multiply));
        assert_eq!(0x7.try_into(), Ok(NormalMneumonic::Divide));
        assert_eq!(0x8.try_into(), Ok(NormalMneumonic::Load));
        assert_eq!(0x9.try_into(), Ok(NormalMneumonic::Memory));
        assert_eq!(0xA.try_into(), Ok(NormalMneumonic::Subroutine));
        assert_eq!(0xB.try_into(), Ok(NormalMneumonic::ReturnFromSubrotine));
        assert_eq!(0xC.try_into(), Ok(NormalMneumonic::HaltMachine));
        assert_eq!(0xD.try_into(), Ok(NormalMneumonic::GetData));
        assert_eq!(0xE.try_into(), Ok(NormalMneumonic::PutData));
        assert_eq!(0xF.try_into(), Ok(NormalMneumonic::OperatingSystem));
    }

    #[test]
    #[ignore = "Expensive exhaustive test not necessary"]
    fn invalid_u8_should_not_convert_to_normal_mneumonic() {
        for i in 16u8..=255u8 {
            // 16u8.. panics on overflow
            let result: Result<NormalMneumonic, _> = i.try_into();
            assert!(result.is_err());
        }
    }

    #[test]
    fn normal_menumonic_should_convert_to_u8() {
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
    fn normal_menumonic_should_convert_to_u8_and_back() {
        for mneumonic_value in 0..16 {
            let mneumonic_value = mneumonic_value as u8;
            let mneumonic = NormalMneumonic::try_from(mneumonic_value).unwrap();
            assert_eq!(mneumonic_value, mneumonic.into());
        }
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