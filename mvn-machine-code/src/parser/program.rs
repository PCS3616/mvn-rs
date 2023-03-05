use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;

use crate::types::{AddressedLine, AddressedProgram};

use super::error;
use super::{AddressPosition, Parse, Relocate};

impl Relocate for AddressedProgram<'_> {
    fn relocate(self, base: AddressPosition) -> Self {
        AddressedProgram::new(
            self.lines
                .into_iter()
                .map(|line| line.relocate(base))
                .collect(),
        )
    }
}

impl<'a> Parse<'a> for AddressedProgram<'a> {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        map(
            separated_list1(line_ending, AddressedLine::parse_machine_code),
            Self::new,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use assembly::types::Line;
    use pretty_assertions::assert_eq;
    use utils::types::*;

    // FIXME Test this function right
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
        let program = AddressedProgram::parse_machine_code(program.as_str().into())
            .unwrap()
            .1;
        for (i, (source_line, parsed_line)) in
            lines.into_iter().zip(program.into_iter()).enumerate()
        {
            let i = (i + 1).try_into().unwrap();
            let mut addressed_line = AddressedLine::parse_machine_code(source_line.into())
                .unwrap()
                .1;
            addressed_line.address.position.line = i;
            addressed_line.operation.instruction.position.line = i;
            addressed_line.operation.operand.position.line = i;
            let relacional_annotation = if let Some(line) = addressed_line.relational_annotation {
                let Operation {
                    instruction,
                    operand,
                } = line.operation;
                Some(Line::new(
                    None,
                    Operation::new(
                        Token::new(
                            Position::new(i, instruction.position.column),
                            instruction.value,
                        ),
                        Token::new(Position::new(i, operand.position.column), operand.value),
                    ),
                ))
            } else {
                None
            };
            addressed_line.relational_annotation = relacional_annotation;
            assert_eq!(addressed_line, parsed_line,);
        }
    }
}
