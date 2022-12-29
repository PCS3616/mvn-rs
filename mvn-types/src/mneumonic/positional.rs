use std::fmt;

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
            Self::SetAbsoluteOrigin => "@",
            Self::ReserveMemory => "$",
            Self::SetEnd => "#",
            Self::SetRelocatableOrigin => "&",
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
        assert_eq!(PositionalMneumonic::SetAbsoluteOrigin.to_string(), "@");
        assert_eq!(PositionalMneumonic::SetRelocatableOrigin.to_string(), "&");
        assert_eq!(PositionalMneumonic::ReserveMemory.to_string(), "$");
        assert_eq!(PositionalMneumonic::SetEnd.to_string(), "#");
    }
}
