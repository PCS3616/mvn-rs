use std::io;
use std::io::Read;

use mvn::parser::lines::Lines;

fn generate_bin_str(input: &str) -> String {
    let (_, lines) = Lines::parse(input).unwrap();
    lines.value().unwrap().into_iter().map(|l| format!("{:04X}", l)).collect::<Vec<_>>().join("\n")
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let binnary = generate_bin_str(&input);
    print!("{}", binnary);
    Ok(())
}
