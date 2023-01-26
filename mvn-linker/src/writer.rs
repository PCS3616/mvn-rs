use utils::error::MvnReportError;

use crate::processor::program::ProgramsProcessor;
use crate::types::AddressPosition;
use crate::types::{AddressedLine, MachineAddress, MachineAddressProperties, Operation, Instruction, Operand, mneumonic::RelationalMneumonic, Label};

pub fn print(processor_output: Result<ProgramsProcessor, MvnReportError>) {
    match processor_output {
        Ok(processor) => print_program(processor),
        Err(error) => print_error(error),
    }
}

fn print_error(error: MvnReportError) {
    println!("{error:#?}");
}

fn print_program(processor: ProgramsProcessor) {
    for line in processor.linked_program {
        let line = AddressedLine { address: MachineAddress::new(MachineAddressProperties::new(false, false, false), line.address.value.position).into(), ..line };
        println!("{line}");
    }

    for (export_label, export_position) in processor.export_map.into_iter() {
        let line = relational_label_position_to_line(export_label, export_position, RelationalMneumonic::Export);
        println!("{line}");
    }

    for (import_label, import_position) in processor.inverted_import_map.into_iter() {
        let line = relational_label_position_to_line(import_label, import_position, RelationalMneumonic::Import);
        println!("{line}");
    }
}

fn relational_label_position_to_line(label: Label, position: AddressPosition, mneumonic: RelationalMneumonic) -> AddressedLine {
    let imported = mneumonic == RelationalMneumonic::Import;
    AddressedLine::new(
        MachineAddress::new(MachineAddressProperties::new(false, false, imported), 0).into(),
        Operation::new(Instruction::Relational(mneumonic).into(), Operand::from(position).into()),
        Some(assembler::types::Line::new(
            None, Operation::new(Instruction::Relational(mneumonic).into(), Operand::from(label).into())
        )),
    )
}
