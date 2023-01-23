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
