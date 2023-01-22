pub mod instruction;
pub mod label;
pub mod line;
pub mod mneumonic;
pub mod operand;
pub mod operation;
pub mod program;

use nom::IResult;
use nom_locate::position;
pub use utils::error;

pub trait Parse<'a>: Sized {
    fn parse(input: error::Span<'a>) -> error::LocatedIResult<'a, Self>;
}

impl<'a, T: Parse<'a>> Parse<'a> for types::Token<T> {
    fn parse(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (input, position) = position(input)?;
        let position = position.into();
        let (rest, value) = T::parse(input)?;
        let token = types::Token { position, value };
        Ok((rest, token))
    }
}

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char};
use nom::character::complete::{not_line_ending, space0};
use nom::combinator::{map, recognize};
use nom::combinator::{opt, value};
use nom::error::ParseError;
use nom::multi::{many0_count, many1, separated_list1};
use nom::sequence::pair;
use nom::InputLength;

/*
 * Parsing identifiers that may start with a letter (or underscore)
 * and may contain underscores, letters and numbers may be parsed like this
 *
 * From: https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#identifiers
 */
pub fn identifier(input: error::Span) -> error::LocatedIResult<&str> {
    let (remainder, matched) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)?;
    Ok((remainder, *matched))
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
) -> impl FnMut(I) -> IResult<I, Vec<O2>, E> {
    map(
        separated_list1(many1(sep), alt_opt(usable, ignorable)),
        |ls| ls.into_iter().flatten().collect(),
    )
}

pub fn comment_or_space(input: error::Span) -> error::LocatedIResult<()> {
    value(
        (), // Output is thrown away.
        pair(space0, opt(pair(char(';'), not_line_ending))),
    )(input)
}
