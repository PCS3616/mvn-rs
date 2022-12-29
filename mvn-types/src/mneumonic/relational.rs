use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RelationalMneumonic {
    Export,
    Import,
}

impl fmt::Display for RelationalMneumonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mneumonic = match &self {
            Self::Export => ">",
            Self::Import => "<",
        };
        write!(f, "{mneumonic}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn relational_pseudo_mneumonic_should_convert_to_string() {
        assert_eq!(RelationalMneumonic::Export.to_string(), ">");
        assert_eq!(RelationalMneumonic::Import.to_string(), "<");
    }
}
