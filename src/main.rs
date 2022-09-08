use std::io;
use std::io::Read;

use mvn::parser::lines::Lines;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (_, lines) = Lines::parse(&input).unwrap();

    print!("{:?}", lines);
    Ok(())
}
