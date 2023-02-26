use std::fmt;

use utils::types::Token;

use crate::types::{AddressPosition, Operation};

use super::MachineAddress;

#[derive(Debug, PartialEq)]
pub struct AddressedLine<'a> {
    pub address: Token<MachineAddress>,
    pub operation: Operation<'a>,
    pub relational_annotation: Option<assembly::types::Line<'a>>,
}

impl<'a> AddressedLine<'a> {
    pub fn new(
        address: Token<MachineAddress>,
        operation: Operation<'a>,
        relational_annotation: Option<assembly::types::Line<'a>>,
    ) -> Self {
        Self {
            address,
            operation,
            relational_annotation,
        }
    }

    // FIXME Modify API to get rid of this method
    pub fn destruct(self) -> (assembly::types::Label<'a>, AddressPosition, bool) {
        let annotation = self.relational_annotation.unwrap();
        let label: assembly::types::Label = annotation.operation.operand.value.try_into().unwrap();
        let position: AddressPosition = self.operation.operand.value.try_into().unwrap();
        let relocatable = self.address.value.properties.operand_relocatable;
        (label, position, relocatable)
    }
}

impl fmt::Display for AddressedLine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Operation {
            instruction,
            operand,
        } = &self.operation;
        write!(f, "{:X} {:X}{:}", self.address, instruction, operand)?;
        if let Some(annotation) = &self.relational_annotation {
            write!(f, " ; {annotation}")
        } else {
            Ok(())
        }
    }
}
