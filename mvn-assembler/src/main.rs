use std::io;

use mvn_assembler::processor::process;
use mvn_assembler::writer::print;

fn main() {
    let program = io::stdin()
        .lines()
        .map(|result| result.expect("unable to read from stdin"))
        .reduce(|mut acc, result| {
            acc.push('\n');
            acc.push_str(&result);
            acc
        });
    if let Some(program) = program {
        let validator_output = process(&program);
        print(&program, validator_output);
    }
}
