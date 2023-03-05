use std::fmt;

use dotenv_codegen::dotenv;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionalMneumonic {
    SetAbsoluteOrigin,
    SetRelocatableOrigin,
    ReserveMemory,
    SetEnd,
}

impl fmt::Display for PositionalMneumonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mneumonic = match &self {
            Self::SetAbsoluteOrigin => dotenv!("MNEUMONIC_SET_ABSOLUTE_ORIGIN"),
            Self::ReserveMemory => dotenv!("MNEUMONIC_RESERVE_MEMORY"),
            Self::SetEnd => dotenv!("MNEUMONIC_SET_END"),
            Self::SetRelocatableOrigin => dotenv!("MNEUMONIC_SET_RELOCATABLE_ORIGIN"),
        };
        write!(f, "{mneumonic}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positional_pseudo_mneumonic_should_convert_to_string() {
        assert_eq!(
            PositionalMneumonic::SetAbsoluteOrigin.to_string(),
            dotenv!("MNEUMONIC_SET_ABSOLUTE_ORIGIN")
        );
        assert_eq!(
            PositionalMneumonic::ReserveMemory.to_string(),
            dotenv!("MNEUMONIC_RESERVE_MEMORY")
        );
        assert_eq!(
            PositionalMneumonic::SetEnd.to_string(),
            dotenv!("MNEUMONIC_SET_END")
        );
        assert_eq!(
            PositionalMneumonic::SetRelocatableOrigin.to_string(),
            dotenv!("MNEUMONIC_SET_RELOCATABLE_ORIGIN")
        );
    }
}
