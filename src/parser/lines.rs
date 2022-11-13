use nom::IResult;
use nom::combinator::map;
use nom::character::complete::line_ending;

use crate::parser::line::Line;

use super::{line::comment_or_space, util::separated_list1_opt};

#[derive(Debug, PartialEq)]
pub struct Lines<'a>(
    pub Vec<Line<'a>>
);

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

impl<'a> IntoIterator for Lines<'a> {
    type Item = Line<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }

}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::parser::{operand::Operand, mneumonic::NormalMneumonic, operation::Operation, label::Label, instruction::Instruction};
    use pretty_assertions::assert_eq;

    use super::*;


    #[test]
    fn should_returns_code_given_asm() {
        let input = indoc! {"
            LOOP    LV  /0
                    JP LOOP
        "};
        let expected = Lines(vec![
             Line::new(Some(Label::new("LOOP")), Operation::new(Instruction::Normal(NormalMneumonic::LoadValue), Operand::new_numeric(0))),
             Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::Jump), Operand::new_simbolic(Label::new("LOOP")))),
        ]);
        assert_eq!(Lines::parse(input), Ok(("", expected)));
    }

    #[test]
    fn should_returns_code_given_asm_blank_lines() {
        let input = indoc! {"

            LOOP    LV  /0

            -- End loop
                    JP LOOP

        "};
        let expected = Lines(vec![
             Line::new(Some(Label::new("LOOP")), Operation::new(Instruction::Normal(NormalMneumonic::LoadValue), Operand::new_numeric(0))),
             Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::Jump), Operand::new_simbolic(Label::new("LOOP")))),
        ]);
        assert_eq!(Lines::parse(input), Ok(("", expected)));
    }
}
