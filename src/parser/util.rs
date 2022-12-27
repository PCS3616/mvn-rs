use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, one_of, char};
use nom::branch::alt;
use nom::error::ParseError;
use nom::multi::{many0_count, many1, many0, separated_list1};
use nom::sequence::{pair, terminated};
use nom::combinator::{recognize, map_res, map};
use nom::{IResult, InputLength};
use num_traits::Num;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type LocatedIResult<'a, O> = nom::IResult<Span<'a>, O>;

/*
 * Parsing identifiers that may start with a letter (or underscore)
 * and may contain underscores, letters and numbers may be parsed like this
 *
 * From: https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#identifiers
 */
pub fn identifier(input: Span) -> LocatedIResult<&str> {
  let (remainder, matched) = recognize(
    pair(
      alt((alpha1, tag("_"))),
      many0_count(alt((alphanumeric1, tag("_"))))
    )
  )(input)?;
  Ok((remainder, *matched))
}

/*
 *
 * From: https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#hexadecimal
 */
pub fn hexadecimal<T: Num>(input: Span) -> LocatedIResult<T> {
  map_res(
    recognize(
      many1(
        terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
      )
    ),
    |out: Span| T::from_str_radix(&str::replace(&out, "_", ""), 16)
  )(input)
}

/*
 * if parse with usable return Some(result),
 * if parse with ignorable, return None
 */
pub fn alt_opt<I: Clone, O, O2, E: ParseError<I>>(
    usable: impl FnMut(I) -> IResult<I, O, E>,
    ignorable: impl FnMut(I) -> IResult<I, O2, E>,
) -> impl FnMut(I) -> IResult<I, Option<O>, E> {
    alt((
        map(usable, |l| Some(l)),
        map(ignorable, |_| None), // Linha em branco
    ))
}


pub fn separated_list1_opt<I: Clone + InputLength, O, O2, O3, E: ParseError<I>>(
  sep: impl FnMut(I) -> IResult<I, O, E>,
  usable: impl FnMut(I) -> IResult<I, O2, E>,
  ignorable: impl FnMut(I) -> IResult<I, O3, E>,
) -> impl FnMut(I) -> IResult<I, Vec<O2>, E>
{
    map(
        separated_list1(
            many1(sep),
            alt_opt(usable, ignorable)
        ),
        |ls| ls.into_iter().flatten().collect()
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_hexa() {
        assert_eq!(hexadecimal::<u8>(Span::new("F")).unwrap().1, 0xF);
        assert_eq!(hexadecimal::<u8>(Span::new("0")).unwrap().1, 0x0);
    }
}

