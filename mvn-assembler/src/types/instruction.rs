use std::fmt;

use super::mneumonic::{NormalMneumonic, PositionalMneumonic, RelationalMneumonic};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Normal(NormalMneumonic),
    Positional(PositionalMneumonic),
    Relational(RelationalMneumonic),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mneumonic = match &self {
            Self::Normal(mneumonic) => mneumonic.to_string(),
            Self::Positional(mneumonic) => mneumonic.to_string(),
            Self::Relational(mneumonic) => mneumonic.to_string(),
        };
        write!(f, "{mneumonic}")
    }
}
