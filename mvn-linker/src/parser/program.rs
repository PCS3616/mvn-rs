use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;

use types::{Instruction, mneumonic::RelationalMneumonic};

use super::{Parse, Position, Relocate};
use super::error;
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

    pub fn get_last_position(&self) -> Position {
        self.lines.iter().max_by_key(
            |line| line.address.position
        )
        .map_or(0, |line| line.address.position)
    }
}

impl Relocate for AddressedProgram<'_> {
    fn relocate(self, base: Position) -> Self {
        AddressedProgram::new(self.lines.into_iter().map(|line| line.relocate(base)).collect())
    }
}

impl<'a> Parse<'a> for AddressedProgram<'a> {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        map(
            separated_list1(line_ending, AddressedLine::parse_machine_code),
            |lines| Self::new(lines),
        )(input)
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
    use pretty_assertions::assert_eq;
    use types::{Line, Operation, Token, Position};
    use super::*;
    use crate::parser::line::AddressedLine;

    #[test]
    fn should_parse_program() {
        let lines = vec![
            "1000 0000 ; < IMPORTED",
            "2000 0202 ; > RESERVE",
            "2010 0206",
            "0012 0002",
            "4200 0004",
            "4206 8012",
            "6100 4200",
            "6102 9202",
            "4104 C000",
        ];
        let program = lines.join("\n");
        let program = AddressedProgram::parse_machine_code(
            program.as_str().into()
        ).unwrap().1;
        for (i, (source_line, parsed_line)) in lines.into_iter().zip(program.into_iter()).enumerate() {
            let i = (i + 1).try_into().unwrap();
            let mut addressed_line = AddressedLine::parse_machine_code(source_line.into()).unwrap().1;
            addressed_line.operation.instruction.position.line = i;
            addressed_line.operation.operand.position.line = i;
            let relacional_annotation = if let Some(line) = addressed_line.relational_annotation {
                let Operation { instruction, operand } = line.operation;
                Some(Line::new(None, Operation::new(
                    Token::new(Position::new(i, instruction.position.column), instruction.value),
                    Token::new(Position::new(i, operand.position.column), operand.value)
                )))
            } else {
                None
            };
            addressed_line.relational_annotation = relacional_annotation;
            assert_eq!(
                addressed_line,
                parsed_line,
            );
        }
    }
}
