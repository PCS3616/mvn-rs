use std::collections::{BTreeMap, BTreeSet};

use utils::error::MvnReportError;
use utils::types::Token;

use crate::types::{Label, Operation, AddressPosition, AddressedProgram, AddressedLine, Operand, MachineAddress, MachineAddressProperties};
use crate::parser::Relocate;

#[derive(Debug, Eq)]
pub struct RelocatableLabel<'a> {
    pub relocatable: bool,
    pub label: Label<'a>,
}

impl<'a> Ord for RelocatableLabel<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label.cmp(&other.label)
    }
}

impl<'a> PartialOrd for RelocatableLabel<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a> PartialEq for RelocatableLabel<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

impl <'a> From<Label<'a>> for RelocatableLabel<'a> {
    fn from(value: Label<'a>) -> Self {
        RelocatableLabel { relocatable: false, label: value }
    }
}

impl<'a> RelocatableLabel<'a> {
    pub fn new(relocatable: bool, label: Label<'a>) -> Self {
        RelocatableLabel { relocatable, label }
    }

    pub fn label(&'a self) -> &'a Label<'a> {
        &self.label
    }

    pub fn relocatable(&self) -> bool {
        self.relocatable
    }
}

type ImportMap<'a> = BTreeMap<AddressPosition, RelocatableLabel<'a>>;
type ExportMap<'a> = BTreeMap<RelocatableLabel<'a>, AddressPosition>;

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
        let mut imports = BTreeSet::<RelocatableLabel>::new();
        for program in programs {
            let processor = ProgramProcessor::process(base, program)?;
            base = processor.program.get_last_position() + 0x2;
            processed_programs.push(processor.program);
            Self::extend_export_unique(&mut export_map, processor.export_map);
            imports.extend(processor.import_map.into_values());
        }
        let inverted_import_map: BTreeMap<_, _> = imports
            .into_iter()
            .filter(|label| !export_map.contains_key(label))
            .enumerate()
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

    // TODO Refactor, possibly splitting into 2-3 functions
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
                // TODO Add relocatable field to Label to remove this clone
                // and the RelocatableLabel struct
                let operand: RelocatableLabel = operand.clone().into();
                let (position, operand_relocatable, operand_imported)= if let Some((relocatable_label, position)) = export_map.get_key_value(&operand) {
                    (position, relocatable_label.relocatable, false)
                } else if let Some((relocatable_label, position)) = inverted_import_map.get_key_value(&operand) {
                    (position, relocatable_label.relocatable, true)
                } else {
                    return Err(MvnReportError::new(
                        line.operation.operand.position,
                        Some("operand marked as imported but not imported".to_owned()),
                    ));
                };
                let properties = MachineAddressProperties { operand_relocatable, operand_imported, ..line.address.value.properties };
                let address = MachineAddress { properties, ..line.address.value};
                let address = Token::new(line.address.position, address);
                let operand: Operand = (*position).into();
                let operand = Token::new(line.operation.operand.position, operand);
                let operation = Operation { operand, ..line.operation };
                AddressedLine { operation, address, ..line }
            } else {
                line
            };
            lines.push(line);
        }
        Ok(AddressedProgram::new(lines))
    }

    fn extend_export_unique(original_map: &mut ExportMap<'a>, new_map: ExportMap<'a>) {
        for (key, value) in new_map.into_iter() {
            if let Some(_) = original_map.insert(key, value) {
                panic!("export map already contained label");
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
            let (label, position, _) = line.destruct();
            import_map.insert(position, label.into());
        }
        import_map
    }

    fn create_export_map(exports: Vec<AddressedLine>) -> ExportMap {
        let mut export_map = ExportMap::new();
        for line in exports.into_iter() {
            // TODO Review API to replace `line.destruct()`
            let (label, position, relocatable) = line.destruct();
            export_map.insert(RelocatableLabel::new(relocatable, label), position);
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
                let operand = if let Some(relocatable_label) = import_map.get(&operand) {
                    Token::new(line.operation.operand.position, relocatable_label.label.clone().into())
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
