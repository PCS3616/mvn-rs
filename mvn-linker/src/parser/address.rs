use std::convert::{TryFrom, From};

use nom::bytes::complete::take;
use utils::hexadecimal;

use super::error;
use super::{Parse, Position, Relocate};

#[derive(Debug, PartialEq)]
pub struct MachineAddress {
    pub properties: MachineAddressProperties,
    pub position: u32,
}

impl MachineAddress {
    pub fn new(properties: MachineAddressProperties, position: u32) -> Self {
        MachineAddress { properties, position }
    }

}

impl Relocate for MachineAddress {
    fn relocate(self, base: Position) -> Self{
        // TODO Add error treatment
        Self::new(self.properties, self.position + base)
    }
}

impl From<MachineAddress> for u32 {
    fn from(value: MachineAddress) -> Self {
        value.position
    }
}

impl Parse<'_> for MachineAddress {
    fn parse_machine_code(input: error::Span) -> error::LocatedIResult<Self> {
        let (position, properties) = take(1usize)(input)?;
        let (_, properties) = MachineAddressProperties::parse_machine_code(properties)?;
        let (rest, position) = hexadecimal::<u32>(position)?;
        Ok((rest, MachineAddress::new(properties, position)))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MachineAddressProperties {
    pub line_relocatable: bool,
    pub operand_relocatable: bool,
    pub operand_imported: bool,
}


impl MachineAddressProperties {
    pub fn new(line_relocatable: bool, operand_relocatable: bool, operand_imported: bool) -> Self {
            MachineAddressProperties {
                line_relocatable,
                operand_relocatable,
                operand_imported,
            }
    }
}

impl TryFrom<u8> for MachineAddressProperties {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let line_relocatable = (value & 0b0100) != 0;
        let operand_relocatable = (value & 0b0010) != 0;
        let operand_imported = (value & 0b0001) != 0;

        if operand_relocatable && operand_imported {
            Err("invalid address properties")
        } else {
            Ok(MachineAddressProperties::new(
                line_relocatable,
                operand_relocatable,
                operand_imported,
            ))
        }
    }
}

impl Parse<'_> for MachineAddressProperties {
    fn parse_machine_code(input: error::Span) -> error::LocatedIResult<Self> {
        let (_, nibble) = hexadecimal::<u8>(input)?;
        let properties = MachineAddressProperties::try_from(nibble);
        match properties {
            Err(_) => Err(nom::Err::Error(error::MvnParseError::new(
                "invalid address properties".to_owned(),
                input
            ))),
            Ok(properties) => Ok(("".into(), properties)),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn should_parse_address_properties() {
        let inputs_outputs = vec![
            ("0", MachineAddressProperties::new(false, false, false)),
            ("1", MachineAddressProperties::new(false, false, true)),
            ("2", MachineAddressProperties::new(false, true, false)),
            ("4", MachineAddressProperties::new(true, false, false)),
            ("5", MachineAddressProperties::new(true, false, true)),
            ("6", MachineAddressProperties::new(true, true, false)),
        ];
        for (input, output) in inputs_outputs {
            assert_eq!(
                MachineAddressProperties::parse_machine_code(input.into()).unwrap().1,
                output
            );
        }
    }

    #[test]
    fn should_parse_address() {
        let properties = MachineAddressProperties::new(false, false, false);
        let inputs_outputs = vec![
            ("0000", MachineAddress::new(properties, 0)),
            ("0002", MachineAddress::new(properties, 2)),
            ("000A", MachineAddress::new(properties, 10)),
            ("000E", MachineAddress::new(properties, 14)),
            ("0010", MachineAddress::new(properties, 16)),
        ];
        for (input, output) in inputs_outputs {
            assert_eq!(
                MachineAddress::parse_machine_code(input.into()).unwrap().1,
                output
            );
        }
    }
}
