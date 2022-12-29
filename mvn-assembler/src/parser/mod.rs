pub mod instruction;
pub mod label;
pub mod line;
pub mod mneumonic;
pub mod operand;
pub mod operation;
pub mod program;

pub(crate) trait Parse<'a>: Sized {
    fn parse(input: &'a str) -> IResult<&'a str, Self>;
}

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, one_of};
use nom::character::complete::{not_line_ending, space0};
use nom::combinator::{map, map_res, recognize};
use nom::combinator::{opt, value};
use nom::error::ParseError;
use nom::multi::{many0, many0_count, many1, separated_list1};
use nom::sequence::{pair, terminated};
use nom::IResult;
use nom::InputLength;
use num_traits::Num;

/*
 * Parsing identifiers that may start with a letter (or underscore)
 * and may contain underscores, letters and numbers may be parsed like this
 *
 * From: https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#identifiers
 */
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

/*
 *
 * From: https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#hexadecimal
 */
fn hexadecimal<T: Num>(input: &str) -> IResult<&str, T> {
    map_res(
        recognize(many1(terminated(
            one_of("0123456789abcdefABCDEF"),
            many0(char('_')),
        ))),
        |out: &str| T::from_str_radix(&str::replace(&out, "_", ""), 16),
    )(input)
}

/*
 * if parse with usable return Some(result),
 * if parse with ignorable, return None
 */
fn alt_opt<I: Clone, O, O2, E: ParseError<I>>(
    usable: impl FnMut(I) -> IResult<I, O, E>,
    ignorable: impl FnMut(I) -> IResult<I, O2, E>,
) -> impl FnMut(I) -> IResult<I, Option<O>, E> {
    alt((
        map(usable, |l| Some(l)),
        map(ignorable, |_| None), // Linha em branco
    ))
}

fn separated_list1_opt<I: Clone + InputLength, O, O2, O3, E: ParseError<I>>(
    sep: impl FnMut(I) -> IResult<I, O, E>,
    usable: impl FnMut(I) -> IResult<I, O2, E>,
    ignorable: impl FnMut(I) -> IResult<I, O3, E>,
) -> impl FnMut(I) -> IResult<I, Vec<O2>, E> {
    map(
        separated_list1(many1(sep), alt_opt(usable, ignorable)),
        |ls| ls.into_iter().flatten().collect(),
    )
}

fn comment_or_space<'a>(input: &'a str) -> IResult<&'a str, ()> {
    value(
        (), // Output is thrown away.
        pair(space0, opt(pair(char(';'), not_line_ending))),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_hexa() {
        assert_eq!(hexadecimal("F"), Ok(("", 0xF)));
        assert_eq!(hexadecimal("0"), Ok(("", 0x0)));
    }
}
