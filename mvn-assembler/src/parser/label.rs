use nom::combinator::map;
use utils::error_or;

use super::error::{LocatedIResult, Span};
use super::identifier;
use super::Parse;

impl<'a> Parse<'a> for types::Label<'a> {
    fn parse(input: Span<'a>) -> LocatedIResult<'a, Self> {
        let label = map(identifier, |out: &str| Self::new(out))(input);
        error_or!(
            label,
            input,
            "invalid label; perhaps you started with a number?"
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::Label;

    use super::*;

    #[test]
    fn should_parse_label() {
        let inputs = ["VAL_A", "V1"];
        for input in inputs.into_iter() {
            let output = Label::new(input);
            assert_eq!(Label::parse(Span::new(input)).unwrap().1, output,);
        }
        assert!(Label::parse(Span::new("1V")).is_err());
    }
}
