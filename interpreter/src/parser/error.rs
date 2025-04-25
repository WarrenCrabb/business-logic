use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError {
    pub msg: String,
    // pub pos: TokenPosition,
}

impl ParserError {
    pub fn new(msg: String) -> ParserError {
        ParserError { msg }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {} ", self.msg)
    }
}
