use std::collections::BTreeMap;

use nom::{character::complete::line_ending, combinator::map, multi::separated_list1, IResult, sequence::terminated};

use crate::parser::line::Line;

use super::{label::Label, operation::Operation};

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

    fn unzip(self) -> (Vec<Option<Label<'a>>>, Vec<Operation<'a>>) {
        self.0.into_iter().map(|l| l.unwrap()).unzip()
    }

    pub fn value(self) -> Result<Vec<u16>, String> {
        let (labels, operations) = self.unzip();
        let labels_map = create_labels_map(labels);
        operations.into_iter().map(|op| op.value(&labels_map)).collect()
    }
}
     
fn create_labels_map(label_lines: Vec<Option<Label>>) -> BTreeMap<Label, u16> {
    label_lines.into_iter()
        .enumerate()
        // (usize, Option<Label>) -> Option<(Label, u16)>
        .flat_map(|(index, opt_label)| opt_label.map(|label| (label, index as u16 * 2)))
        .collect()
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
    fn should_returns_asm_bin() {
        let input = Lines(vec![
             Line::new(None, Operation::new(Mneumonic::Jump, Operand::new_numeric(0))),
             Line::new(Some(Label::new("LOOP")), Operation::new(Mneumonic::LoadValue, Operand::new_numeric(0))),
             Line::new(None, Operation::new(Mneumonic::Jump, Operand::new_simbolic(Label::new("LOOP")))),
        ]);

        let expected = vec![
            0x0000,
            0x3000,
            0x0002
        ];
        assert_eq!(input.value(), Ok(expected));
    }
}
