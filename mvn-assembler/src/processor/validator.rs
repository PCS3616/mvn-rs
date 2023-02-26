use crate::types::{mneumonic, Instruction, Line, Operand};

use crate::processor::address::{Address, AddressedProgram, LabelMap};

use super::MvnReportError;

type ValidatorResult<'a> = Result<(), MvnReportError>;

pub fn validate<'a, 'b>(
    program: &'a AddressedProgram<'b>,
    label_map: &'a LabelMap<'b>,
) -> ValidatorResult<'b> {
    for line in program.lines.iter() {
        let validator = LineValidator::new(&line.line, &line.address, label_map);
        validator.validate()?;
    }
    Ok(())
}

struct LineValidator<'a, 'b> {
    line: &'a Line<'b>,
    address: &'a Address,
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
        match &self.line.operation.instruction.value {
            Instruction::Relational(_) => match &self.line.operation.operand.value {
                Operand::Numeric(_) => Err(MvnReportError::new(
                    self.line.operation.operand.position,
                    Some("numeric operand cannot be imported nor exported".to_string()),
                )),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }

    fn symbolic_operand_on_positional(&self) -> ValidatorResult<'b> {
        match &self.line.operation.instruction.value {
            Instruction::Positional(mneumonic) => match mneumonic {
                mneumonic::PositionalMneumonic::SetEnd => Ok(()),
                _ => match &self.line.operation.operand.value {
                    Operand::Symbolic(_) => Err(MvnReportError::new(
                        self.line.operation.operand.position,
                        Some(
                            "symbolic operand cannot be used to reserve addresses or set positions"
                                .to_string(),
                        ),
                    )),
                    _ => Ok(()),
                },
            },
            _ => Ok(()),
        }
    }

    fn undefined_label(&self) -> ValidatorResult<'b> {
        match &self.line.operation.operand.value {
            Operand::Symbolic(label) => match &self.label_map.get(label) {
                None => Err(MvnReportError::new(
                    self.line.operation.operand.position,
                    Some("undefined label used as operand".to_string()),
                )),
                Some(_) => Ok(()),
            },
            _ => Ok(()),
        }
    }

    fn code_exceeding_address_space(&self) -> ValidatorResult<'b> {
        if self.address.position > 0xFFF {
            Err(MvnReportError::new(
                self.line.position(),
                Some("address outside memory".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    // fn implicit_memory_overwrite(&self) -> ValidatorResult {} // TODO Implement

    fn numeric_operand_too_wide(&self) -> ValidatorResult<'b> {
        let immediate = if let Operand::Numeric(immediate) = &self.line.operation.operand.value {
            *immediate
        } else {
            return Ok(());
        };

        match &self.line.operation.instruction.value {
            Instruction::Normal(mneumonic) => match mneumonic {
                mneumonic::NormalMneumonic::SetConstant => {
                    if immediate > 0xFFFF {
                        Err(MvnReportError::new(
                            self.line.operation.operand.position,
                            Some(
                                "immediate over 16 bits for constant pseudoinstruction".to_string(),
                            ),
                        ))
                    } else {
                        Ok(())
                    }
                }
                _ => {
                    if immediate > 0xFFF {
                        Err(MvnReportError::new(
                            self.line.operation.operand.position,
                            Some("immediate cannot be larger than 12 bits".to_string()),
                        ))
                    } else {
                        Ok(())
                    }
                }
            },
            _ => Ok(()),
        }
    }
}

impl<'a, 'b> LineValidator<'a, 'b> {
    fn new(line: &'a Line<'b>, address: &'a Address, label_map: &'a LabelMap<'b>) -> Self {
        Self {
            line,
            address,
            label_map,
        }
    }
}

// TODO Implement unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::address::*;
    use crate::types::{mneumonic::*, *};
    use utils::types::*;

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
                AddressedLine::new(
                    Address {
                        position: 0,
                        imported: true,
                        ..Default::default()
                    },
                    Line::new(
                        None,
                        Operation::new(
                            Token::new(
                                Position::new(1, 1),
                                Instruction::Relational(RelationalMneumonic::Import),
                            ),
                            Token::new(Position::new(1, 3), self.import),
                        ),
                    ),
                ),
                AddressedLine::new(
                    Address {
                        position: 2,
                        ..Default::default()
                    },
                    Line::new(
                        Some(Token::new(Position::new(4, 1), "ONE".into())),
                        Operation::new(
                            Token::new(
                                Position::new(4, 9),
                                Instruction::Normal(NormalMneumonic::SetConstant),
                            ),
                            Token::new(Position::new(4, 13), self.constant.into()),
                        ),
                    ),
                ),
                AddressedLine::new(
                    Address {
                        position: 8,
                        ..Default::default()
                    },
                    Line::new(
                        None,
                        Operation::new(
                            Token::new(
                                Position::new(7, 1),
                                Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin),
                            ),
                            Token::new(Position::new(7, 3), self.position),
                        ),
                    ),
                ),
                AddressedLine::new(
                    Address {
                        position: main_position,
                        ..Default::default()
                    },
                    Line::new(
                        Some(Token::new(Position::new(8, 1), "MAIN".into())),
                        Operation::new(
                            Token::new(
                                Position::new(8, 9),
                                Instruction::Normal(NormalMneumonic::Load),
                            ),
                            Token::new(Position::new(8, 13), self.load_label.into()),
                        ),
                    ),
                ),
                AddressedLine::new(
                    Address {
                        position: main_position + 6,
                        ..Default::default()
                    },
                    Line::new(
                        None,
                        Operation::new(
                            Token::new(
                                Position::new(11, 9),
                                Instruction::Normal(NormalMneumonic::LoadValue),
                            ),
                            Token::new(Position::new(11, 13), self.load_value.into()),
                        ),
                    ),
                ),
            ]);
            let label_map = LabelMap::from([
                (
                    "IMPORT".into(),
                    Address {
                        position: 0,
                        imported: true,
                        ..Default::default()
                    },
                ),
                (
                    "ONE".into(),
                    Address {
                        position: 2,
                        ..Default::default()
                    },
                ),
                (
                    "MAIN".into(),
                    Address {
                        position: 0x100,
                        ..Default::default()
                    },
                ),
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
        let test_program = TestProgram {
            import: 0.into(),
            ..Default::default()
        };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn numeric_operand_on_positional_should_pass() {

    // }
    #[test]
    fn symbolic_operand_on_positional_should_fail() {
        let test_program = TestProgram {
            position: "FOO".into(),
            ..Default::default()
        };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn defined_label_should_pass() {

    // }
    #[test]
    fn undefined_label_should_fail() {
        let test_program = TestProgram {
            load_label: "FOO".into(),
            ..Default::default()
        };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn code_inside_address_space_should_pass() {

    // }
    #[test]
    fn code_outside_address_space_should_fail() {
        let test_program = TestProgram {
            position: 0xFFF.into(),
            ..Default::default()
        };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn numeric_operand_12_bit_should_pass() {

    // }
    #[test]
    fn numeric_operand_over_12_bit_should_fail() {
        let test_program = TestProgram {
            load_value: 0x1000,
            ..Default::default()
        };
        assert!(test_program.validate().is_err());
    }

    // #[test]
    // fn numeric_operand_16_bit_constant_should_pass() {

    // }
    #[test]
    fn numeric_operand_over_16_bit_constant_should_fail() {
        let test_program = TestProgram {
            constant: 0x1_0000,
            ..Default::default()
        };
        assert!(test_program.validate().is_err());
    }
}
