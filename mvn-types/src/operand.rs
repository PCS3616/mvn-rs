use std::fmt;
use std::convert::{From, TryFrom};

use crate::Label;

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Symbolic(Label<'a>),
    Numeric(u32),
}

// FIXME Replace calls to `new_numeric` and `new_symbolic` with `from`
impl<'a> Operand<'a> {
    pub fn new_numeric(value: u32) -> Self {
        Self::Numeric(value)
    }

    pub fn new_symbolic(label: Label<'a>) -> Self {
        Self::Symbolic(label)
    }
}

impl From<u32> for Operand<'_> {
    fn from(value: u32) -> Self {
        Self::Numeric(value)
    }
}

impl<'a> From<Label<'a>> for Operand<'a> {
    fn from(value: Label<'a>) -> Self {
        Self::Symbolic(value)
    }
}

impl<'a> From<&'a str> for Operand<'a> {
    fn from(value: &'a str) -> Self {
        Self::Symbolic(value.into())
    }
}

impl TryFrom<Operand<'_>> for u32 {
    type Error = &'static str;
    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Numeric(value) => Ok(value),
            Operand::Symbolic(_) => Err("operand is not numeric, so cannot be converted to `u16`"),
        }
    }
}

impl<'a> TryFrom<Operand<'a>> for Label<'a> {
    type Error = &'static str;
    fn try_from(value: Operand<'a>) -> Result<Self, Self::Error> {
        match value {
            Operand::Numeric(_) => Err("operand is not symbolic, so cannot be converted to `Label`"),
            Operand::Symbolic(label) => Ok(label),
        }
    }
}

impl<'a> fmt::Display for Operand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operand = match &self {
            Self::Symbolic(label) => label.to_string(),
            Self::Numeric(immediate) => immediate.to_string(),
        };
        write!(f, "{}", operand)
    }
}
