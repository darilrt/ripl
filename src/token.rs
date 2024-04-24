use std::fmt::Debug;
use std::fmt::{self, Formatter};

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lit: String,
    pub loc: Location,
}

#[derive(Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Unknown,
    Eof,
    Ident,
    Integer,
    Float,
    String,
    Symbol,
    Keyword,
    Operator,
    Comment,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.lit)
    }
}
