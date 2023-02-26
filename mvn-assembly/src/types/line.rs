use std::fmt;

use utils::types::{Position, Token};

use super::{Label, Operation};

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub label: Option<Token<Label<'a>>>,
    pub operation: Operation<'a>,
}

impl<'a> Line<'a> {
    pub fn new(label: Option<Token<Label<'a>>>, operation: Operation<'a>) -> Self {
        Self { label, operation }
    }

    pub fn position(&self) -> Position {
        let column = if let Some(token) = &self.label {
            token.position.column
        } else {
            self.operation.instruction.position.column
        };
        Position::new(self.operation.instruction.position.line, column)
    }
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = if let Some(label) = &self.label {
            label.to_string()
        } else {
            "".to_owned()
        };
        write!(f, "{}\t\t{}", label, &self.operation)
    }
}
