use std::collections::BTreeMap;

use crate::types::{
    mneumonic::{PositionalMneumonic, RelationalMneumonic},
    Instruction, Label, Line, Operand, Operation, Program,
};

#[derive(Debug, PartialEq)]
pub struct AddressedProgram<'a> {
    pub lines: Vec<AddressedLine<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct AddressedLine<'a> {
    pub address: Address,
    pub line: Line<'a>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Address {
    pub position: u32,
    pub relocatable: bool,
    pub imported: bool,
    pub exported: bool,
}

pub type LabelMap<'a> = BTreeMap<Label<'a>, Address>;

impl<'a> AddressedProgram<'a> {
    pub fn process(program: Program<'a>) -> AddressedProgram<'a> {
        let mut position = 0;
        let mut import_counter = 0;
        let mut addresses: Vec<Address> = Vec::new();
        let mut relocatable = false;

        for line in &program.lines {
            let Operation {
                instruction,
                operand,
            } = &line.operation;
            let address_position = if let Instruction::Relational(mneumonic) = instruction.value {
                if let RelationalMneumonic::Import = mneumonic {
                    import_counter += 1;
                    import_counter - 1
                } else {
                    position
                }
            } else {
                position
            };

            let address = Address {
                position: address_position,
                ..Default::default()
            };
            let address =
                AddressedProgram::resolve_address_metadata(&instruction.value, &mut relocatable, address);
            addresses.push(address);
            position = AddressedProgram::resolve_next_position(&instruction.value, &operand.value, position);
        }

        AddressedProgram::new(
            std::iter::zip(addresses, program)
                .map(|(address, line)| AddressedLine::new(address, line))
                .collect(),
        )
    }

    fn resolve_address_metadata(
        instruction: &Instruction,
        relocatable: &mut bool,
        address: Address,
    ) -> Address {
        let mut imported = false;
        let mut exported = false;

        match instruction {
            Instruction::Normal(_) => (),
            Instruction::Positional(mneumonic) => match mneumonic {
                PositionalMneumonic::SetAbsoluteOrigin => *relocatable = false,
                PositionalMneumonic::SetRelocatableOrigin => *relocatable = true,
                _ => (),
            },
            Instruction::Relational(mneumonic) => match mneumonic {
                RelationalMneumonic::Export => exported = true,
                RelationalMneumonic::Import => imported = true,
            },
        }

        Address {
            relocatable: *relocatable,
            imported,
            exported,
            ..address
        }
    }

    fn resolve_next_position(
        instruction: &Instruction,
        operand: &Operand,
        current_position: u32,
    ) -> u32 {
        match instruction {
            Instruction::Normal(_) => current_position + 2,
            Instruction::Positional(mneumonic) => {
                if let Operand::Numeric(operand) = operand {
                    let operand = *operand;
                    match mneumonic {
                        // Memory reservers are specified in 16b words, while position is in bytes
                        PositionalMneumonic::ReserveMemory => current_position + 2 * operand,
                        PositionalMneumonic::SetAbsoluteOrigin
                        | PositionalMneumonic::SetRelocatableOrigin => operand,
                        _ => current_position,
                    }
                } else {
                    current_position
                }
            }
            _ => current_position,
        }
    }

    pub fn map_labels(&self) -> LabelMap<'a> {
        let mut label_vector: Vec<(Label, Address)> = Vec::new();
        for AddressedLine { address, line } in &self.lines {
            if let Some(label) = &line.label {
                label_vector.push((label.value.clone(), address.clone()));
            } else if let Instruction::Relational(mneumonic) = &line.operation.instruction.value {
                if let RelationalMneumonic::Import = mneumonic {
                    if let Operand::Symbolic(label) = &line.operation.operand.value {
                        label_vector.push((
                            label.clone(),
                            Address {imported: true, position: address.position, ..Default::default()}
                        ));
                    }
                }
            }
        }
        label_vector.into_iter().collect()
    }

    pub fn new(lines: Vec<AddressedLine<'a>>) -> Self {
        Self { lines }
    }
}

impl<'a> AddressedLine<'a> {
    pub fn new(address: Address, line: Line<'a>) -> Self {
        Self { address, line }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use utils::types::*;
    use crate::parser::error::Span;
    use crate::parser::Parse;
    use crate::types::mneumonic::*;
    use super::*;

    #[test]
    fn should_resolve_addresses_without_pseudoinstructions() {
        let input = Program::parse_assembler(Span::new(indoc! {"
            JP /0
            K /FFFF
            ; Test if comments are ignored
            AD /0001
        "}))
        .unwrap()
        .1;
        let expected = AddressedProgram::new(vec![
            AddressedLine {
                address: Address {
                    position: 0,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(1, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(1, 4), Operand::from(0)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 2,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(2, 1), Instruction::Normal(NormalMneumonic::SetConstant)),
                    Token::new(Position::new(2, 3), Operand::from(0xFFFF)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 4,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(4, 1), Instruction::Normal(NormalMneumonic::Add)),
                    Token::new(Position::new(4, 4), Operand::from(1)),
                )),
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_resolve_imported_and_exported_addresses() {
        let input = Program::parse_assembler(Span::new(indoc! {"
            > EXPORTED
            ; Position for imported symbols reflects the order they
            ; were imported in, starting from 0
            < IMPORTED1
            < IMPORTED2
            < IMPORTED3
            ; Test if value is neither imported nor exported
            JP /0
        "}))
        .unwrap()
        .1;
        let expected = AddressedProgram::new(vec![
            AddressedLine {
                address: Address {
                    position: 0,
                    exported: true,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(1, 1), Instruction::Relational(RelationalMneumonic::Export)),
                    Token::new(Position::new(1, 3), Operand::from("EXPORTED")),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0,
                    imported: true,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(4, 1), Instruction::Relational(RelationalMneumonic::Import)),
                    Token::new(Position::new(4, 3), Operand::from("IMPORTED1")),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 1,
                    imported: true,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(5, 1), Instruction::Relational(RelationalMneumonic::Import)),
                    Token::new(Position::new(5, 3), Operand::from("IMPORTED2")),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 2,
                    imported: true,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(6, 1), Instruction::Relational(RelationalMneumonic::Import)),
                    Token::new(Position::new(6, 3), Operand::from("IMPORTED3")),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0,
                    exported: false,
                    imported: false,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(8, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(8, 4), Operand::from(0)),
                )),
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_set_absolute_address() {
        let input = Program::parse_assembler(Span::new(indoc! {"
            JP /0
            @ /100
            JP /0
        "}))
        .unwrap()
        .1;
        let expected = AddressedProgram::new(vec![
            AddressedLine {
                address: Address {
                    position: 0,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(1, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(1, 4), Operand::from(0)),
                )),
            },
            // On the second line, position is meaningless
            AddressedLine {
                address: Address {
                    position: 2,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(2, 1), Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin)),
                    Token::new(Position::new(2, 3), Operand::from(0x100)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x100,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(3, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(3, 4), Operand::from(0)),
                )),
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_resolve_relocatable_addresses() {
        let input = Program::parse_assembler(Span::new(indoc! {"
            JP /0
            & /100 ; Instructions after this should be relocatable
            AD /001
            @ /010 ; Instructions after this should NOT be relocatable
            JP /0
        "}))
        .unwrap()
        .1;
        let expected = AddressedProgram::new(vec![
            AddressedLine {
                address: Address {
                    position: 0,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(1, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(1, 4), Operand::from(0)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 2,
                    relocatable: true,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(2, 1), Instruction::Positional(PositionalMneumonic::SetRelocatableOrigin)),
                    Token::new(Position::new(2, 3), Operand::from(0x100)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x100,
                    relocatable: true,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(3, 1), Instruction::Normal(NormalMneumonic::Add)),
                    Token::new(Position::new(3, 4), Operand::from(1)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x102,
                    relocatable: false,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(4, 1), Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin)),
                    Token::new(Position::new(4, 3), Operand::from(0x10)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x10,
                    relocatable: false,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(5, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(5, 4), Operand::from(0)),
                )),
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_resolve_reserved_memory_addresses() {
        let input = Program::parse_assembler(Span::new(indoc! {"
            JP /0
            $ /1
            JP /0
            $ /2
            JP /0
            $ /10
            JP /0
        "}))
        .unwrap()
        .1;
        let expected = AddressedProgram::new(vec![
            AddressedLine {
                address: Address {
                    position: 0x0,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(1, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(1, 4), Operand::from(0)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x2,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(2, 1), Instruction::Positional(PositionalMneumonic::ReserveMemory)),
                    Token::new(Position::new(2, 3), Operand::from(1)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x4,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(3, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(3, 4), Operand::from(0)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x6,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(4, 1), Instruction::Positional(PositionalMneumonic::ReserveMemory)),
                    Token::new(Position::new(4, 3), Operand::from(2)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0xA,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(5, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(5, 4), Operand::from(0)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0xC,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(6, 1), Instruction::Positional(PositionalMneumonic::ReserveMemory)),
                    Token::new(Position::new(6, 3), Operand::from(0x10)),
                )),
            },
            AddressedLine {
                address: Address {
                    position: 0x2C,
                    ..Default::default()
                },
                line: Line::new(None, Operation::new(
                    Token::new(Position::new(7, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(7, 4), Operand::from(0)),
                )),
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_map_labels_without_import_export() {
        let input = AddressedProgram::process(
            Program::parse_assembler(Span::new(indoc! {"
                TEST00 JP /0
                TEST01 JP /0
                @ /100
                TEST10 JP /0
                $ /10
                TEST11 JP /0
                & /200
                TEST20 JP /0
                # THEEND
            "}))
            .unwrap()
            .1,
        );
        let expected = LabelMap::from([
            (
                Label("TEST00"),
                Address {
                    position: 0x0,
                    ..Default::default()
                },
            ),
            (
                Label("TEST01"),
                Address {
                    position: 0x2,
                    ..Default::default()
                },
            ),
            (
                Label("TEST10"),
                Address {
                    position: 0x100,
                    relocatable: false,
                    ..Default::default()
                },
            ),
            (
                Label("TEST11"),
                Address {
                    position: 0x122,
                    relocatable: false,
                    ..Default::default()
                },
            ),
            (
                Label("TEST20"),
                Address {
                    position: 0x200,
                    relocatable: true,
                    ..Default::default()
                },
            ),
        ]);
        assert_eq!(input.map_labels(), expected);
    }

    #[test]
    fn should_map_import_export_labels() {
        let input = AddressedProgram::process(
            Program::parse_assembler(Span::new(indoc! {"
                > EXPORT0
                > EXPORT1
                < IMPORT0
                < IMPORT1
                NORMAL  JP /0
                EXPORT0 JP /0
                EXPORT1 JP /0
            "}))
            .unwrap()
            .1,
        );
        let expected = LabelMap::from([
            (
                Label("IMPORT0"),
                Address {
                    position: 0x0,
                    imported: true,
                    ..Default::default()
                },
            ),
            (
                Label("IMPORT1"),
                Address {
                    position: 0x1,
                    imported: true,
                    ..Default::default()
                },
            ),
            (
                Label("NORMAL"),
                Address {
                    position: 0x0,
                    ..Default::default()
                },
            ),
            (
                Label("EXPORT0"),
                Address {
                    position: 0x2,
                    ..Default::default()
                },
            ),
            (
                Label("EXPORT1"),
                Address {
                    position: 0x4,
                    ..Default::default()
                },
            ),
        ]);
        assert_eq!(input.map_labels(), expected);
    }

    #[test]
    fn imported_labels_should_not_get_line_attributes() {
        let input = AddressedProgram::process(Program::parse_assembler(indoc! {"
            & /0
            < IMPORT
        "}.into()).unwrap().1);
        let expected = LabelMap::from([(
            Label("IMPORT"),
            Address {
                position: 0,
                relocatable: false,
                imported: true,
                ..Default::default()
            }
        )]);
        assert_eq!(input.map_labels(), expected);
    }
}
