use nom::IResult;

use crate::parser::util::identifier;

#[derive(Debug, PartialEq)]
pub struct Label<'a>(&'a str);

impl<'a> Label<'a> {
    pub fn new(label: &'a str) -> Self {
        Label(label)
    }

    pub fn parse(input: &str) -> IResult<&str, Label> {
        let (rest, label) = identifier(input)?;
        Ok((rest, Label::new(label)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_label() {
        assert_eq!(Label::parse("VAL_A"), Ok(("", Label("VAL_A"))));
        assert_eq!(Label::parse("V1"), Ok(("",Label("V1"))));
        assert!(Label::parse("1V").is_err());
    }
}

