use nom::{character::complete::space1, combinator::map, sequence::separated_pair, IResult};
use types;

use super::Parse;

impl<'a> Parse<'a> for types::Operation<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            separated_pair(types::Instruction::parse, space1, types::Operand::parse),
            |(instruction, operand)| Self::new(instruction, operand),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::mneumonic::*;
    use types::*;

    use super::*;

    #[test]
    fn should_parse_operation() {
        assert_eq!(
            Operation::parse("< FUNC"),
            Ok((
                "",
                Operation::new(
                    Instruction::Relational(RelationalMneumonic::Import),
                    Operand::new_symbolic(Label("FUNC"))
                )
            ))
        );
        assert_eq!(
            Operation::parse("JP  /0"),
            Ok((
                "",
                Operation::new(
                    Instruction::Normal(NormalMneumonic::Jump),
                    Operand::new_numeric(0)
                )
            ))
        );
        assert_eq!(
            Operation::parse("AD VAR"),
            Ok((
                "",
                Operation::new(
                    Instruction::Normal(NormalMneumonic::Add),
                    Operand::new_symbolic(Label("VAR"))
                )
            ))
        );
        assert_eq!(
            Operation::parse("K /0"),
            Ok((
                "",
                Operation::new(
                    Instruction::Normal(NormalMneumonic::SetConstant),
                    Operand::new_numeric(0)
                )
            ))
        );
        assert_eq!(
            Operation::parse("# MAIN"),
            Ok((
                "",
                Operation::new(
                    Instruction::Positional(PositionalMneumonic::SetEnd),
                    Operand::new_symbolic(Label("MAIN"))
                )
            ))
        );
    }
}
