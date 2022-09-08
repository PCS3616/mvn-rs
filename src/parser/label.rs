use std::fmt;

use nom::IResult;
use nom::combinator::map;

use crate::parser::util::identifier;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Label<'a>(&'a str);

impl<'a> Label<'a> {
    pub fn new(label: &'a str) -> Self {
        Self(label)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
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
        assert_eq!(Label::parse("VAL_A"), Ok(("", Label("VAL_A"))));
        assert_eq!(Label::parse("V1"), Ok(("",Label("V1"))));
        assert!(Label::parse("1V").is_err());
    }
}

