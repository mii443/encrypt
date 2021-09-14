#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    CONTROL,
    RETURN,
    RESERVED,
    IDENT,
    NUMBER,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub num: usize,
    pub str: String,
}
