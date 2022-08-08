use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::branch::alt;
use nom::multi::many0_count;
use nom::sequence::pair;
use nom::combinator::recognize;
use nom::IResult;

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
