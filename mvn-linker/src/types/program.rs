use super::{AddressPosition, Instruction, mneumonic::RelationalMneumonic};
use super::line::AddressedLine;

type Lines<'a> = Vec<AddressedLine<'a>>;

#[derive(Debug)]
pub struct AddressedProgram<'a> {
    pub lines: Lines<'a>,
}

impl<'a> AddressedProgram<'a> {
    pub fn new(lines: Lines<'a>) -> Self {
        Self { lines }
    }

    pub fn partition(self) -> (Lines<'a>, Lines<'a>, Lines<'a>) {
        let (symbol_table, instructions): (Vec<AddressedLine>, Vec<AddressedLine>) = self.lines.into_iter().partition(
            |line| line.relational_annotation.is_some()
        );
        let (imports, exports): (Vec<AddressedLine>, Vec<AddressedLine>) = symbol_table.into_iter().partition(
            |line| {
                match &line.relational_annotation {
                    Some(line) => match &line.operation.instruction.value {
                        Instruction::Relational(mneumonic) => mneumonic == &RelationalMneumonic::Import,
                        _ => false,
                    }
                    None => false,
                }
            }
        );
        (imports, exports, instructions)
    }

    pub fn get_last_position(&self) -> AddressPosition {
        self.lines.iter().max_by_key(
            |line| line.address.value.position
        )
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
