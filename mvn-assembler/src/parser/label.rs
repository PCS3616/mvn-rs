use nom::combinator::map;
use nom::IResult;

use super::identifier;
use super::Parse;

impl<'a> Parse<'a> for types::Label<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(identifier, |out: &str| Self::new(out))(input)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use types::Label;

    use super::*;

    #[test]
    fn should_parse_label() {
        assert_eq!(Label::parse("VAL_A"), Ok(("", Label::new("VAL_A"))));
        assert_eq!(Label::parse("V1"), Ok(("", Label::new("V1"))));
        assert!(Label::parse("1V").is_err());
    }
}
