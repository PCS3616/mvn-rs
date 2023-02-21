use nom::character::complete::space1;
use nom::sequence::tuple;

use assembly::parser::Parse as ParseAssembler;
use utils::{comment_or_space, types::Token};

use crate::types::{MachineAddress, AddressPosition, AddressedLine};

use super::{Parse, Relocate};
use super::error;

impl<'a> Parse<'a> for AddressedLine<'a> {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (rest, (address, _, operation, comment)) = tuple((
            Token::<MachineAddress>::parse_machine_code,
            space1,
            assembly::types::Operation::parse_machine_code,
            comment_or_space,
        ))(input)?;
        let relational_annotation = match comment {
            Some(annotation) => {
                let annotation = assembly::types::Line::parse_assembler(annotation);
                match annotation {
                    Ok((_, line)) => match line.operation.instruction.value {
                        assembly::types::Instruction::Relational(_)  => Some(line),
                        _ => None,
                    },
                    _ => None,
                }
            },
            _ => None,
        };
        Ok((rest, Self::new(address, operation, relational_annotation)))
    }
}

impl Relocate for AddressedLine<'_> {
    fn relocate(self, base: AddressPosition) -> Self {
        let position = self.address.position;
        let address = self.address.value;
        let properties = address.properties;
        let address = if properties.line_relocatable {
            address.relocate(base)
        } else {
            address
        };

        let operation = if properties.operand_relocatable {
            self.operation.relocate(base)
        } else {
            self.operation
        };

        Self::new(Token::new(position, address), operation, self.relational_annotation)
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use utils::types::*;
    use assembly::types::{*, mneumonic::*};
    use super::*;
    use crate::types::{MachineAddress, MachineAddressProperties};

    #[test]
    fn should_parse_lines_with_varying_spacing() {
        let inputs_positions= vec![
            ("0000 0000", 6),
            ("0000  0000", 7),
            ("0000\t0000", 6),
        ];
        for (input, position) in inputs_positions {
            assert_eq!(
                AddressedLine::parse_machine_code(input.into()).unwrap().1,
                AddressedLine::new(
                    Token::new(
                        Position::new(1, 1),
                        MachineAddress::new(MachineAddressProperties::new(false, false, false), 0),
                    ),
                    Operation::new(
                        Token::new(Position::new(1, position), Instruction::Normal(NormalMneumonic::Jump)),
                        Token::new(Position::new(1, position + 1), Operand::from(0)),
                    ),
                    None,
                )
            )
        }
    }

    #[test]
    fn should_parse_lines_with_relational_annotations() {
        let inputs_outputs= vec![
            ("0000 0000 ; > FOO", (13, RelationalMneumonic::Export, "FOO")),
            ("0000 0000; < BAR", (12, RelationalMneumonic::Import, "BAR")),
            ("0000 0000 ;> BAZ", (12, RelationalMneumonic::Export, "BAZ")),
            ("0000 0000;< FOOBAR", (11, RelationalMneumonic::Import, "FOOBAR")),
        ];
        for (input, (position, mneumonic, label)) in inputs_outputs {
            let relational_annotation = Some(
                Line::new(
                    None,
                    Operation::new(
                        Token::new(Position::new(1, position), Instruction::Relational(mneumonic)),
                        Token::new(Position::new(1, position + 2), label.into()),
                    )
                )
            );
            assert_eq!(
                AddressedLine::parse_machine_code(input.into()).unwrap().1,
                AddressedLine::new(
                    Token::new(
                        Position::new(1, 1),
                        MachineAddress::new(MachineAddressProperties::new(false, false, false), 0),
                    ),
                    Operation::new(
                        Token::new(Position::new(1, 6), Instruction::Normal(NormalMneumonic::Jump)),
                        Token::new(Position::new(1, 7), Operand::from(0)),
                    ),
                    relational_annotation,
                )
            )
        }
    }

    #[test]
    fn non_relational_comments_should_not_lead_to_relational_annotations() {
        let inputs= vec![
            ("0000 0000 ; Foo bar"),
            ("0000 0000 ; K /0"),
            ("0000 0000 ; K /0 ; Nested comments"),
            ("0000 0000 ; K ZERO "),
            ("0000 0000 ; XX FOOBAR"),
        ];
        for input in inputs {
            assert_eq!(
                AddressedLine::parse_machine_code(input.into()).unwrap().1,
                AddressedLine::new(
                    Token::new(
                        Position::new(1, 1),
                        MachineAddress::new(MachineAddressProperties::new(false, false, false), 0),
                    ),
                    Operation::new(
                        Token::new(Position::new(1, 6), Instruction::Normal(NormalMneumonic::Jump)),
                        Token::new(Position::new(1, 7), Operand::from(0)),
                    ),
                    None,
                )
            )
        }
    }
}
