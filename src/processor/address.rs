use std::collections::BTreeMap;

use crate::parser::{lines::Lines, line::Line, label::Label, instruction::Instruction, mneumonic::{PositionalMneumonic, RelationalMneumonic}, operation::Operation, operand::Operand};

#[derive(Debug, PartialEq, Default)]
pub struct Address {
    pub position: u16,
    pub relocatable: bool,
    pub imported: bool,
    pub exported: bool,
}

#[derive(Debug, PartialEq)]
pub struct AddressedLine<'a> {
    pub address: Address,
    pub line: Line<'a>,
}

#[derive(Debug, PartialEq)]
pub struct AddressedLines<'a>(Vec<AddressedLine<'a>>);

type LabelMap<'a> = BTreeMap<Label<'a>, &'a Address>;
impl<'a> AddressedLines<'a> {

    pub fn parse(program: Lines<'a>) -> AddressedLines<'a> {
        let mut position: u16 = 0;
        let mut import_counter: u16 = 0;
        let mut addresses: Vec<Address> = Vec::new();
        let mut relocatable = false;

        for line in &program.0 {
            let Operation { instruction, operand } = &line.1;
            let address_position: u16 = if let Instruction::Relational(mneumonic) = instruction {
                if let RelationalMneumonic::Import = mneumonic {
                    import_counter += 1;
                    import_counter-1
                } else { position }
            } else { position };

            let address = Address { position: address_position, ..Default::default() };
            let address = AddressedLines::resolve_address_metadata(instruction, &mut relocatable, address);
            addresses.push(address);
            position = AddressedLines::resolve_next_position(instruction, operand, position);
        }

        AddressedLines(
            std::iter::zip(addresses, program)
                .map(|(address, line)| AddressedLine{address, line})
                .collect()
        )
    }

    fn resolve_address_metadata(instruction: &Instruction, relocatable: &mut bool, address: Address) -> Address {
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

        Address { relocatable: *relocatable, imported, exported, ..address }
    }

    fn resolve_next_position(instruction: &Instruction,  operand: &Operand, current_position: u16) -> u16 {
        match instruction {
            Instruction::Normal(_) => current_position + 2,
            Instruction::Positional(mneumonic) => {
                if let Operand::Numeric(operand) = operand {
                    let operand = *operand;
                    match mneumonic {
                        PositionalMneumonic::ReserveMemory => current_position + operand,
                        PositionalMneumonic::SetAbsoluteOrigin | PositionalMneumonic::SetRelocatableOrigin => operand,
                        _ => current_position,
                    }
                } else {
                    current_position
                }
            },
            _ => current_position,
        }
    }

    pub fn map_labels(&'a self) -> LabelMap<'a> {
        let mut label_vector: Vec<(Label, &Address)> = Vec::new();
        for AddressedLine { address, line } in &self.0 {
            if let Some(label) = &line.0 {
                label_vector.push((label.clone(), address));
            } else if let Instruction::Relational(mneumonic) = &line.1.instruction {
                if let RelationalMneumonic::Import = mneumonic {
                    if let Operand::Simbolic(label) = &line.1.operand {
                        label_vector.push((label.clone(), address));
                    }
                }
            }
        }
        label_vector.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {

    use indoc::indoc;

    use crate::parser::{line::Line, lines::Lines, label::Label};

    use super::*;

    #[test]
    fn should_resolve_addresses_without_pseudoinstructions() {
        let input = Lines::parse(indoc! {"
            JP /0
            K /FFFF
            ; Test if comments are ignored
            AD /0001
        "}).unwrap().1;
        let expected = AddressedLines(vec![
            AddressedLine { address: Address { position: 0, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
            AddressedLine { address: Address { position: 2, ..Default::default() }, line: Line::parse("K /FFFF").unwrap().1 },
            AddressedLine { address: Address { position: 4, ..Default::default() }, line: Line::parse("AD /0001").unwrap().1 },
        ]);
        assert_eq!(AddressedLines::parse(input), expected);
    }

    #[test]
    fn should_resolve_imported_and_exported_addresses() {
        let input = Lines::parse(indoc! {"
            > EXPORTED
            ; Position for imported symbols reflects the order they
            ; were imported in, starting from 0
            < IMPORTED1
            < IMPORTED2
            < IMPORTED3
            ; Test if value is neither imported nor exported
            JP /0
        "}).unwrap().1;
        let expected = AddressedLines(vec![
            AddressedLine { address: Address { position: 0, exported: true, ..Default::default() }, line: Line::parse("> EXPORTED").unwrap().1 },
            AddressedLine { address: Address { position: 0, imported: true, ..Default::default() }, line: Line::parse("< IMPORTED1").unwrap().1 },
            AddressedLine { address: Address { position: 1, imported: true, ..Default::default() }, line: Line::parse("< IMPORTED2").unwrap().1 },
            AddressedLine { address: Address { position: 2, imported: true, ..Default::default() }, line: Line::parse("< IMPORTED3").unwrap().1 },
            AddressedLine { address: Address { position: 0, exported: false, imported: false, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
        ]);
        assert_eq!(AddressedLines::parse(input), expected);
    }

    #[test]
    fn should_set_absolute_address() {
        let input = Lines::parse(indoc! {"
            JP /0
            @ /100
            JP /0
        "}).unwrap().1;
        let expected = AddressedLines(vec![
            AddressedLine { address: Address { position: 0, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
            // On the second line, position is meaningless
            AddressedLine { address: Address { position: 2, ..Default::default() }, line: Line::parse("@ /100").unwrap().1 },
            AddressedLine { address: Address { position: 0x100, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
        ]);
        assert_eq!(AddressedLines::parse(input), expected);
    }

    #[test]
    fn should_resolve_relocatable_addresses() {
        let input = Lines::parse(indoc! {"
            JP /0
            & /100 ; Instructions after this should be relocatable
            AD /001
            @ /010 ; Instructions after this should NOT be relocatable
            JP /0
        "}).unwrap().1;
        let expected = AddressedLines(vec![
            AddressedLine { address: Address { position: 0, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
            AddressedLine { address: Address { position: 2, relocatable: true, ..Default::default() }, line: Line::parse("& /100").unwrap().1 },
            AddressedLine { address: Address { position: 0x100, relocatable: true, ..Default::default() }, line: Line::parse("AD /001").unwrap().1 },
            AddressedLine { address: Address { position: 0x102, relocatable: false, ..Default::default() }, line: Line::parse("@ /010").unwrap().1 },
            AddressedLine { address: Address { position: 0x10, relocatable: false, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
        ]);
        assert_eq!(AddressedLines::parse(input), expected);
    }

    #[test]
    fn should_resolve_reserved_memory_addresses() {
        let input = Lines::parse(indoc! {"
            JP /0
            $ /2
            JP /0
            $ /10
            JP /0
        "}).unwrap().1;
        let expected = AddressedLines(vec![
            AddressedLine { address: Address { position: 0x0, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
            AddressedLine { address: Address { position: 0x2, ..Default::default() }, line: Line::parse("$ /2").unwrap().1 },
            AddressedLine { address: Address { position: 0x4, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
            AddressedLine { address: Address { position: 0x6, ..Default::default() }, line: Line::parse("$ /10").unwrap().1 },
            AddressedLine { address: Address { position: 0x16, ..Default::default() }, line: Line::parse("JP /0").unwrap().1 },
        ]);
        assert_eq!(AddressedLines::parse(input), expected);
    }

    #[test]
    fn should_map_labels_without_import_export() {
        let input = AddressedLines::parse(
            Lines::parse(indoc! {"
                TEST00 JP /0
                TEST01 JP /0
                @ /100
                TEST10 JP /0
                $ /10
                TEST11 JP /0
                & /200
                TEST20 JP /0
                # THEEND
            "}).unwrap().1
        );
        let expected_addresses = [
            Address {position: 0x0, ..Default::default()},
            Address {position: 0x2, ..Default::default()},
            Address {position: 0x100, relocatable: false, ..Default::default()},
            Address {position: 0x112, relocatable: false, ..Default::default()},
            Address {position: 0x200, relocatable: true, ..Default::default()},
        ];
        let expected = LabelMap::from([
            (Label("TEST00"), &expected_addresses[0]),
            (Label("TEST01"), &expected_addresses[1]),
            (Label("TEST10"), &expected_addresses[2]),
            (Label("TEST11"), &expected_addresses[3]),
            (Label("TEST20"), &expected_addresses[4]),
        ]);
        assert_eq!(input.map_labels(), expected);
    }

    #[test]
    fn should_map_import_export_labels() {
        let input = AddressedLines::parse(
            Lines::parse(indoc! {"
                > EXPORT0
                > EXPORT1
                < IMPORT0
                < IMPORT1
                NORMAL  JP /0
                EXPORT0 JP /0
                EXPORT1 JP /0
            "}).unwrap().1
        );
        let expected_addresses = [
            Address {position: 0x0, imported: true, ..Default::default()},
            Address {position: 0x1, imported: true, ..Default::default()},
            Address {position: 0x0, ..Default::default()},
            Address {position: 0x2, ..Default::default()},
            Address {position: 0x4, ..Default::default()},
        ];
        let expected = LabelMap::from([
            (Label("IMPORT0"), &expected_addresses[0]),
            (Label("IMPORT1"), &expected_addresses[1]),
            (Label("NORMAL"), &expected_addresses[2]),
            (Label("EXPORT0"), &expected_addresses[3]),
            (Label("EXPORT1"), &expected_addresses[4]),
        ]);
        assert_eq!(input.map_labels(), expected);
    }

}
