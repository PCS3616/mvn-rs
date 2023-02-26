use std::path::{Path, PathBuf};
use std::fs;

use clap::{Parser, Subcommand, ArgAction, ArgGroup};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Assemble {
        #[arg(short, long, value_parser = file_exists)]
        input: PathBuf
    },
    #[command(group(
        ArgGroup::new("linkage-type")
        .required(true)
        .args(["partial", "complete"])
    ))]
    Link {
        #[arg(
            short,
            long = "input",
            action = ArgAction::Append,
            value_parser = file_exists
        )]
        inputs: Vec<PathBuf>,
        #[arg(long)]
        partial: bool,
        #[arg(long)]
        complete: bool,
    },
    Relocate {
        #[arg(short, long, value_parser = file_exists)]
        input: PathBuf,
        #[arg(short, long, value_parser = clap_num::maybe_hex::<u16>)]
        base: u16,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Assemble { input } => {
            let program = read_to_string(input);
            let process_result = assembler::processor::process(&program);
            assembler::writer::print(&program, process_result);
        },
        Commands::Link { inputs, partial: _, complete} => {
            let programs: Vec<String> = inputs
                .iter()
                .map(|path| read_to_string(path))
                .collect();
            let programs: Vec<&str> = programs.iter().map(String::as_str).collect();
            let process_result = linker::processor::process(programs, *complete);
            linker::writer::print(process_result, *complete);
        },
        Commands::Relocate { input, base } => {
            let program = read_to_string(input);
            let process_result = relocator::processor::process(&program, *base);
            relocator::writer::print(process_result);
        }
    }
}

fn file_exists(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if let Ok(exists) = path.try_exists() {
        if exists {
            return Ok(path.to_path_buf())
        }
    }
    Err("input file does not exist")
}

fn read_to_string(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("failed to read file").to_uppercase()
}
