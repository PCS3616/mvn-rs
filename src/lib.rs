use std::convert::identity;

pub fn asm2mvn(asm: &str) -> &str {
    Code::parse(asm).to_mvn()
}

#[derive(Debug, PartialEq)]
struct Label(String);
impl Label {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, PartialEq)]
enum Operand {
    Simbolic(Label),
    Numeric(u16),
}
impl Operand {
    pub fn parse(src: &str) -> Self{
        u16::from_str_radix(src, 16).map_or_else(|_| Self::simbolic(src), Self::numeric)
    }
    fn simbolic(value: &str) -> Self { Operand::Simbolic(Label::new(value)) }
    fn numeric(value: u16) -> Self { Operand::Numeric(value) }
}

#[derive(Debug, PartialEq)]
enum Mneumonic {
    Constant(Operand),
    Add(Operand),
    Load(Operand),
    Store(Operand),
    Address(Operand),
}
impl Mneumonic {
    pub fn const_val(value: u16) -> Self {
        Mneumonic::Constant(Operand::Numeric(value))
    }

    pub fn add(label: &str) -> Self {
        Mneumonic::Add(Operand::Simbolic(Label::new(label)))
    }

    pub fn load(label: &str) -> Self {
        Mneumonic::Load(Operand::Simbolic(Label::new(label)))
    }

    pub fn store(label: &str) -> Self {
        Mneumonic::Store(Operand::Simbolic(Label::new(label)))
    }

    pub fn parse(mneumonic: &str, arg: Operand) -> Self {
        match mneumonic {
            "K" => Mneumonic::Constant(arg),
            "LD" => Mneumonic::Load(arg),
            "AD" => Mneumonic::Add(arg),
            "MM" => Mneumonic::Store(arg),
            "@"  => Mneumonic::Address(arg),
            _ => panic!("Invalid mneumonic")
        }
    }
}

#[derive(Debug, PartialEq)]
struct Address(u16);

#[derive(Debug, PartialEq)]
enum CodeLine {
    Instruction(Option<Label>, Mneumonic),
    NextAddress(Address)
}
impl CodeLine {
    pub fn with_label (label: &str, mneu: Mneumonic) -> Self {
        CodeLine::Instruction(Some(Label::new(label)), mneu)
    }
    pub fn mneumonic (mneu: Mneumonic) -> Self {
        CodeLine::Instruction(None, mneu)
    }
    pub fn address (addr: u16) -> Self {
        CodeLine::NextAddress(Address(addr))
    }

    // TODO: reduce unwrap
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None
        }
        
        let mut terms = line.split_whitespace();
        let first_term = terms.next();
        let second_term = terms.next();
        let third_term = terms.next();
        let forth_term = terms.next();

        if first_term.is_none() {
            return None
        }

        if !forth_term.is_none() && forth_term != Some("--") {
            let term = forth_term.unwrap();
            panic!("Error on parse {}", term);
        }

        let code_line = if let Some(third_term_value) = third_term {
            Self::parse_splited(first_term, second_term.unwrap(), third_term_value) 
        } else {
            Self::parse_splited(None, first_term.unwrap(), second_term.unwrap()) 
        };
        Some(code_line)
    }

    fn parse_splited(lab: Option<&str>, mneu: &str, arg: &str) -> Self {
        let label = lab.map(Label::new);
        let operand = Operand::parse(arg);
        let mneumonic = Mneumonic::parse(mneu, operand);
        Self::Instruction(label, mneumonic)
    }
}

#[derive(Debug, PartialEq)]
struct Code {
    pub lines: Vec<CodeLine>
}
impl Code {
    fn new(lines: Vec<CodeLine>) -> Self {
        Self { lines}
    }
    fn parse(code: &str) -> Self {
        Code::new(code.lines().map(CodeLine::parse).filter_map(identity).collect())
    }
    fn to_mvn(&self) -> &'static str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn should_returns_line_code_given_asm_mneumonic() {
        assert_eq!(CodeLine::parse("VAL_A   K   /0001"), Some(CodeLine::with_label("VAL_B", Mneumonic::const_val(2))));
        assert_eq!(CodeLine::parse("VAL_B   K   /0002"), Some(CodeLine::with_label("RESLT", Mneumonic::const_val(0))));
        assert_eq!(CodeLine::parse("RESLT   K   /0000"), Some(CodeLine::with_label("MAIN", Mneumonic::load("VAL_A"))));
        assert_eq!(CodeLine::parse("@   /0100"),         Some(CodeLine::address(0x100)));
        assert_eq!(CodeLine::parse("MAIN    LD  VAL_A"), Some(CodeLine::mneumonic(Mneumonic::add("VAL_B"))));
        assert_eq!(CodeLine::parse("AD  VAL_B"),         Some(CodeLine::mneumonic(Mneumonic::add("VAL_B"))));
        assert_eq!(CodeLine::parse("MM  RESLT"),         Some(CodeLine::mneumonic(Mneumonic::store("RESLT"))));
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
        let expected = Code::new(vec![
             CodeLine::with_label("VAL_A", Mneumonic::const_val(1)),
             CodeLine::with_label("VAL_B", Mneumonic::const_val(2)),
             CodeLine::with_label("RESLT", Mneumonic::const_val(0)),
             CodeLine::address(0x100),
             CodeLine::with_label("MAIN", Mneumonic::load("VAL_A")),
             CodeLine::mneumonic(Mneumonic::add("VAL_B")),
             CodeLine::mneumonic(Mneumonic::store("RESLT")),
        ]);
        assert_eq!(Code::parse(input), expected);
    }

    #[test]
    fn should_returns_mvn_given_code() {
        let input = Code::new(vec![
             CodeLine::with_label("VAL_A", Mneumonic::const_val(1)),
             CodeLine::with_label("VAL_B", Mneumonic::const_val(2)),
             CodeLine::with_label("RESLT", Mneumonic::const_val(0)),
             CodeLine::address(0x100),
             CodeLine::with_label("MAIN", Mneumonic::load("VAL_A")),
             CodeLine::mneumonic(Mneumonic::add("VAL_B")),
             CodeLine::mneumonic(Mneumonic::store("RESLT")),
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
}
