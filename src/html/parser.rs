#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Parser {
    idx: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { idx: 0 }
    }

    pub fn parse(&mut self, _html: &str) -> usize {
        1
    }
}

mod tests {
    #[test]
    fn test() {}
}
