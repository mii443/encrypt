use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    CONTROL,
    RETURN,
    RESERVED,
    IDENT,
    NUMBER,
    EOF,
    TEXT,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub num: i64,
    pub str: String,
}
