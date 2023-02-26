use super::line::AddressedLine;
use super::{mneumonic::RelationalMneumonic, AddressPosition, Instruction};

type Lines<'a> = Vec<AddressedLine<'a>>;

#[derive(Debug)]
pub struct AddressedProgram<'a> {
    pub lines: Lines<'a>,
}

impl<'a> AddressedProgram<'a> {
    pub fn new(lines: Lines<'a>) -> Self {
        Self { lines }
    }

    // TODO Implement wrapper types for imports, exports and instructions
    // so we don't have to depend on getting the return order right
    pub fn partition(self) -> (Lines<'a>, Lines<'a>, Lines<'a>) {
        let (symbol_table, instructions): (Vec<AddressedLine>, Vec<AddressedLine>) = self
            .lines
            .into_iter()
            .partition(|line| line.relational_annotation.is_some());
        let (imports, exports): (Vec<AddressedLine>, Vec<AddressedLine>) = symbol_table
            .into_iter()
            .partition(|line| match &line.relational_annotation {
                Some(line) => match &line.operation.instruction.value {
                    Instruction::Relational(mneumonic) => mneumonic == &RelationalMneumonic::Import,
                    _ => false,
                },
                None => false,
            });
        (imports, exports, instructions)
    }

    pub fn get_last_position(&self) -> AddressPosition {
        self.lines
            .iter()
            .max_by_key(|line| line.address.value.position)
            .map_or(0, |line| line.address.value.position)
    }
}

impl<'a> IntoIterator for AddressedProgram<'a> {
    type Item = AddressedLine<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

impl<'a> FromIterator<AddressedLine<'a>> for AddressedProgram<'a> {
    fn from_iter<T: IntoIterator<Item = AddressedLine<'a>>>(iter: T) -> Self {
        let mut lines: Lines = Vec::new();
        for line in iter {
            lines.push(line);
        }
        Self::new(lines)
    }
}

#[cfg(test)]
mod tests {
    use assembly::types::Line;
    use pretty_assertions::assert_eq;

    use crate::types::mneumonic::{NormalMneumonic, RelationalMneumonic};
    use crate::types::{MachineAddress, MachineAddressProperties, Operand, Operation};

    use super::*;

    fn test_exports() -> Lines<'static> {
        vec![AddressedLine::new(
            MachineAddress::new(MachineAddressProperties::new(false, false, false), 0x000).into(),
            Operation::new(
                Instruction::Normal(NormalMneumonic::Jump).into(),
                Operand::from(0).into(),
            ),
            Some(Line::new(
                None,
                Operation::new(
                    Instruction::Relational(RelationalMneumonic::Export).into(),
                    Operand::from("FOO").into(),
                ),
            )),
        )]
    }

    fn test_imports() -> Lines<'static> {
        vec![AddressedLine::new(
            MachineAddress::new(MachineAddressProperties::new(false, false, false), 0x002).into(),
            Operation::new(
                Instruction::Normal(NormalMneumonic::Jump).into(),
                Operand::from(0).into(),
            ),
            Some(Line::new(
                None,
                Operation::new(
                    Instruction::Relational(RelationalMneumonic::Import).into(),
                    Operand::from("BAR").into(),
                ),
            )),
        )]
    }

    fn test_instructions() -> Lines<'static> {
        vec![
            AddressedLine::new(
                MachineAddress::new(MachineAddressProperties::new(false, false, false), 0x004)
                    .into(),
                Operation::new(
                    Instruction::Normal(NormalMneumonic::LoadValue).into(),
                    Operand::new_numeric(0x001).into(),
                ),
                None,
            ),
            AddressedLine::new(
                MachineAddress::new(MachineAddressProperties::new(false, false, false), 0x100)
                    .into(),
                Operation::new(
                    Instruction::Normal(NormalMneumonic::HaltMachine).into(),
                    Operand::new_numeric(0x100).into(),
                ),
                None,
            ),
        ]
    }

    fn test_program() -> AddressedProgram<'static> {
        let mut lines = test_exports();
        lines.append(&mut test_imports());
        lines.append(&mut test_instructions());
        AddressedProgram::new(lines)
    }

    #[test]
    fn should_partition_code() {
        let program = test_program();
        let (imports, exports, instructions) = program.partition();
        assert_eq!(exports, test_exports());
        assert_eq!(imports, test_imports());
        assert_eq!(instructions, test_instructions());
    }

    #[test]
    fn should_get_last_position() {
        assert_eq!(test_program().get_last_position(), 0x100);
    }
}
