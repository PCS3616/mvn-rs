use nom::{character::complete::line_ending, combinator::map, multi::separated_list1, IResult, sequence::terminated};

use crate::parser::line::Line;

#[derive(Debug, PartialEq)]
pub struct Lines<'a>(Vec<Line<'a>>);

impl<'a> Lines<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        Lines(lines)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        terminated(
            map(separated_list1(line_ending, Line::parse), |lines| {
                Self::new(lines)
            }),
            line_ending
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

    /*
    #[test]
    fn should_returns_mvn_given_code() {
        let input = Code(vec![
            Instruction(Some("VAL_A"), Mneumonic::Constant(Operand::Numeric(1))),
            Instruction(Some("VAL_B"), Mneumonic::Constant(Operand::Numeric(2))),
            Instruction(Some("RESLT"), Mneumonic::Constant(Operand::Numeric(0))),
            Instruction(None, Mneumonic::Address(Operand::Numeric(0x100))),
            Instruction(Some("MAIN"), Mneumonic::Load(Operand::Simbolic("VAL_A"))),
            Instruction(None, Mneumonic::Add(Operand::Simbolic("VAL_B"))),
            Instruction(None, Mneumonic::Add(Operand::Simbolic("RESLT"))),
        ]);
        let expected = indoc! {"
            00000001
            00020002
            00040000
            01008000
            01024002
            01049004
        "};

        assert_eq!(input.to_mvn(), expected);
    }

    #[test]
    fn should_parse_and_reuturs_value() {
        let input = indoc! {"
            VAL_A   K   /0001
            VAL_B   K   /0002
            RESLT   K   /0000
                    @   /0100
            MAIN    LD  VAL_A
                    AD  VAL_B
                    MM  RESLT
        "};

        let expected = indoc! {"
            00000001
            00020002
            00040000
            01008000
            01024002
            01049004
        "};

        assert_eq!(Line::parse(input).values()), Ok(expected));
    }
    */
}
