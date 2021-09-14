use crate::source::*;
use crate::token::*;

#[derive(Clone)]
pub struct Tokenizer {
    pub tokens: Vec<Token>,
    pub cursor: usize,
}
impl Tokenizer {
    pub fn current_token(&mut self) -> &mut Token {
        &mut self.tokens[self.cursor]
    }

    pub fn consume(&mut self, op: String) -> bool {
        return if self.current_token().kind != TokenKind::RESERVED || self.current_token().str != op
        {
            false
        } else {
            self.cursor += 1;
            true
        };
    }

    pub fn consume_kind(&mut self, kind: TokenKind) -> bool {
        return if self.current_token().kind != kind {
            false
        } else {
            self.cursor += 1;
            true
        };
    }

    pub fn consume_kind_str(&mut self, kind: TokenKind, string: String) -> bool {
        return if self.current_token().kind == kind && self.current_token().str == string {
            self.cursor += 1;
            true
        } else {
            false
        };
    }

    pub fn expect(&mut self, op: String) -> Result<(), String> {
        if self.current_token().str != op {
            return Err(format!("Unexpected type : {}", op));
        }
        self.cursor += 1;
        Ok(())
    }

    pub fn expect_kind(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.current_token().kind != kind {
            return Err(format!("Unexpected token: {:?}", self.current_token().kind));
        }
        self.cursor += 1;
        Ok(())
    }

    pub fn expect_ident(&mut self) -> Result<String, String> {
        if self.current_token().kind != TokenKind::IDENT {
            return Err(format!(
                "Unexpected type : {:?}",
                self.current_token().kind
            ));
        }
        let val = self.current_token().str.clone();
        self.cursor += 1;
        Ok(val.to_string())
    }

    pub fn expect_number(&mut self) -> Result<usize, String> {
        let kind = self.current_token().kind;
        if kind != TokenKind::NUMBER {
            return Err(format!("Unexpected type : {:?}", kind));
        }
        let val = self.current_token().num;
        self.cursor += 1;
        Ok(val)
    }

    pub fn new() -> Tokenizer {
        Tokenizer {
            cursor: 0,
            tokens: vec![],
        }
    }

    pub fn create_reserved(op: String) -> Token {
        Token {
            kind: TokenKind::RESERVED,
            str: op,
            num: 0,
        }
    }

    pub fn create_number(num: usize) -> Token {
        Token {
            kind: TokenKind::NUMBER,
            num: num,
            str: String::default(),
        }
    }

    pub fn tokenize(&mut self, source: &mut Source) -> Result<Vec<Token>, String> {
        let reserved: Vec<String> = vec![
            String::from("+="),
            String::from("-="),
            String::from("*="),
            String::from("/="),
            String::from("+"),
            String::from("-"),
            String::from("*"),
            String::from("/"),
            String::from("&&"),
            String::from("&"),
            String::from("{"),
            String::from("}"),
            String::from("("),
            String::from(")"),
            String::from("=="),
            String::from("!="),
            String::from(">="),
            String::from("<="),
            String::from("<"),
            String::from(">"),
            String::from("="),
            String::from(";"),
            String::from(":"),
        ];

        let controls: Vec<String> = vec![
            String::from("for"),
            String::from("while"),
            String::from("if"),
            String::from("else"),
        ];

        while source.has_next() {
            match source.get_char(is_whitespace) {
                Ok(_) => {
                    continue;
                }
                Err(_) => {}
            }
            match contains_list_chars(source, reserved.clone()) {
                Ok(op) => {
                    self.tokens
                        .push(Tokenizer::create_reserved(String::from(op)));
                    continue;
                }
                Err(_) => {}
            }
            match source.get_chars(is_digit) {
                Ok(num) => {
                    self.tokens
                        .push(Tokenizer::create_number(num.parse().unwrap()));
                    continue;
                }
                Err(_) => {}
            }
            match source.get_chars(or(is_ascii_lowercase, or(is_digit, is('_')))) {
                Ok(c) => {
                    if c == String::from("return") {
                        self.tokens.push(Token {
                            kind: TokenKind::RETURN,
                            str: String::default(),
                            num: 0,
                        });
                        continue;
                    }

                    if controls.contains(&c) {
                        self.tokens.push(Token {
                            kind: TokenKind::CONTROL,
                            str: c,
                            num: 0,
                        });
                        continue;
                    }

                    self.tokens.push(Token {
                        kind: TokenKind::IDENT,
                        str: c,
                        num: 0,
                    });
                    continue;
                }
                Err(_) => {}
            }
            return Err(String::from("Failed to tokenize"));
        }

        self.tokens.push(Token {
            kind: TokenKind::EOF,
            str: String::default(),
            num: 0,
        });

        Ok(self.tokens.clone())
    }
}

fn contains_list_chars(source: &mut Source, list: Vec<String>) -> Result<String, String> {
    for target in list {
        match source.get_string(target) {
            Ok(string) => {
                return Ok(string);
            }
            Err(_) => {}
        }
    }
    return Err(String::from(""));
}

fn or(f: impl Fn(char) -> bool, g: impl Fn(char) -> bool) -> impl Fn(char) -> bool {
    move |c| f(c) || g(c)
}

fn is(ch: char) -> impl Fn(char) -> bool {
    move |c| c == ch
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_ascii_lowercase(c: char) -> bool {
    c.is_ascii_lowercase()
}
