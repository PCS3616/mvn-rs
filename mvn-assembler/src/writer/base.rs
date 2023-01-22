// use std::fmt;
use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use nom;

use crate::processor::address::{Address, AddressedLine, AddressedProgram, LabelMap};
use types::{Instruction, Line, Operand};
use utils::error::MvnParseError;

// pub fn write(
//     validator_output: Result<(AddressedProgram, LabelMap), MvnParseError>,
//     // output: impl std::io::Write,
//     // error: impl std::io::Write
// ) {}
pub fn print(
    program: &str,
    validator_output: Result<(AddressedProgram, LabelMap), nom::Err<MvnParseError>>,
) {
    match validator_output {
        Ok((program, label_map)) => print_program(program, label_map),
        Err(e) => match e {
            nom::Err::Error(error) | nom::Err::Failure(error) => print_error(program, error),
            nom::Err::Incomplete(e) => panic!("Unhandled error `{e:?}` occured"),
        },
    }
}

fn print_error(program: &str, error: MvnParseError) {
    let line: usize = (error.line() - 1).try_into().unwrap();
    let source = program
        .lines()
        .nth(line)
        .unwrap();
    let column = error.span().get_column();
    let span_length = error.span().len();

    let snippet = Snippet {
        title: Some(Annotation {
            label: Some("error while parsing input file"),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![Slice {
            source: source,
            line_start: line,
            origin: Some("examples/footer.rs"),
            fold: false,
            annotations: vec![
                SourceAnnotation {
                    label: "invalid mneumonic",
                    annotation_type: AnnotationType::Error,
                    range: (column, column + 2),
                },
            ],
        }],
        opt: FormatOptions {
            color: true,
            ..Default::default()
        },
    };
    let dl = DisplayList::from(snippet);
    println!("{}", dl);
    eprintln!("{error:?}");
}

// fn write_program(program: AddressedProgram, label_map: LabelMap, output: impl fmt::Write) {
fn print_program(program: AddressedProgram, label_map: LabelMap) {
    let default_address = Address::default();

    for AddressedLine { address, line } in program.lines.iter() {
        let Line {
            label: _,
            operation,
        } = line;

        let instruction_value: u8 = match operation.instruction {
            Instruction::Positional(_) => continue,
            Instruction::Normal(mneumonic) => mneumonic.into(),
            _ => 0,
        };

        let (operand_address, operand_value) = match &operation.operand {
            Operand::Symbolic(label) => {
                let operand_address = label_map.get(&label).unwrap();
                (operand_address, operand_address.position)
            },
            Operand::Numeric(immediate) => (&default_address, *immediate),
        };

        let operation_value = ((instruction_value as u32) << 12) + operand_value;

        let nibble_value = resolve_nibble(address, operand_address);

        let operation_address = ((nibble_value as u32) << 12) + address.position;

        print!("{:04X} {:04X}", operation_address, operation_value);
        if let Instruction::Relational(relational_mneumonic) = &operation.instruction {
            if let Operand::Symbolic(relational_label) = &operation.operand {
                print!(" ; {} {}", relational_mneumonic, relational_label.0);
            }
        }
        println!("");
    }
}

fn resolve_nibble(line: &Address, operand: &Address) -> u8 {
    ((0 as u8) << 3) // One bit is not necessary, so it's fixed at zero
    + ((line.relocatable as u8) << 2)
    + ((operand.relocatable as u8) << 1)
    + (operand.imported as u8)
}
