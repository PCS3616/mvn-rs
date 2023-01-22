use utils::error::{MvnParseError, Span};

use crate::processor::address::{AddressedProgram, LabelMap, Address};

type ValidatorResult<'a> = Result<(), nom::Err<MvnParseError<'a>>>;

pub fn validate<'a, 'b>(program: &'a AddressedProgram<'b>, label_map: &'a LabelMap<'b>) -> ValidatorResult<'b> {
    for (line_number, line) in program.lines.iter().enumerate() {
        let validator = LineValidator::new(&line.line, &line.address, line_number as u16, label_map);
        validator.validate()?;
    }
    Ok(())
}

struct LineValidator<'a, 'b> {
    line: &'a types::Line<'b>,
    address: &'a Address,
    line_number: u16,
    label_map: &'a LabelMap<'b>,
}

impl<'b> LineValidator<'_, 'b> {
    pub fn validate(self) -> ValidatorResult<'b> {
        self.numeric_operand_on_import_export()?;
        self.symbolic_operand_on_positional()?;
        self.undefined_label()?;
        self.code_exceeding_address_space()?;
        self.numeric_operand_too_wide()?;
        Ok(())
    }

    /* Every validator function's name should
     * answer the question: "Does the program
     * contain {name}?"
     */

    fn numeric_operand_on_import_export(&self) -> ValidatorResult<'b> {
        match &self.line.operation.instruction {
            types::Instruction::Relational(_) => match &self.line.operation.operand {
                types::Operand::Numeric(_) => nom_failure(
                    "numeric operand cannot be imported nor exported",
                    self.new_span_from_line("")
                ),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }

    fn symbolic_operand_on_positional(&self) -> ValidatorResult<'b> {
        match &self.line.operation.instruction {
            types::Instruction::Positional(mneumonic) => match mneumonic {
                types::mneumonic::PositionalMneumonic::SetEnd => Ok(()),
                _ => match &self.line.operation.operand {
                    types::Operand::Symbolic(_) => nom_failure(
                        "symbolic operand cannot be used to reserve addresses or set positions",
                        self.new_span_from_line("")
                    ),
                    _ => Ok(()),
                },
            }
            _ => Ok(()),
        }
    }

    fn undefined_label(&self) -> ValidatorResult<'b> {
        match &self.line.operation.operand {
            types::Operand::Symbolic(label) => match &self.label_map.get(label) {
                None => nom_failure(
                    "undefined label used as operand",
                    self.new_span_from_line("")
                ),
                Some(_) => Ok(()),
            }
            _ => Ok(()),
        }
    }

    fn code_exceeding_address_space(&self) -> ValidatorResult<'b> {
        if self.address.position > 0xFFF {
            nom_failure("address outside memory", self.new_span_from_line(""))
        } else {
            Ok(())
        }
    }

    // fn implicit_memory_overwrite(&self) -> ValidatorResult {} // TODO Implement

    fn numeric_operand_too_wide(&self) -> ValidatorResult<'b> {
        let immediate = if let types::Operand::Numeric(immediate) = &self.line.operation.operand {
            *immediate
        } else {
            return Ok(());
        };

        match &self.line.operation.instruction {
            types::Instruction::Normal(mneumonic) => match mneumonic {
                types::mneumonic::NormalMneumonic::SetConstant => if immediate > 0xFFFF {
                    nom_failure(
                        "immediate over 16 bits for constant pseudoinstruction",
                        self.new_span_from_line("")
                    )
                } else {
                    Ok(())
                },
                _ => if immediate > 0xFFF {
                    nom_failure(
                        "immediate cannot be larger than 12 bits",
                        self.new_span_from_line("")
                    )
                } else {
                    Ok(())
                },
            },
            _ => Ok(()),
        }
    }
}

impl<'a, 'b> LineValidator<'a, 'b> {
    fn new_span_from_line(&self, fragment: &'b str) -> Span<'b> {
        // Calling this function is safe because we pass zero
        // as the offset rather than an arbitrary value
        unsafe {
            Span::new_from_raw_offset(
                0,
                self.line_number as u32,
                fragment,
                (),
            )
        }
    }

    fn new(line: &'a types::Line<'b>, address: &'a Address, line_number: u16, label_map: &'a LabelMap<'b>) -> Self {
        Self { line, address, line_number, label_map }
    }
}

fn nom_failure<'a>(message: &'static str, span: Span<'a>) -> ValidatorResult<'a> {
    Err(nom::Err::Failure(MvnParseError::new(message.to_owned(), span)))
}

// TODO Implement unit tests
#[cfg(test)]
mod tests {
    use types::{*, mneumonic::*};

    use crate::processor::address::*;

    use super::*;

    /*
     *  < IMPORT
     *  > RESULT
     *          JP  MAIN
     *  ONE     K   /1
     *  TWO     K   /2
     *  RESULT  $   =2
     *  @ /100
     *  MAIN    LD  ONE
     *          AD  TWO
     *          MM  RESULT
     *          LV  /10
     *          AD  RESULT
     *          MM  RESULT
     *  # MAIN
     */

    struct TestProgram {
        import: Operand<'static>,
        constant: u32,
        position: Operand<'static>,
        load_label: Label<'static>,
        load_value: u32,
    }

    impl Default for TestProgram {
        fn default() -> Self {
            Self {
                import: "IMPORT".into(),
                constant: 0x1,
                position: 0x100.into(),
                load_label: "ONE".into(),
                load_value: 0x10,
            }
        }
    }

    impl TestProgram {
        fn validate(self) -> ValidatorResult<'static> {
            let (program, label_map) = self.render();
            validate(&program, &label_map)
        }

        fn render(self) -> (AddressedProgram<'static>, LabelMap<'static>) {
            let main_position = if let Operand::Numeric(immediate) = self.position {
                immediate
            } else {
                0x100
            };
            let program = AddressedProgram::new(vec![
                AddressedLine::new(Address { position: 0, imported: true, ..Default::default() }, Line::new(None, Operation::new(Instruction::Relational(RelationalMneumonic::Import), self.import))),
                AddressedLine::new(Address { position: 0, exported: true, ..Default::default() }, Line::new(None, Operation::new(Instruction::Relational(RelationalMneumonic::Export), "RESULT".into()))),

                AddressedLine::new(Address { position: 0, ..Default::default() }, Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::Jump), "MAIN".into()))),
                AddressedLine::new(Address { position: 2, ..Default::default() }, Line::new(Some("ONE".into()), Operation::new(Instruction::Normal(NormalMneumonic::SetConstant), self.constant.into()))),
                AddressedLine::new(Address { position: 4, ..Default::default() }, Line::new(Some("TWO".into()), Operation::new(Instruction::Normal(NormalMneumonic::SetConstant), 2.into()))),
                AddressedLine::new(Address { position: 6, ..Default::default() }, Line::new(Some("RESULT".into()), Operation::new(Instruction::Positional(PositionalMneumonic::ReserveMemory), 2.into()))),
                AddressedLine::new(Address { position: 8, ..Default::default() }, Line::new(None, Operation::new(Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin), self.position))),
                AddressedLine::new(Address { position: main_position, ..Default::default() }, Line::new(Some("MAIN".into()), Operation::new(Instruction::Normal(NormalMneumonic::Load), self.load_label.into()))),
                AddressedLine::new(Address { position: main_position + 2, ..Default::default() }, Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::Add), "TWO".into()))),
                AddressedLine::new(Address { position: main_position + 4, ..Default::default() }, Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::Memory), "RESULT".into()))),
                AddressedLine::new(Address { position: main_position + 6, ..Default::default() }, Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::LoadValue), self.load_value.into()))),
                AddressedLine::new(Address { position: main_position + 8, ..Default::default() }, Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::Add), "RESULT".into()))),
                AddressedLine::new(Address { position: main_position + 10, ..Default::default() }, Line::new(None, Operation::new(Instruction::Normal(NormalMneumonic::Memory), "RESULT".into()))),
                AddressedLine::new(Address { position: main_position + 12, ..Default::default() }, Line::new(None, Operation::new(Instruction::Positional(PositionalMneumonic::SetEnd), "MAIN".into()))),
            ]);
            let label_map = LabelMap::from([
                ("IMPORT".into(), Address { position: 0, imported: true, ..Default::default() }),
                ("ONE".into(), Address { position: 2, ..Default::default() }),
                ("TWO".into(), Address { position: 4, ..Default::default() }),
                ("RESULT".into(), Address { position: 6, exported: true, ..Default::default() }),
                ("MAIN".into(), Address { position: 0x100, ..Default::default() }),
            ]);
            (program, label_map)
        }
    }

    #[test]
    // fn symbolic_operand_on_import_export_should_pass() {
    fn default_should_pass() {
        let test_program = TestProgram::default();
        assert!(test_program.validate().is_ok());
    }

    // #[test]
    // fn symbolic_operand_on_import_export_should_pass() {

    // }
    #[test]
    fn numeric_operand_on_import_export_should_fail() {
        let test_program = TestProgram { import: 0.into(), ..Default::default() };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn numeric_operand_on_positional_should_pass() {

    // }
    #[test]
    fn symbolic_operand_on_positional_should_fail() {
        let test_program = TestProgram { position: "FOO".into(), ..Default::default() };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn defined_label_should_pass() {

    // }
    #[test]
    fn undefined_label_should_fail() {
        let test_program = TestProgram { load_label: "FOO".into(), ..Default::default() };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn code_inside_address_space_should_pass() {

    // }
    #[test]
    fn code_outside_address_space_should_fail() {
        let test_program = TestProgram { position: 0xFFF.into(), ..Default::default() };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn numeric_operand_12_bit_should_pass() {

    // }
    #[test]
    fn numeric_operand_over_12_bit_should_fail() {
        let test_program = TestProgram { load_value: 0x1000, ..Default::default() };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn numeric_operand_16_bit_constant_should_pass() {

    // }
    #[test]
    fn numeric_operand_over_16_bit_constant_should_fail() {
        let test_program = TestProgram { constant: 0x1_0000, ..Default::default() };
        assert!(test_program.validate().is_err());
    }
}
