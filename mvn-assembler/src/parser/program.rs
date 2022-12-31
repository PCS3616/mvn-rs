use nom::character::complete::line_ending;
use nom::combinator::map;
use types;

use super::error::{LocatedIResult, Span};
use super::Parse;
use super::{comment_or_space, separated_list1_opt};

impl<'a> Parse<'a> for types::Program<'a> {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
        map(
            separated_list1_opt(line_ending, types::Line::parse, comment_or_space),
            |lines| Self::new(lines),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use types::mneumonic::*;
    use types::*;

    use super::*;

    #[test]
    fn should_returns_code_given_asm() {
        let input = Span::new(indoc! {"
            LOOP    LV  /0
                    JP LOOP
        "});
        let expected = Program::new(vec![
            Line::new(
                Some(Label::new("LOOP")),
                Operation::new(
                    Instruction::Normal(NormalMneumonic::LoadValue),
                    Operand::new_numeric(0),
                ),
            ),
            Line::new(
                None,
                Operation::new(
                    Instruction::Normal(NormalMneumonic::Jump),
                    Operand::new_symbolic(Label::new("LOOP")),
                ),
            ),
        ]);
        assert_eq!(Program::parse(input).unwrap().1, expected);
    }

    #[test]
    fn should_returns_code_given_asm_blank_lines() {
        let input = Span::new(indoc! {"

            LOOP    LV  /0

            ; End loop
                    JP LOOP

        "});
        let expected = Program::new(vec![
            Line::new(
                Some(Label::new("LOOP")),
                Operation::new(
                    Instruction::Normal(NormalMneumonic::LoadValue),
                    Operand::new_numeric(0),
                ),
            ),
            Line::new(
                None,
                Operation::new(
                    Instruction::Normal(NormalMneumonic::Jump),
                    Operand::new_symbolic(Label::new("LOOP")),
                ),
            ),
        ]);
        assert_eq!(Program::parse(input).unwrap().1, expected);
    }
}
