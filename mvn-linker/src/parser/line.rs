use nom::character::complete::space1;
use nom::sequence::tuple;

use assembler::parser::Parse as ParseAssembler;
use types;
use utils::comment_or_space;

use super::address::MachineAddress;
use super::error;
use super::Parse;

#[derive(Debug, PartialEq)]
pub struct AddressedLine<'a> {
    pub address: MachineAddress,
    pub operation: types::Operation<'a>,
    pub relational_annotation: Option<types::Line<'a>>,
}

impl<'a> AddressedLine<'a> {
    pub fn new(address: MachineAddress, operation: types::Operation<'a>, relational_annotation: Option<types::Line<'a>>) -> Self{
        Self { address, operation, relational_annotation }
    }
}

impl<'a> Parse<'a> for AddressedLine<'a> {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (rest, (address, _, operation, comment)) = tuple((
            MachineAddress::parse_machine_code,
            space1,
            types::Operation::parse_machine_code,
            comment_or_space,
        ))(input)?;
        let relational_annotation = match comment {
            Some(annotation) => {
                let annotation = types::Line::parse_assembler(annotation);
                match annotation {
                    Ok((_, line)) => match line.operation.instruction.value {
                        types::Instruction::Relational(_)  => Some(line),
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


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::{Operation, Line, Token, Position, Instruction, Operand, mneumonic::{NormalMneumonic, RelationalMneumonic}};
    use super::*;
    use crate::parser::address::{MachineAddress, MachineAddressProperties};

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
                    MachineAddress::new(MachineAddressProperties::new(false, false, false), 0),
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
                    MachineAddress::new(MachineAddressProperties::new(false, false, false), 0),
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
                    MachineAddress::new(MachineAddressProperties::new(false, false, false), 0),
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
