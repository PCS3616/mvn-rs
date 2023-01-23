use std::collections::BTreeMap;

use utils::types::Token;

use crate::types::{Label, Operation, AddressPosition, AddressedProgram, AddressedLine};
use crate::parser::Relocate;

type ImportMap<'a> = BTreeMap<AddressPosition, Label<'a>>;
type ExportMap<'a> = BTreeMap<Label<'a>, AddressPosition>;

struct ProgramProcessor<'a> {
    program: AddressedProgram<'a>,
    export_map: ExportMap<'a>,
}

impl<'a> ProgramProcessor<'a> {
    fn process(base: AddressPosition, program: AddressedProgram<'a>) -> Self {
        let program = program.relocate(base);
        let (imports, exports, instructions) = program.partition();
        let import_map = Self::create_import_map(imports);
        let export_map = Self::create_export_map(exports);
        let program = Self::replace_imported_operands_with_labels(instructions, import_map);
        ProgramProcessor { program, export_map }
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

    fn replace_imported_operands_with_labels(instructions: Vec<AddressedLine<'a>>, mut import_map: ImportMap<'a>) -> AddressedProgram<'a> {
        let mut lines:  Vec<AddressedLine> = Vec::new();
        for line in instructions.into_iter() {
            let line = if line.address.properties.operand_imported {
                let operand = line.operation.operand.value.try_into().unwrap();
                let operation = if let Some(label) = import_map.remove(&operand) {
                    let operand = Token::new(line.operation.operand.position, label.into());
                    Operation {
                        operand,
                        ..line.operation
                    }
                } else {
                    // TODO Add error treatment
                    panic!("imported symbol not found")
                };
                AddressedLine { operation, ..line }
            } else {
                line
            };
            lines.push(line);
        }
        AddressedProgram::new(lines)
    }
}

pub struct ProgramsProcessor<'a> {
    pub linked_program: AddressedProgram<'a>,
    pub export_map: ExportMap<'a>,
}

impl<'a> ProgramsProcessor<'a> {
    pub fn new(programs: Vec<AddressedProgram<'a>>) -> Self {
        Self::process(programs)
    }

    pub fn process(programs: Vec<AddressedProgram<'a>>) -> Self {
        let mut processed_programs: Vec<AddressedProgram> = Vec::new();
        let mut base: AddressPosition = 0;
        let mut export_map = ExportMap::new();
        for program in programs {
            let processor = ProgramProcessor::process(base, program);
            base = processor.program.get_last_position() + 0x2;
            processed_programs.push(processor.program);
            export_map.extend(processor.export_map)
        }
        let merged_program = Self::merge_programs(processed_programs);
        let linked_program = Self::replace_imported_operands_with_positions(merged_program, &export_map);
        ProgramsProcessor { linked_program, export_map }
    }

    fn merge_programs(programs: Vec<AddressedProgram<'a>>) -> AddressedProgram<'a> {
        let mut lines: Vec<AddressedLine> = Vec::new();
        for program in programs {
            lines.extend(program);
        }
        AddressedProgram::new(lines)
    }

    fn replace_imported_operands_with_positions(program: AddressedProgram<'a>, export_map: &ExportMap<'a>) -> AddressedProgram<'a> {
        let mut lines: Vec<AddressedLine> = Vec::new();
        for line in program {
            let line = if line.address.properties.operand_imported {
                let operand: Label = line.operation.operand.value.try_into().unwrap();
                let operation = if let Some(position) = export_map.get(&operand) {
                    let operand = Token::new(line.operation.operand.position, (*position).into());
                    Operation {
                        operand,
                        ..line.operation
                    }
                } else {
                    // TODO Add error treatment
                    panic!("imported symbol not found")
                };
                AddressedLine { operation, ..line }
            } else {
                line
            };
            lines.push(line);
        }
        AddressedProgram::new(lines)
    }
}
