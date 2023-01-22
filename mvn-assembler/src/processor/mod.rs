pub(crate) mod address;
mod validator;

use nom;
use types::Program;
use utils::error::MvnParseError;

use crate::parser::Parse;
use crate::processor::address::{AddressedProgram, LabelMap};

use validator::validate;

pub fn process(program: &str) -> Result<(AddressedProgram, LabelMap), nom::Err<MvnParseError>> {
    let (_, program) = Program::parse(program.into())?;
    let addressed_program = AddressedProgram::process(program);
    let label_map = addressed_program.map_labels();
    validate(&addressed_program, &label_map).map(|_| (addressed_program, label_map))
}
