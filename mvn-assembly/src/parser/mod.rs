pub mod instruction;
pub mod label;
pub mod line;
pub mod mneumonic;
pub mod operand;
pub mod operation;
pub mod program;

use nom_locate::position;
pub use utils::error;
use utils::types::Token;

pub trait Parse<'a>: Sized {
    fn parse_assembler(input: error::Span<'a>) -> error::LocatedIResult<'a, Self>;
}

impl<'a, T: Parse<'a>> Parse<'a> for Token<T> {
    fn parse_assembler(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (input, position) = position(input)?;
        let position = position.into();
        let (rest, value) = T::parse_assembler(input)?;
        let token = Token { position, value };
        Ok((rest, token))
    }
}

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::recognize;
use nom::multi::many0_count;
use nom::sequence::pair;

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
