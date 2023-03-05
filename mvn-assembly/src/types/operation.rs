use std::fmt;

use utils::types::Token;

use super::{Instruction, Operand};

#[derive(Debug, PartialEq)]
pub struct Operation<'a> {
    pub instruction: Token<Instruction>,
    pub operand: Token<Operand<'a>>,
}

impl<'a> Operation<'a> {
    pub fn new(instruction: Token<Instruction>, operand: Token<Operand<'a>>) -> Self {
        Self {
            instruction,
            operand,
        }
    }
}

impl<'a> fmt::Display for Operation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.instruction, &self.operand)
    }
}
