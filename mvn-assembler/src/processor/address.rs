use std::collections::BTreeMap;

use types::{
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
    pub position: u16,
    pub relocatable: bool,
    pub imported: bool,
    pub exported: bool,
}

pub type LabelMap<'a> = BTreeMap<Label<'a>, Address>;

impl<'a> AddressedProgram<'a> {
    pub fn process(program: Program<'a>) -> AddressedProgram<'a> {
        let mut position: u16 = 0;
        let mut import_counter: u16 = 0;
        let mut addresses: Vec<Address> = Vec::new();
        let mut relocatable = false;

        for line in &program.lines {
            let Operation {
                instruction,
                operand,
            } = &line.operation;
            let address_position: u16 = if let Instruction::Relational(mneumonic) = instruction {
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
                AddressedProgram::resolve_address_metadata(instruction, &mut relocatable, address);
            addresses.push(address);
            position = AddressedProgram::resolve_next_position(instruction, operand, position);
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
        current_position: u16,
    ) -> u16 {
        match instruction {
            Instruction::Normal(_) => current_position + 2,
            Instruction::Positional(mneumonic) => {
                if let Operand::Numeric(operand) = operand {
                    let operand = *operand;
                    match mneumonic {
                        PositionalMneumonic::ReserveMemory => current_position + operand,
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

    pub fn map_labels(&'a self) -> LabelMap<'a> {
        let mut label_vector: Vec<(Label, Address)> = Vec::new();
        for AddressedLine { address, line } in &self.lines {
            if let Some(label) = &line.label {
                label_vector.push((label.clone(), address.clone()));
            } else if let Instruction::Relational(mneumonic) = &line.operation.instruction {
                if let RelationalMneumonic::Import = mneumonic {
                    if let Operand::Symbolic(label) = &line.operation.operand {
                        label_vector.push((label.clone(), address.clone()));
                    }
                }
            }
        }
        label_vector.into_iter().collect()
    }

    fn new(lines: Vec<AddressedLine<'a>>) -> Self {
        Self { lines }
    }
}

impl<'a> AddressedLine<'a> {
    fn new(address: Address, line: types::Line<'a>) -> Self {
        Self { address, line }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::parser::error::Span;
    use crate::parser::Parse;
    use types::{Label, Line, Program};

    use super::*;

    #[test]
    fn should_resolve_addresses_without_pseudoinstructions() {
        let input = Program::parse(Span::new(indoc! {"
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
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 2,
                    ..Default::default()
                },
                line: Line::parse(Span::new("K /FFFF")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 4,
                    ..Default::default()
                },
                line: Line::parse(Span::new("AD /0001")).unwrap().1,
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_resolve_imported_and_exported_addresses() {
        let input = Program::parse(Span::new(indoc! {"
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
                line: Line::parse(Span::new("> EXPORTED")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0,
                    imported: true,
                    ..Default::default()
                },
                line: Line::parse(Span::new("< IMPORTED1")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 1,
                    imported: true,
                    ..Default::default()
                },
                line: Line::parse(Span::new("< IMPORTED2")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 2,
                    imported: true,
                    ..Default::default()
                },
                line: Line::parse(Span::new("< IMPORTED3")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0,
                    exported: false,
                    imported: false,
                    ..Default::default()
                },
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_set_absolute_address() {
        let input = Program::parse(Span::new(indoc! {"
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
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
            // On the second line, position is meaningless
            AddressedLine {
                address: Address {
                    position: 2,
                    ..Default::default()
                },
                line: Line::parse(Span::new("@ /100")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x100,
                    ..Default::default()
                },
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_resolve_relocatable_addresses() {
        let input = Program::parse(Span::new(indoc! {"
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
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 2,
                    relocatable: true,
                    ..Default::default()
                },
                line: Line::parse(Span::new("& /100")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x100,
                    relocatable: true,
                    ..Default::default()
                },
                line: Line::parse(Span::new("AD /001")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x102,
                    relocatable: false,
                    ..Default::default()
                },
                line: Line::parse(Span::new("@ /010")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x10,
                    relocatable: false,
                    ..Default::default()
                },
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_resolve_reserved_memory_addresses() {
        let input = Program::parse(Span::new(indoc! {"
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
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x2,
                    ..Default::default()
                },
                line: Line::parse(Span::new("$ /2")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x4,
                    ..Default::default()
                },
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x6,
                    ..Default::default()
                },
                line: Line::parse(Span::new("$ /10")).unwrap().1,
            },
            AddressedLine {
                address: Address {
                    position: 0x16,
                    ..Default::default()
                },
                line: Line::parse(Span::new("JP /0")).unwrap().1,
            },
        ]);
        assert_eq!(AddressedProgram::process(input), expected);
    }

    #[test]
    fn should_map_labels_without_import_export() {
        let input = AddressedProgram::process(
            Program::parse(Span::new(indoc! {"
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
                    position: 0x112,
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
            Program::parse(Span::new(indoc! {"
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
}
