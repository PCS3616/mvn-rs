pub mod program;

use utils::error::MvnReportError;

use crate::parser::Parse;
use crate::types::AddressedProgram;

use program::ProgramsProcessor;

pub fn process(programs: Vec<&str>) -> Result<ProgramsProcessor, MvnReportError> {
    let mut parsed_programs: Vec<AddressedProgram> = Vec::new();
    for program in programs {
        let parse_result = AddressedProgram::parse_machine_code(program.into());
        let (_, program) = parse_result.map_err(|e| match e {
            nom::Err::Error(e) | nom::Err::Failure(e) => MvnReportError::from(e),
            nom::Err::Incomplete(e) => panic!("Unhandled error `{e:?}` occured"),
        })?;
        parsed_programs.push(program);
    }
    let processor = ProgramsProcessor::process(parsed_programs)?;
    Ok(processor)
}
