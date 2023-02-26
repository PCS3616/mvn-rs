use nom::character::complete::space1;
use nom::combinator::{map, not};
use nom::sequence::terminated;
use utils::error_or;

use super::error::{LocatedIResult, Span};
use super::identifier;
use super::Parse;
use crate::types::{Instruction, Label};

impl<'a> Parse<'a> for Label<'a> {
    fn parse_assembler(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let label = not(terminated(Instruction::parse_assembler, space1))(input)
            .and_then(|(input, _)| map(identifier, Self::new)(input));
        error_or!(
            label,
            input,
            "invalid label; perhaps you used non-ASCII characters or started with a number"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::mneumonic::*;
    use pretty_assertions::assert_eq;

    fn normal_mneumonics() -> [NormalMneumonic; 17] {
        [
            NormalMneumonic::Jump,
            NormalMneumonic::JumpIfZero,
            NormalMneumonic::JumpIfNegative,
            NormalMneumonic::LoadValue,
            NormalMneumonic::Add,
            NormalMneumonic::Subtract,
            NormalMneumonic::Multiply,
            NormalMneumonic::Divide,
            NormalMneumonic::Load,
            NormalMneumonic::Memory,
            NormalMneumonic::Subroutine,
            NormalMneumonic::ReturnFromSubrotine,
            NormalMneumonic::HaltMachine,
            NormalMneumonic::GetData,
            NormalMneumonic::PutData,
            NormalMneumonic::OperatingSystem,
            NormalMneumonic::SetConstant,
        ]
    }

    fn positional_mneumonics() -> [PositionalMneumonic; 4] {
        [
            PositionalMneumonic::SetAbsoluteOrigin,
            PositionalMneumonic::SetRelocatableOrigin,
            PositionalMneumonic::ReserveMemory,
            PositionalMneumonic::SetEnd,
        ]
    }

    fn relational_mneumonics() -> [RelationalMneumonic; 2] {
        [RelationalMneumonic::Export, RelationalMneumonic::Import]
    }

    #[test]
    fn should_parse_label() {
        let inputs = ["VAL_A", "V1"];
        for input in inputs.into_iter() {
            let output = Label::new(input);
            assert_eq!(Label::parse_assembler(Span::new(input)).unwrap().1, output,);
        }
        assert!(Label::parse_assembler(Span::new("1V")).is_err());
    }

    #[test]
    fn should_parse_label_starting_with_mneumonic() {
        let label = format!("{}FOO", NormalMneumonic::Jump.to_string());
        assert!(Label::parse_assembler(label.as_str().into()).is_ok());
    }

    #[test]
    fn should_not_parse_normal_mneumonic() {
        for mneumonic in normal_mneumonics().into_iter() {
            let mut mneumonic = mneumonic.to_string();
            mneumonic.push_str(" ");
            assert!(Label::parse_assembler(mneumonic.as_str().into()).is_err());
        }
    }

    #[test]
    fn should_not_parse_positional_mneumonic() {
        for mneumonic in positional_mneumonics().into_iter() {
            let mut mneumonic = mneumonic.to_string();
            mneumonic.push_str(" ");
            assert!(Label::parse_assembler(mneumonic.as_str().into()).is_err());
        }
    }

    #[test]
    fn should_not_parse_relational_mneumonic() {
        for mneumonic in relational_mneumonics().into_iter() {
            let mut mneumonic = mneumonic.to_string();
            mneumonic.push_str(" ");
            assert!(Label::parse_assembler(mneumonic.as_str().into()).is_err());
        }
    }
}
