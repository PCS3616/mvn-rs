use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::branch::alt;
use nom::multi::many0_count;
use nom::sequence::pair;
use nom::combinator::recognize;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Label<'a>(&'a str);

impl<'a> Label<'a> {
    fn new(label: &'a str) -> Self {
        Label(label)
    }
}

/*
 * Parsing identifiers that may start with a letter (or underscore)
 * and may contain underscores, letters and numbers may be parsed like this
 *
 * From: https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#identifiers
 */
pub fn identifier(input: &str) -> IResult<&str, &str> {
  recognize(
    pair(
      alt((alpha1, tag("_"))),
      many0_count(alt((alphanumeric1, tag("_"))))
    )
  )(input)
}

pub fn parse_label(input: &str) -> IResult<&str, Label> {
    let (rest, label) = identifier(input)?;
    Ok((rest, Label::new(label)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_label() {
        assert_eq!(parse_label("VAL_A"), Ok(("", Label("VAL_A"))));
        assert_eq!(parse_label("V1"), Ok(("",Label("V1"))));
        assert!(parse_label("1V").is_err());
    }
}

