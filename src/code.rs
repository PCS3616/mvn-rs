use crate::intruction::Instruction;

pub fn asm2mvn(asm: &'static str) -> &str {
    Code::parse(asm).to_mvn()
}

#[derive(Debug, PartialEq)]
pub struct Code(Vec<Instruction>);

impl Code {
    pub fn parse(code: &'static str) -> Self {
        Code(code.lines()
             .map(Instruction::parse)
             .collect())
    }
    pub fn to_mvn(&self) -> &'static str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intruction::{Mneumonic, Operand};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_returns_mvn_given_asm() {
        let input = indoc! {"
            VAL_A   K   /0001
            VAL_B   K   /0002
            RESLT   K   /0000
                    @   /0100
            MAIN    LD  VAL_A
                    AD  VAL_B
                    MM  RESLT
        "};

        let expected = indoc! {"
            00000001
            00020002
            00040000
            01008000
            01024002
            01049004
        "};

        assert_eq!(asm2mvn(input), expected);
    }

    #[test]
    fn should_returns_code_given_asm() {
        let input = indoc! {"
            VAL_A   K   /0001
            VAL_B   K   /0002
            RESLT   K   /0000
                    @   /0100
            MAIN    LD  VAL_A
                    AD  VAL_B
                    MM  RESLT
        "};
        let expected = Code(vec![
             Instruction(Some("VAL_A"), Mneumonic::Constant(Operand::Numeric(1))),
             Instruction(Some("VAL_B"), Mneumonic::Constant(Operand::Numeric(2))),
             Instruction(Some("RESLT"), Mneumonic::Constant(Operand::Numeric(0))),
             Instruction(None, Mneumonic::Address(Operand::Numeric(0x100))),
             Instruction(Some("MAIN"), Mneumonic::Load(Operand::Simbolic("VAL_A"))),
             Instruction(None, Mneumonic::Add(Operand::Simbolic("VAL_B"))),
             Instruction(None, Mneumonic::Store(Operand::Simbolic("RESLT"))),
        ]);
        assert_eq!(Code::parse(input), expected);
    }

    #[test]
    fn should_returns_mvn_given_code() {
        let input = Code(vec![
             Instruction(Some("VAL_A"), Mneumonic::Constant(Operand::Numeric(1))),
             Instruction(Some("VAL_B"), Mneumonic::Constant(Operand::Numeric(2))),
             Instruction(Some("RESLT"), Mneumonic::Constant(Operand::Numeric(0))),
             Instruction(None, Mneumonic::Address(Operand::Numeric(0x100))),
             Instruction(Some("MAIN"), Mneumonic::Load(Operand::Simbolic("VAL_A"))),
             Instruction(None, Mneumonic::Add(Operand::Simbolic("VAL_B"))),
             Instruction(None, Mneumonic::Add(Operand::Simbolic("RESLT"))),
        ]);
        let expected = indoc! {"
            00000001
            00020002
            00040000
            01008000
            01024002
            01049004
        "};

        assert_eq!(input.to_mvn(), expected);
    }
}
