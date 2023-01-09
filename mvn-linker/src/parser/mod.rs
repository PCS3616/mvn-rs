mod address;
mod instruction;
mod line;
mod operation;
mod program;

pub use utils::error;

pub trait Parse<'a>: Sized {
    fn parse_machine_code(input: error::Span<'a>) -> error::LocatedIResult<'a, Self>;
}
