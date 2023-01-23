mod address;
mod instruction;
pub mod line;
mod operation;
pub mod program;

pub use utils::error;

use crate::types::AddressPosition;

pub trait Parse<'a>: Sized {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self>;
}

pub trait Relocate: Sized {
    fn relocate(self, base: AddressPosition) -> Self;
}

use utils::types::Token;
use nom_locate::position;

impl<'a, T: Parse<'a>> Parse<'a> for Token<T> {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self> {
        let (input, position) = position(input)?;
        let position = position.into();
        let (rest, value) = T::parse_machine_code(input)?;
        let token = Token { position, value };
        Ok((rest, token))
    }
}
