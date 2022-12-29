use std::fmt;

use crate::{Instruction, Operand};

#[derive(Debug, PartialEq)]
pub struct Operation<'a> {
    pub instruction: Instruction,
    pub operand: Operand<'a>,
}

impl<'a> Operation<'a> {
    pub fn new(instruction: Instruction, operand: Operand<'a>) -> Self {
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
