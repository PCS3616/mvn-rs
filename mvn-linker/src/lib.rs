pub mod processor;
pub mod writer;

pub use machine_code::*;

use std::path::PathBuf;

use clap::{ArgAction, ArgGroup, Parser};
use utils::io::{file_exists, read_to_string};
use utils::Executor;

use crate::{processor::process, writer::print};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("linkage-type")
    .required(true)
    .args(["partial", "complete"])
))]
pub struct Args {
    #[arg(
        short,
        long = "input",
        required = true,
        action = ArgAction::Append,
        value_parser = file_exists
    )]
    pub inputs: Vec<PathBuf>,
    #[arg(long)]
    pub partial: bool,
    #[arg(long)]
    pub complete: bool,
}

impl Executor for Args {
    fn execute(&self) {
        let programs: Vec<String> = self.inputs.iter().map(read_to_string).collect();
        let programs: Vec<&str> = programs.iter().map(String::as_str).collect();
        let process_result = process(programs, self.complete);
        print(process_result, self.complete);
    }
}
