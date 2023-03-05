use std::path::PathBuf;

use clap::{Parser, Subcommand};
use utils::io::{file_exists, read_to_string};
use utils::Executor;

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
        input: PathBuf,
    },
    Link(linker::Args),
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
        }
        Commands::Link(args) => args.execute(),
        Commands::Relocate { input, base } => {
            let program = read_to_string(input);
            let process_result = relocator::processor::process(&program, *base);
            relocator::writer::print(process_result);
        }
    }
}

