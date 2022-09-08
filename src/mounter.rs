use crate::parser::lines::Lines;

pub fn parse_and_mount_str(input: &str) -> String {
    let (_, lines) = Lines::parse(input).unwrap();
    mount(lines).unwrap().into_iter().map(|l| format!("{:04X}", l)).collect::<Vec<_>>().join("\n")
}

pub fn mount(lines: Lines) -> Result<Vec<u16>, &str> {
    Ok(vec![0x12])
}

fn create_labels_map(label_lines: Vec<Option<Label>>) -> BTreeMap<Label, u16> {
    label_lines.into_iter()
        .enumerate()
        // (usize, Option<Label>) -> Option<(Label, u16)>
        .flat_map(|(index, opt_label)| opt_label.map(|label| (label, index as u16 * 2)))
        .collect()
}

pub fn operand_value(&self, label_value: &BTreeMap<Label, u16>) -> Result<u16, String> {
    match self {
        Self::Numeric(value) => Ok(*value),
        Self::Simbolic(label) => match label_value.get(label) {
            Some(value) => Some(*value),
            None => None,
        }
        .ok_or(format!("{label} not found")),
    }
}

pub fn operation_value(&self, label_value: &BTreeMap<Label, u16>) -> Result<u16, String> {
    let operand_value = self.operand.value(&label_value)?;
    Ok(combine(self.mneumonic.value(), operand_value))
}

 pub fn line_value(self) -> Result<Vec<u16>, String> {
    let (labels, operations) = self.unzip();
    let labels_map = create_labels_map(labels);
    operations.into_iter().map(|op| op.value(&labels_map)).collect()
}

fn combine(mneumonic_value: u8, label_value: u16) -> u16 {
    ((mneumonic_value as u16) << 12) | label_value
}

#[cfg(test)]
mod tests {
    use crate::{parser::{mneumonic::Mneumonic, operand::Operand, operation::Operation, intruction::Instruction, lines::Lines, line::Line, label::Label}, mounter::mount};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_returns_asm_bin() {
        let input = Lines::new(vec![
             Line::new(None, Operation::new(Instruction::Real(Mneumonic::Jump), Operand::new_numeric(0))),
             Line::new(Some(Label::new("LOOP")), Operation::new(Instruction::Real(Mneumonic::LoadValue), Operand::new_numeric(0))),
             Line::new(None, Operation::new(Instruction::Real(Mneumonic::Jump), Operand::new_simbolic(Label::new("LOOP")))),
        ]);

        let expected = vec![
            0x0000,
            0x3000,
            0x0002
        ];
        assert_eq!(mount(input)
        mount(input), Ok(expected));
    }
}
