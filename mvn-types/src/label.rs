use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Label<'a>(pub &'a str);

// FIXME Store Span instead of str to locate after parsing
impl<'a> Label<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(input)
    }
}

impl<'a> fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
