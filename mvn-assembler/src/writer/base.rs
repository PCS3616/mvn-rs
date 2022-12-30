use crate::processor::address::{AddressedProgram, AddressedLine, Address};
use types::{Line, Instruction, Operand};

pub fn print(program: &AddressedProgram) -> () {
    let labels = program.map_labels();
    let default_address = Address { ..Default::default() };

    for AddressedLine { address, line } in program.lines.iter() {
        let Line { label: _, operation } = line;
        if let Instruction::Positional(_) = operation.instruction {
            continue;
        }

        let operand_address = if let Operand::Symbolic(label) = &operation.operand {
            labels.get(&label).unwrap()
        } else {
            &default_address
        };

        let operand_value: u16 = if let Operand::Numeric(value) =  &operation.operand {
            *value
        } else {
            operand_address.position
        };

        let instruction_value = if let Instruction::Normal(mneumonic) = operation.instruction {
            mneumonic.into()
        } else {
            0
        };

        let operation_value = ((instruction_value as u16) << 12) + operand_value;

        let nibble_value = resolve_nibble(address, operand_address);

        let operation_address = ((nibble_value as u16) << 12) + address.position;

        print!("{:04X} {:04X}", operation_address, operation_value);
        if let Instruction::Relational(relational_mneumonic) = &operation.instruction {
            if let Operand::Symbolic(relational_label) = &operation.operand {
                print!(" ; {} {}", relational_mneumonic, relational_label.0);
            }
        }
        print!("\n");
    }
}


fn resolve_nibble(line: &Address, operand: &Address) -> u8 {
    ((0 as u8) << 3) // One bit is not necessary, so it's fixed at zero
    + ((line.relocatable as u8) << 2)
    + ((operand.relocatable as u8) << 1)
    + (operand.imported as u8)
}
