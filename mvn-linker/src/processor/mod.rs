use std::collections::BTreeMap;
use types::{Operand, Operation};
use types::{Label, Instruction, mneumonic::RelationalMneumonic};

use crate::parser::Position;
use crate::parser::{program::AddressedProgram, line::AddressedLine, Relocate};

type ImportMap<'a> = BTreeMap<Position, Label<'a>>;
type ExportMap<'a> = BTreeMap<Label<'a>, Position>;

pub fn process_programs(programs: Vec<AddressedProgram>) -> AddressedProgram {
    let mut exports: ExportMap = BTreeMap::new();
    let mut processed_programs: Vec<AddressedProgram> = Vec::new();
    let mut base = 0u16;
    for program in programs {
        let program = process_program(base, program,  &mut exports);
        base = find_latest_position(&program) + 0x2;
        processed_programs.push(program);
    }

    let program = merge_programs(processed_programs);
    let program = replace_imported_operands_with_positions(program, &exports);
    program
}

fn merge_programs(programs: Vec<AddressedProgram>) -> AddressedProgram {
    let mut lines: Vec<AddressedLine> = Vec::new();
    for program in programs {
        lines.extend(program);
    }
    AddressedProgram::new(lines)
}

fn replace_imported_operands_with_positions<'a>(program: AddressedProgram<'a>, exports: &ExportMap<'a>) -> AddressedProgram<'a> {
    AddressedProgram::new(
        program
        .into_iter()
        .map(|line| replace_imported_operand_line_with_position(line, exports))
        .collect()
    )
}

fn replace_imported_operand_line_with_position<'a>(line: AddressedLine<'a>, exports: &ExportMap<'a>) -> AddressedLine<'a> {
    if line.address.properties.operand_imported {
        let operand = replace_imported_operand_with_position(line.operation.operand, exports);
        let operation = Operation {operand, ..line.operation};
        AddressedLine { operation, ..line }
    } else {
        line
    }
}

fn replace_imported_operand_with_position<'a>(operand: Operand, exports: &ExportMap<'a>) -> Operand<'a> {
    let operand: Label = operand.try_into().unwrap();
    match exports.get(&operand) {
        Some(position) => Operand::new_numeric(*position),
        // TODO Add error treatment
        None => panic!("imported symbol not found"),
    }
}

fn find_latest_position(program: &AddressedProgram) -> u16 {
    let line = program.lines.iter().max_by_key(|line| line.address.position);
    match line {
        Some(line) => line.address.position,
        None => 0,
    }

}

fn process_program<'a>(base: Position, program: AddressedProgram<'a>, global_exports: &mut ExportMap<'a>) -> AddressedProgram<'a> {
    let program = resolve_program_physical_addresses(base, program);
    let (symbol_table, instructions): (Vec<AddressedLine>, Vec<AddressedLine>) = program.into_iter().partition(
        |line| line.relational_annotation.is_some()
    );
    let (imports, exports): (Vec<AddressedLine>, Vec<AddressedLine>) = symbol_table.into_iter().partition(
        |line| line_is_import(&line.relational_annotation)
    );
    let imports = create_import_map(AddressedProgram::new(imports));
    let instructions = replace_imported_operands_with_labels(
        AddressedProgram::new(instructions),
        &imports
    );

    update_export_map(AddressedProgram::new(exports), global_exports);
    instructions
}

fn replace_imported_operands_with_labels<'a>(program: AddressedProgram<'a>, imports: &ImportMap<'a>) -> AddressedProgram<'a> {
    AddressedProgram::new(
        program
        .into_iter()
        .map(|line| replace_imported_operand_line_with_label(line, imports))
        .collect()
    )
}

fn replace_imported_operand_line_with_label<'a>(line: AddressedLine<'a>, imports: &ImportMap<'a>) -> AddressedLine<'a> {
    if line.address.properties.operand_imported {
        let operand = replace_imported_operand_with_label(line.operation.operand, imports);
        let operation = Operation {operand, ..line.operation};
        AddressedLine { operation, ..line }
    } else {
        line
    }
}

fn replace_imported_operand_with_label<'a>(operand: Operand, imports: &ImportMap<'a>) -> Operand<'a> {
    let operand: u16 = operand.try_into().unwrap();
    match imports.get(&operand) {
        Some(label) => Operand::new_symbolic(label.clone()),
        // TODO Add error treatment
        None => panic!("imported symbol not found"),
    }
}

// TODO Find a way to reduce code duplication between
// create map functions

fn update_export_map<'a>(exports: AddressedProgram<'a>, global_exports: &mut ExportMap<'a>) -> () {
    for line in exports {
        let annotation = line.relational_annotation.unwrap();
        let label: Label = annotation.operation.operand.try_into().unwrap();
        let position: u16 = line.operation.operand.try_into().unwrap();
        global_exports.insert(label, position);
    }
}

fn create_import_map(imports: AddressedProgram) -> ImportMap {
    let mut import_map: Vec<(Position, Label)> = Vec::new();
    for line in imports {
        let annotation = line.relational_annotation.unwrap();
        let label: Label = annotation.operation.operand.try_into().unwrap();
        let position: u16 = line.operation.operand.try_into().unwrap();
        import_map.push((position, label));
    }
    import_map.into_iter().collect()
}

fn line_is_import(line: &Option<types::Line>) -> bool {
    if let Some(line) = line {
        if let Instruction::Relational(mneumonic) = &line.operation.instruction {
            return mneumonic == &RelationalMneumonic::Import
        }
    }
    false
}

fn resolve_program_physical_addresses(base: Position, program: AddressedProgram) -> AddressedProgram {
    AddressedProgram::new(
        program.into_iter()
        .map(|line| resolve_line_physical_addresses(base, line))
        .collect()
    )
}

fn resolve_line_physical_addresses(base: Position, line: AddressedLine) -> AddressedLine {
    let properties = line.address.properties;
    let address = if properties.line_relocatable {
        line.address.relocate(base)
    } else {
        line.address
    };

    let operation = if properties.operand_relocatable {
        line.operation.relocate(base)
    } else {
        line.operation
    };

    AddressedLine::new(address, operation, line.relational_annotation)
}
