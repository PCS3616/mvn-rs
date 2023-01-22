pub(crate) mod address;
mod validator;

use nom;
use types::Program;
use utils::error::MvnParseError;

use std::convert::From;

use types::Position;

#[derive(Debug)]
pub struct MvnReportError {
    pub position: Position,
    pub message: Option<String>,
}

impl From<MvnParseError<'_>> for MvnReportError {
    fn from(value: MvnParseError) -> Self {
        Self {
            position: value.span.into(),
            message: value.message,
        }
    }
}

impl MvnReportError {
    fn new(position: Position, message: Option<String>) -> Self {
        Self { position, message }
    }
}

use crate::parser::Parse;
use crate::processor::address::{AddressedProgram, LabelMap};

use validator::validate;

pub fn process(program: &str) -> Result<(AddressedProgram, LabelMap), MvnReportError> {
    let parse_result = Program::parse(program.into());
    let (_, program) = parse_result.map_err(|e| match e {
        nom::Err::Error(e) | nom::Err::Failure(e) => MvnReportError::from(e),
        nom::Err::Incomplete(e) => panic!("Unhandled error `{e:?}` occured"),
    })?;
    let addressed_program = AddressedProgram::process(program);
    let label_map = addressed_program.map_labels();
    validate(&addressed_program, &label_map).map(|_| (addressed_program, label_map))
}
