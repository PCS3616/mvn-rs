use utils::error::MvnReportError;

use crate::processor::program::{ProgramsProcessor, RelocatableLabel};
use crate::types::AddressPosition;
use crate::types::{
    mneumonic::RelationalMneumonic, AddressedLine, Instruction, Label, MachineAddress,
    MachineAddressProperties, Operand, Operation,
};

pub fn print(processor_output: Result<ProgramsProcessor, MvnReportError>, complete_linkage: bool) {
    match processor_output {
        Ok(processor) => print_program(processor, complete_linkage),
        Err(error) => print_error(error),
    }
}

fn print_error(error: MvnReportError) {
    eprintln!("{error:#?}");
}

fn print_program(processor: ProgramsProcessor, complete_linkage: bool) {
    for line in processor.linked_program {
        println!("{line}");
    }

    if complete_linkage {
        return;
    }

    for (export_label, export_position) in processor.export_map.into_iter() {
        let RelocatableLabel {
            relocatable,
            label: export_label,
        } = export_label;
        let line = relational_label_position_to_line(
            export_label,
            relocatable,
            export_position,
            RelationalMneumonic::Export,
        );
        println!("{line}");
    }

    for (import_label, import_position) in processor.inverted_import_map.into_iter() {
        let RelocatableLabel {
            relocatable: _,
            label: import_label,
        } = import_label;
        let line = relational_label_position_to_line(
            import_label,
            false,
            import_position,
            RelationalMneumonic::Import,
        );
        println!("{line}");
    }
}

fn relational_label_position_to_line(
    label: Label,
    relocatable: bool,
    position: AddressPosition,
    mneumonic: RelationalMneumonic,
) -> AddressedLine {
    let imported = mneumonic == RelationalMneumonic::Import;
    AddressedLine::new(
        MachineAddress::new(
            MachineAddressProperties::new(false, relocatable, imported),
            0,
        )
        .into(),
        Operation::new(
            Instruction::Relational(mneumonic).into(),
            Operand::from(position).into(),
        ),
        Some(assembly::types::Line::new(
            None,
            Operation::new(
                Instruction::Relational(mneumonic).into(),
                Operand::from(label).into(),
            ),
        )),
    )
}
