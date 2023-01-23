use crate::types::AddressPosition;

use super::MachineAddress;

#[derive(Debug, PartialEq)]
pub struct AddressedLine<'a> {
    pub address: MachineAddress,
    pub operation: assembler::types::Operation<'a>,
    pub relational_annotation: Option<assembler::types::Line<'a>>,
}

impl<'a> AddressedLine<'a> {
    pub fn new(address: MachineAddress, operation: assembler::types::Operation<'a>, relational_annotation: Option<assembler::types::Line<'a>>) -> Self{
        Self { address, operation, relational_annotation }
    }

    // FIXME Modify API to get rid of this method
    pub fn destruct(self) -> (assembler::types::Label<'a>, AddressPosition) {
        let annotation = self.relational_annotation.unwrap();
        let label: assembler::types::Label = annotation.operation.operand.value.try_into().unwrap();
        let position: AddressPosition = self.operation.operand.value.try_into().unwrap();
        (label, position)
    }
}
