use std::collections::{BTreeMap, BTreeSet};

use utils::error::MvnReportError;
use utils::types::Token;

use crate::types::{Label, Operation, AddressPosition, AddressedProgram, AddressedLine, Operand};
use crate::parser::Relocate;

type ImportMap<'a> = BTreeMap<AddressPosition, Label<'a>>;
// TODO Store tuple (label, relocatable) in export map
// to correctly write exports back
type ExportMap<'a> = BTreeMap<Label<'a>, AddressPosition>;

#[derive(Debug)]
pub struct ProgramsProcessor<'a> {
    pub linked_program: AddressedProgram<'a>,
    pub export_map: ExportMap<'a>,
    pub inverted_import_map: ExportMap<'a>,
}

impl<'a> ProgramsProcessor<'a> {
    pub fn process(programs: Vec<AddressedProgram<'a>>) -> Result<Self, MvnReportError> {
        let mut processed_programs: Vec<AddressedProgram> = Vec::new();
        let mut base: AddressPosition = 0;
        let mut export_map = ExportMap::new();
        let mut imports = BTreeSet::<Label>::new();
        for program in programs {
            let processor = ProgramProcessor::process(base, program)?;
            base = processor.program.get_last_position() + 0x2;
            processed_programs.push(processor.program);
            Self::extend_export_unique(&mut export_map, processor.export_map);
            imports.extend(processor.import_map.into_values());
        }
        let inverted_import_map: BTreeMap<_, _> = imports
            .into_iter()
            .enumerate()
            .filter(|(_, label)| !export_map.contains_key(label))
            .map(|(i, label)| (label, u32::try_from(i).unwrap()))
            .collect();
        let merged_program = Self::merge_programs(processed_programs);
        let linked_program = match Self::replace_imported_operands_with_positions(merged_program, &export_map, &inverted_import_map) {
            Ok(program) => program,
            Err(e) => return Err(e),
        };
        Ok(ProgramsProcessor { linked_program, export_map, inverted_import_map })
    }

    fn merge_programs(programs: Vec<AddressedProgram<'a>>) -> AddressedProgram<'a> {
        let mut lines: Vec<AddressedLine> = Vec::new();
        for program in programs {
            lines.extend(program);
        }
        AddressedProgram::new(lines)
    }

    fn replace_imported_operands_with_positions(program: AddressedProgram<'a>, export_map: &ExportMap<'a>, inverted_import_map: &ExportMap<'a>) -> Result<AddressedProgram<'a>, MvnReportError> {
        let mut lines: Vec<AddressedLine> = Vec::new();
        for line in program {
            let line = if line.address.value.properties.operand_imported {
                let operand = match line.operation.operand.value {
                    Operand::Numeric(_) => return Err(MvnReportError::new(
                        line.operation.operand.position,
                        Some("can't replace numeric operand with position".to_owned()),
                    )),
                    Operand::Symbolic(label) => label,
                };
                let position = if let Some(position) = export_map.get(&operand) {
                    position
                } else if let Some(position) = inverted_import_map.get(&operand) {
                    position
                } else {
                    return Err(MvnReportError::new(
                        line.operation.operand.position,
                        Some("operand marked as imported but not imported".to_owned()),
                    ));
                };
                let operand = Token::new(line.operation.operand.position, (*position).into());
                let operation = Operation { operand, ..line.operation };
                AddressedLine { operation, ..line }
            } else {
                line
            };
            lines.push(line);
        }
        Ok(AddressedProgram::new(lines))
    }

    fn extend_export_unique(original_map: &mut ExportMap<'a>, new_map: ExportMap<'a>) {
        for (key, value) in new_map.into_iter() {
            match original_map.insert(key, value) {
                // TODO Consider how to represent this error
                // since there is no `Position` so `MvnReportError` wouldn't do
                Some(_) => panic!("export map already contained label"),
                None => continue,
            }
        }
    }
}

struct ProgramProcessor<'a> {
    program: AddressedProgram<'a>,
    export_map: ExportMap<'a>,
    import_map: ImportMap<'a>,
}

impl<'a> ProgramProcessor<'a> {
    fn process(base: AddressPosition, program: AddressedProgram<'a>) -> Result<Self, MvnReportError> {
        let program = program.relocate(base);
        let (imports, exports, instructions) = program.partition();
        let import_map = Self::create_import_map(imports);
        let export_map = Self::create_export_map(exports);
        let program = Self::replace_imported_operands_with_labels(instructions, &import_map)?;
        Ok(ProgramProcessor { program, export_map, import_map })
    }

    fn create_import_map(imports: Vec<AddressedLine>) -> ImportMap {
        let mut import_map = ImportMap::new();
        for line in imports.into_iter() {
            // TODO Review API to replace `line.destruct()`
            let (label, position) = line.destruct();
            import_map.insert(position, label);
        }
        import_map
    }

    fn create_export_map(exports: Vec<AddressedLine>) -> ExportMap {
        let mut export_map = ExportMap::new();
        for line in exports.into_iter() {
            // TODO Review API to replace `line.destruct()`
            let (label, position) = line.destruct();
            export_map.insert(label, position);
        }
        export_map
    }

    fn replace_imported_operands_with_labels(instructions: Vec<AddressedLine<'a>>, import_map: &ImportMap<'a>) -> Result<AddressedProgram<'a>, MvnReportError> {
        let mut lines:  Vec<AddressedLine> = Vec::new();
        for line in instructions.into_iter() {
            let line = if line.address.value.properties.operand_imported {
                let operand = match line.operation.operand.value {
                    Operand::Numeric(immediate) => immediate,
                    Operand::Symbolic(_) => return Err(MvnReportError::new(
                        line.operation.operand.position,
                        Some("can't replace numeric operand with position".to_owned()),
                    ))
                };
                let operand = if let Some(label) = import_map.get(&operand) {
                    Token::new(line.operation.operand.position, label.clone().into())
                } else {
                    return Err(MvnReportError::new(
                        line.operation.operand.position,
                        Some("operand marked as imported but not imported".to_owned()),
                    ));
                };
                let operation = Operation { operand, ..line.operation };
                AddressedLine { operation, ..line }
            } else {
                line
            };
            lines.push(line);
        }
        Ok(AddressedProgram::new(lines))
    }
}
