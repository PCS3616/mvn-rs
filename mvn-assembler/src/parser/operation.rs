use nom::{character::complete::space1, combinator::map, sequence::separated_pair};
use types::{Instruction, Operand, Token};

use super::error::{LocatedIResult, Span};
use super::Parse;

impl<'a> Parse<'a> for types::Operation<'a> {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        map(
            separated_pair(Token::<Instruction>::parse_assembler, space1, Token::<Operand>::parse_assembler),
            |(instruction, operand)| Self::new(instruction, operand),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::Position;
    use types::mneumonic::*;
    use types::*;

    use super::*;

    #[test]
    fn should_parse_operation_with_symbolic_operand() {
        assert_eq!(
            Operation::parse_assembler(Span::new("< FUNC")).unwrap().1,
            Operation::new(
                Token::new(Position::new(1, 1), Instruction::Relational(RelationalMneumonic::Import)),
                Token::new(Position::new(1, 3), Operand::from(Label::from("FUNC"))),
            )
        );
        assert_eq!(
            Operation::parse_assembler(Span::new("AD VAR")).unwrap().1,
            Operation::new(
                Token::new(Position::new(1, 1), Instruction::Normal(NormalMneumonic::Add)),
                Token::new(Position::new(1, 4), Operand::new_symbolic(Label::new("VAR"))),
            )
        );
        assert_eq!(
            Operation::parse_assembler(Span::new("# MAIN")).unwrap().1,
            Operation::new(
                Token::new(Position::new(1, 1), Instruction::Positional(PositionalMneumonic::SetEnd)),
                Token::new(Position::new(1, 3), Operand::new_symbolic(Label::new("MAIN"))),
            )
        );
    }

    #[test]
    fn should_parse_operation_with_numeric_operand() {
        assert_eq!(
            Operation::parse_assembler(Span::new("JP  /0")).unwrap().1,
            Operation::new(
                Token::new(Position::new(1, 1), Instruction::Normal(NormalMneumonic::Jump)),
                Token::new(Position::new(1, 5), Operand::new_numeric(0)),
            )
        );
        assert_eq!(
            Operation::parse_assembler(Span::new("K /0")).unwrap().1,
            Operation::new(
                Token::new(Position::new(1, 1), Instruction::Normal(NormalMneumonic::SetConstant)),
                Token::new(Position::new(1, 3), Operand::new_numeric(0)),
            )
        );
    }
}
