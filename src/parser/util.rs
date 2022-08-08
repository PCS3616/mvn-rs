use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, one_of, char};
use nom::branch::alt;
use nom::multi::{many0_count, many1, many0};
use nom::sequence::{pair, terminated};
use nom::combinator::{recognize, map_res};
use nom::IResult;
use num_traits::Num;

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

/*
 *
 * From: https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#hexadecimal
 */
pub fn hexadecimal<T: Num>(input: &str) -> IResult<&str, T> {
  map_res(
    recognize(
      many1(
        terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
      )
    ),
    |out: &str| T::from_str_radix(&str::replace(&out, "_", ""), 16)
  )(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_hexa() {
        assert_eq!(hexadecimal("F"), Ok(("", 0xF)));
        assert_eq!(hexadecimal("0"), Ok(("", 0x0)));
    }
}

