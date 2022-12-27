use std::fmt;

use nom::combinator::map;

use crate::parser::util::{identifier, LocatedIResult, Span};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Label<'a>(
    pub &'a str
);

impl<'a> Label<'a> {
    pub fn new(label: &'a str) -> Self {
        Self(label)
    }

    pub fn parse(input: Span<'a>) -> LocatedIResult<Self> {
        map(
            identifier,
            |out: &str| Self::new(out)
        )(input)
    }
}

impl<'a> fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Label({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_label() {
        let inputs = ["VAL_A", "V1"];
        for input in inputs.into_iter() {
            let output = Label(input);
            assert_eq!(
                Label::parse(Span::new(input)).unwrap().1,
                output,
            );
        }
        assert!(Label::parse(Span::new("1V")).is_err());
    }
}

