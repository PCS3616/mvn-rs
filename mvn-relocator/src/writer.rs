use utils::error::MvnReportError;

use crate::types::AddressedProgram;

pub fn print(processor_output: Result<AddressedProgram, MvnReportError>) {
    match processor_output {
        Ok(program) => print_program(program),
        Err(error) => print_error(error),
    }
}

fn print_error(error: MvnReportError) {
    eprintln!("{error:#?}");
}

fn print_program(program: AddressedProgram) {
    for line in program.into_iter() {
        println!("{line}");
    }
}
