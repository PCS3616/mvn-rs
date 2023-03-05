pub(crate) mod address;
mod validator;

use nom;

use utils::error::MvnReportError;

use crate::parser::Parse;
use crate::processor::address::{AddressedProgram, LabelMap};
use crate::types::Program;

use validator::validate;

pub fn process(program: &str) -> Result<(AddressedProgram, LabelMap), MvnReportError> {
    let parse_result = Program::parse_assembler(program.into());
    let (_, program) = parse_result.map_err(|e| match e {
        nom::Err::Error(e) | nom::Err::Failure(e) => MvnReportError::from(e),
        nom::Err::Incomplete(e) => panic!("Unhandled error `{e:?}` occured"),
    })?;
    let addressed_program = AddressedProgram::process(program);
    let label_map = addressed_program.map_labels();
    validate(&addressed_program, &label_map).map(|_| (addressed_program, label_map))
}
