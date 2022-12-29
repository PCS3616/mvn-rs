use std::fmt;

use crate::Label;

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Symbolic(Label<'a>),
    Numeric(u16),
}

impl<'a> Operand<'a> {
    pub fn new_numeric(value: u16) -> Self {
        Self::Numeric(value)
    }

    pub fn new_symbolic(label: Label<'a>) -> Self {
        Self::Symbolic(label)
    }
}

impl<'a> std::convert::From<u16> for Operand<'a> {
    fn from(value: u16) -> Self {
        Self::Numeric(value)
    }
}

impl<'a> std::convert::From<Label<'a>> for Operand<'a> {
    fn from(value: Label<'a>) -> Self {
        Self::Symbolic(value)
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
