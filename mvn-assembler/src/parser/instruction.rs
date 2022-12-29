use nom::{branch::alt, combinator::map, IResult};
use types;
use types::mneumonic;

use super::Parse;

impl Parse<'_> for types::Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(mneumonic::NormalMneumonic::parse, |o| Self::Normal(o)),
            map(mneumonic::PositionalMneumonic::parse, |o| {
                Self::Positional(o)
            }),
            map(mneumonic::RelationalMneumonic::parse, |o| {
                Self::Relational(o)
            }),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::mneumonic::*;
    use types::*;

    use super::*;

    #[test]
    fn should_parse_instructions() {
        assert_eq!(
            Instruction::parse("JP"),
            Ok(("", Instruction::Normal(NormalMneumonic::Jump)))
        );
        assert_eq!(
            Instruction::parse("@"),
            Ok((
                "",
                Instruction::Positional(PositionalMneumonic::SetAbsoluteOrigin)
            ))
        );
        assert_eq!(
            Instruction::parse(">"),
            Ok(("", Instruction::Relational(RelationalMneumonic::Export)))
        );
    }
}
