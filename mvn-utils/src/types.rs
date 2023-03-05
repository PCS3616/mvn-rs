pub type AddressPosition = u32;

use std::convert::From;
use std::fmt;

use super::error::Span;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub column: usize,
}

impl Position {
    pub fn new(line: u32, column: usize) -> Self {
        Self { line, column }
    }
}

impl From<Span<'_>> for Position {
    fn from(value: Span<'_>) -> Self {
        Self {
            line: value.location_line(),
            column: value.get_column(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<T> {
    pub position: Position,
    pub value: T,
}

impl<T> Token<T> {
    pub fn new(position: Position, value: T) -> Self {
        Self { position, value }
    }
}

impl<T> From<T> for Token<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            position: Position::default(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", &self.value)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", &self.value)
    }
}
