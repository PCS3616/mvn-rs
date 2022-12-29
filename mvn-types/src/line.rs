use std::fmt;

use super::Label;
use super::Operation;

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub label: Option<Label<'a>>,
    pub operation: Operation<'a>,
}

impl<'a> Line<'a> {
    pub fn new(label: Option<Label<'a>>, operation: Operation<'a>) -> Self {
        Self { label, operation }
    }
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = if let Some(label) = &self.label {
            label.to_string()
        } else {
            "".to_owned()
        };
        write!(f, "{}\t\t{}", label, &self.operation)
    }
}
