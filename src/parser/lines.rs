use std::collections::BTreeMap;

use nom::IResult;
use nom::combinator::map;
use nom::character::complete::line_ending;

use crate::parser::line::Line;

use super::{label::Label, operation::Operation, line::comment_or_space, util::separated_list1_opt};

#[derive(Debug, PartialEq)]
pub struct Lines<'a>(Vec<Line<'a>>);

impl<'a> Lines<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        Lines(lines)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            separated_list1_opt(
                line_ending,
                Line::parse,
                comment_or_space,
            ),
            |lines| Self::new(lines)
        )(input)
    }
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::parser::{operand::Operand, mneumonic::Mneumonic, operation::Operation, label::Label};

    use super::*;


    #[test]
    fn should_returns_code_given_asm() {
        let input = indoc! {"
            LOOP    LV  /0
                    JP LOOP
        "};
        let expected = Lines(vec![
             Line::new(Some(Label::new("LOOP")), Operation::new(Mneumonic::LoadValue, Operand::new_numeric(0))),
             Line::new(None, Operation::new(Mneumonic::Jump, Operand::new_simbolic(Label::new("LOOP")))),
        ]);
        assert_eq!(Lines::parse(input), Ok(("", expected)));
    }

    #[test]
    fn should_returns_code_given_asm_blank_lines() {
        let input = indoc! {"

            LOOP    LV  /0

            # End loop
                    JP LOOP

        "};
        let expected = Lines(vec![
             Line::new(Some(Label::new("LOOP")), Operation::new(Mneumonic::LoadValue, Operand::new_numeric(0))),
             Line::new(None, Operation::new(Mneumonic::Jump, Operand::new_simbolic(Label::new("LOOP")))),
        ]);
        assert_eq!(Lines::parse(input), Ok(("", expected)));
    }
}
