mod instruction;
mod label;
mod line;
pub mod mneumonic;
mod operand;
mod operation;
mod program;

pub use instruction::Instruction;
pub use label::Label;
pub use line::Line;
pub use operand::Operand;
pub use operation::Operation;
pub use program::Program;

use std::convert::From;
use std::fmt;

use utils::error::Span;

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
pub struct Token<T> where {
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
        Self { value, position: Position::default() }
    }
}

impl<T: fmt::Display> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}
