use regex::Regex;

pub type Label = &'static str;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Simbolic(Label),
    Numeric(u16),
}

fn split_first(src: &str) -> (char, &str) {
    let mut charts = src.chars();
    let first_chart = charts.next().unwrap();
    (first_chart, charts.as_str())
}

impl Operand {
    pub fn parse(src: &'static str) -> Self {
        let (first_chart, charts) = split_first(src.clone());
        match first_chart {
            '/' => Operand::Numeric(u16::from_str_radix(charts, 16).unwrap()),
            _   => Operand::Simbolic(src)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mneumonic {
    Constant(Operand),
    Add(Operand),
    Load(Operand),
    Store(Operand),
    Address(Operand),
}
impl Mneumonic {
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

    pub fn value(mneumonic: Mneumonic) -> u16 {
        match mneumonic {
            // Pseudo Instructions
            Mneumonic::Constant(_) => 0x0000,
            Mneumonic::Address(_) => 0x0000,

            Mneumonic::Add(_) => 0x4000,
            Mneumonic::Load(_) => 0x8000,
            Mneumonic::Store(_) => 0x9000,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction(pub Option<Label>, pub Mneumonic);

impl Instruction {
    pub fn parse(line: &'static str) -> Self {
        println!("parsing {:?}", line);
        let re = Regex::new(r"(?P<labl>\S*) *(?P<mneu>\S+) +(?P<oprd>\S+) ?-{0, 2}.*").unwrap();
        let caps = re.captures(line).unwrap();
        println!("captures {:?}", caps);
        
        let labl = caps.name("labl").unwrap().as_str();
        let mneum = caps.name("mneu").unwrap().as_str();
        let oprd = caps.name("oprd").unwrap().as_str();
        let label = if labl.is_empty() {None} else { Some(labl) };

        Self::parse_splited(label, mneum, oprd)
    }

    pub fn parse_splited(label: Option<Label>, mneu: &str, oprd: &'static str) -> Self {
        let operand = Operand::parse(oprd);
        let mneumonic = Mneumonic::parse(mneu, operand);
        Self(label, mneumonic)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_contants_with_label() {
        let line = "VAL_A   K   /0001";
        let result = Instruction::parse(line);
        assert_eq!(result, Instruction(Some("VAL_A"), Mneumonic::Constant(Operand::Numeric(1))));
    }

    #[test]
    fn should_parse_address() {
        let line = "    @  /0100";
        let result = Instruction::parse(line);
        assert_eq!(result, Instruction(None, Mneumonic::Address(Operand::Numeric(0x100))));
    }

    #[test]
    fn should_parse_add_withoul_label() {
        let line = "    AD VAL_A";
        let result = Instruction::parse(line);
        assert_eq!(result, Instruction(None, Mneumonic::Add(Operand::Simbolic("VAL_A"))));
    }
}

