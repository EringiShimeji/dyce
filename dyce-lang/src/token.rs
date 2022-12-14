#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Illegal,     // 解析できないトークン
    Number,      // 数字
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Eq,          // = ==
    Ne,          // != <>
    Lt,          // <
    Le,          // <=
    Gt,          // >
    Ge,          // >=
    Comma,       // ,
    DoubleQuote, // "
    LParen,      // (
    RParen,      // )
    Ident,       // 識別子
    Separator,   // 定義を区切るための改行群
    Arrow,       // =>
}
impl Default for TokenKind {
    fn default() -> Self {
        Self::Illegal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}
impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn literal(&self) -> String {
        self.literal.clone()
    }
}
