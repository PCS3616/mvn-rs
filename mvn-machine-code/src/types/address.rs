use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct MachineAddress {
    pub properties: MachineAddressProperties,
    pub position: u32,
}

impl MachineAddress {
    pub fn new(properties: MachineAddressProperties, position: u32) -> Self {
        MachineAddress {
            properties,
            position,
        }
    }
}

impl fmt::UpperHex for MachineAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}{:03X}", self.properties, self.position)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
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

impl fmt::UpperHex for MachineAddressProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = ((self.line_relocatable as u8) << 2)
            + ((self.operand_relocatable as u8) << 1)
            + self.operand_imported as u8;
        write!(f, "{:X}", value)
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
