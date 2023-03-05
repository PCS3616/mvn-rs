use clap::Parser;
use utils::Executor;

use mvn_linker::Args;

fn main() {
    let args = Args::parse();
    args.execute();
}
