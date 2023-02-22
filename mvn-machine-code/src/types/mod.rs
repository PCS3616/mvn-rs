mod address;
mod line;
mod program;

// TODO Reference utils::types::AddressPosition
pub type AddressPosition = u32;

pub use address::{MachineAddress, MachineAddressProperties};
pub use line::AddressedLine;
pub use program::AddressedProgram;

pub use assembly::types::{
    Instruction,
    Operand,
    Operation,
    Label,
    mneumonic,
};
