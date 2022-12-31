use nom::{character::complete::space1, combinator::map, sequence::separated_pair};
use types;

use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for types::Operation<'a> {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
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
            Operation::parse(Span::new("< FUNC")).unwrap().1,
            Operation::new(
                Instruction::Relational(RelationalMneumonic::Import),
                Operand::new_symbolic(Label::new("FUNC"))
            )
        );
        assert_eq!(
            Operation::parse(Span::new("JP  /0")).unwrap().1,
            Operation::new(
                Instruction::Normal(NormalMneumonic::Jump),
                Operand::new_numeric(0)
            )
        );
        assert_eq!(
            Operation::parse(Span::new("AD VAR")).unwrap().1,
            Operation::new(
                Instruction::Normal(NormalMneumonic::Add),
                Operand::new_symbolic(Label::new("VAR"))
            )
        );
        assert_eq!(
            Operation::parse(Span::new("K /0")).unwrap().1,
            Operation::new(
                Instruction::Normal(NormalMneumonic::SetConstant),
                Operand::new_numeric(0)
            )
        );
        assert_eq!(
            Operation::parse(Span::new("# MAIN")).unwrap().1,
            Operation::new(
                Instruction::Positional(PositionalMneumonic::SetEnd),
                Operand::new_symbolic(Label::new("MAIN"))
            )
        );
    }
}
