use crate::Line;

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        Self { lines }
    }
}

impl<'a> IntoIterator for Program<'a> {
    type Item = Line<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}
