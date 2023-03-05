use machine_code::parser::Relocate;
use utils::error::MvnReportError;

use crate::parser::Parse;
use crate::types::{AddressedLine, AddressedProgram};

pub fn process(program: &str, relocation_base: u16) -> Result<AddressedProgram, MvnReportError> {
    let instructions = parse(program)?;
    Ok(instructions
        .into_iter()
        .map(|line| {
            let mut line = line.relocate(relocation_base as u32);
            line.address.value.properties = Default::default();
            line
        })
        .collect())
}

fn parse(program: &str) -> Result<Vec<AddressedLine>, MvnReportError> {
    let parse_result = AddressedProgram::parse_machine_code(program.into());
    let (_, program) = parse_result.map_err(|e| match e {
        nom::Err::Error(e) | nom::Err::Failure(e) => MvnReportError::from(e),
        nom::Err::Incomplete(e) => panic!("Unhandled error `{e:?}` occured"),
    })?;
    let (symbol_table, instructions): (Vec<AddressedLine>, Vec<AddressedLine>) = program
        .lines
        .into_iter()
        .partition(|line| line.relational_annotation.is_some());
    if symbol_table.first().is_some() {
        panic!("symbol table present")
    }
    Ok(instructions)
}
