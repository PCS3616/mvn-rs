use nom;
use nom::character::complete::line_ending;
use nom::combinator::{map, value, eof};
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, pair};
use types;
use utils::comment_or_space;

use super::error::{LocatedIResult, Span};
use super::Parse;

fn ignorable<'a>(input: Span<'a>) -> LocatedIResult<'a, ()> {
    value((), many0(pair(comment_or_space, line_ending)))(input)
}

impl<'a> Parse<'a> for types::Program<'a> {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        map(
            many_till(
                delimited(
                    ignorable,
                    types::Line::parse_assembler,
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
        let input = indoc! {"LOOP    LV  /0\nJP LOOP"}.into();
        let expected = Program::new(vec![
            Line::new(
                Some(Token::new(Position::new(1, 1), Label::new("LOOP"))),
                Operation::new(
                    Token::new(Position::new(1, 9), Instruction::Normal(NormalMneumonic::LoadValue)),
                    Token::new(Position::new(1, 13), Operand::new_numeric(0)),
                ),
            ),
            Line::new(
                None,
                Operation::new(
                    Token::new(Position::new(2, 1), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(2, 4), Operand::new_symbolic(Label::new("LOOP"))),
                ),
            ),
        ]);
        assert_eq!(Program::parse_assembler(input).unwrap().1, expected);
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
                Some(Token::new(Position::new(3, 1), Label::new("LOOP"))),
                Operation::new(
                    Token::new(Position::new(3, 9), Instruction::Normal(NormalMneumonic::LoadValue)),
                    Token::new(Position::new(3, 13), Operand::new_numeric(0)),
                ),
            ),
            Line::new(
                None,
                Operation::new(
                    Token::new(Position::new(6, 9), Instruction::Normal(NormalMneumonic::Jump)),
                    Token::new(Position::new(6, 12), Operand::new_symbolic(Label::new("LOOP"))),
                ),
            ),
        ]);
        assert_eq!(Program::parse_assembler(input).unwrap().1, expected);
    }
}
