use nom;
use nom::character::complete::line_ending;
use nom::combinator::{map, value, eof};
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, pair};
use types;

use super::error::{LocatedIResult, Span};
use super::Parse;
use super::comment_or_space;

fn ignorable<'a>(input: Span<'a>) -> LocatedIResult<'a, ()> {
    value((), many0(pair(comment_or_space, line_ending)))(input)
}

impl<'a> Parse<'a> for types::Program<'a> {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
        map(
            many_till(
                delimited(
                    ignorable,
                    types::Line::parse,
                    ignorable,
                ),
                eof,
            ),
            |(lines, _)| Self::new(lines)
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
    fn should_returnscode_given_asm() {
        let input = Span::new(indoc! {"LOOP    LV  /0\nJP LOOP"});
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
