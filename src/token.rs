#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Illegal,
    Number,
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}
impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}
