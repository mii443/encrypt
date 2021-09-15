use crate::node::*;
use crate::token::*;
use crate::tokenizer::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Parser {
    pub tokenizer: Tokenizer,
    pub local_vars: HashMap<String, usize>,
}

impl Parser {
    pub fn program(&mut self) -> Result<Vec<Box<Node>>, String> {
        let mut nodes: Vec<Box<Node>> = vec![];
        loop {
            if self.tokenizer.current_token().kind != TokenKind::EOF {
                nodes.push(self.stmt()?);
            } else {
                return Ok(nodes);
            }
        }
    }

    pub fn stmt(&mut self) -> Result<Box<Node>, String> {
        if self
            .tokenizer
            .consume_kind_str(TokenKind::IDENT, String::from("let"))
        {
            let ident = self.tokenizer.current_token().clone();
            self.tokenizer.expect_kind(TokenKind::IDENT)?;
            self.tokenizer.expect(String::from(":"))?;
            let var_type = self.tokenizer.current_token().clone();
            self.tokenizer.expect_kind(TokenKind::IDENT)?;
            self.tokenizer.expect(String::from(";"))?;
            return Ok(Box::new(Node::Define {
                name: ident.str,
                var_type: var_type.str,
            }));
        }

        if self
            .tokenizer
            .consume_kind_str(TokenKind::RESERVED, String::from("{"))
        {
            let mut stmts: Vec<Box<Node>> = vec![];
            loop {
                if self
                    .tokenizer
                    .consume_kind_str(TokenKind::RESERVED, String::from("}"))
                {
                    return Ok(Box::new(Node::Block { stmts }));
                } else {
                    stmts.push(self.stmt()?);
                }
            }
        }

        if self.tokenizer.consume_kind(TokenKind::RETURN) {
            let node = Node::Return { lhs: self.expr()? };
            self.tokenizer.expect(String::from(";"))?;
            return Ok(Box::new(node));
        }

        if self.tokenizer.current_token().kind == TokenKind::CONTROL {
            match &*self.tokenizer.current_token().str {
                "if" => {
                    self.tokenizer.cursor += 1;
                    self.tokenizer.expect(String::from("("))?;
                    let condition = self.expr()?;
                    self.tokenizer.expect(String::from(")"))?;
                    let stmt = self.stmt()?;
                    let mut else_stmt: Option<Box<Node>> = None;
                    if self
                        .tokenizer
                        .consume_kind_str(TokenKind::CONTROL, String::from("else"))
                    {
                        else_stmt = Some(self.stmt()?);
                    }
                    return Ok(Box::new(Node::If {
                        condition,
                        stmt,
                        else_stmt,
                    }));
                }
                "while" => {
                    self.tokenizer.cursor += 1;
                    self.tokenizer.expect(String::from("("))?;
                    let condition = self.expr()?;
                    self.tokenizer.expect(String::from(")"))?;
                    let stmt = self.stmt()?;
                    return Ok(Box::new(Node::While { condition, stmt }));
                }
                "for" => {
                    self.tokenizer.cursor += 1;
                    self.tokenizer.expect(String::from("("))?;
                    let init: Option<Box<Node>> =
                        if self.tokenizer.current_token().str != String::from(";") {
                            Some(self.expr()?)
                        } else {
                            None
                        };
                    self.tokenizer.expect(String::from(";"))?;

                    let condition: Option<Box<Node>> =
                        if self.tokenizer.current_token().str != String::from(";") {
                            Some(self.expr()?)
                        } else {
                            None
                        };
                    self.tokenizer.expect(String::from(";"))?;

                    let update: Option<Box<Node>> =
                        if self.tokenizer.current_token().str != String::from(")") {
                            Some(self.expr()?)
                        } else {
                            None
                        };
                    self.tokenizer.expect(String::from(")"))?;

                    let stmt = self.stmt()?;

                    return Ok(Box::new(Node::For {
                        init,
                        condition,
                        update,
                        stmt,
                    }));
                }
                _ => {}
            }
        }

        let node = self.expr();
        self.tokenizer.expect(String::from(";"))?;
        return node;
    }

    pub fn expr(&mut self) -> Result<Box<Node>, String> {
        Ok(self.assign()?)
    }

    pub fn assign(&mut self) -> Result<Box<Node>, String> {
        let mut node = self.equality()?;

        if self.tokenizer.consume(String::from("=")) {
            node = Node::new_node(NodeKind::ASSIGN, node, self.assign()?);
        }

        Ok(node)
    }

    pub fn equality(&mut self) -> Result<Box<Node>, String> {
        let mut node = self.relational()?;

        loop {
            if self.tokenizer.consume(String::from("==")) {
                node = Node::new_node(NodeKind::EQ, node, self.relational()?);
            } else if self.tokenizer.consume(String::from("!=")) {
                node = Node::new_node(NodeKind::NE, node, self.relational()?);
            } else {
                return Ok(node);
            }
        }
    }

    pub fn relational(&mut self) -> Result<Box<Node>, String> {
        let mut node = self.add()?;

        loop {
            if self.tokenizer.consume(String::from("<=")) {
                node = Node::new_node(NodeKind::LE, node, self.add()?);
            } else if self.tokenizer.consume(String::from("<")) {
                node = Node::new_node(NodeKind::LT, node, self.add()?);
            } else if self.tokenizer.consume(String::from(">=")) {
                node = Node::new_node(NodeKind::LE, self.add()?, node);
            } else if self.tokenizer.consume(String::from(">")) {
                node = Node::new_node(NodeKind::LT, self.add()?, node);
            } else {
                return Ok(node);
            }
        }
    }

    pub fn add(&mut self) -> Result<Box<Node>, String> {
        let mut node = self.mul()?;

        loop {
            if self.tokenizer.consume(String::from("+")) {
                node = Node::new_node(NodeKind::ADD, node, self.mul()?);
            } else if self.tokenizer.consume(String::from("-")) {
                node = Node::new_node(NodeKind::SUB, node, self.mul()?);
            } else if self.tokenizer.consume(String::from("+=")) {
                node = Node::new_node(
                    NodeKind::ASSIGN,
                    Box::new((*node).clone()),
                    Node::new_node(NodeKind::ADD, node, self.mul()?),
                );
            } else if self.tokenizer.consume(String::from("-=")) {
                node = Node::new_node(
                    NodeKind::ASSIGN,
                    Box::new((*node).clone()),
                    Node::new_node(NodeKind::SUB, node, self.mul()?),
                );
            } else {
                return Ok(node);
            }
        }
    }

    pub fn mul(&mut self) -> Result<Box<Node>, String> {
        let mut node = self.unary()?;
        loop {
            if self.tokenizer.consume(String::from("*")) {
                node = Node::new_node(NodeKind::MUL, node, self.unary()?);
            } else if self.tokenizer.consume(String::from("/")) {
                node = Node::new_node(NodeKind::DIV, node, self.unary()?);
            } else if self.tokenizer.consume(String::from("*=")) {
                node = Node::new_node(
                    NodeKind::ASSIGN,
                    Box::new((*node).clone()),
                    Node::new_node(NodeKind::MUL, node, self.unary()?),
                );
            } else if self.tokenizer.consume(String::from("/=")) {
                node = Node::new_node(
                    NodeKind::ASSIGN,
                    Box::new((*node).clone()),
                    Node::new_node(NodeKind::DIV, node, self.unary()?),
                );
            } else {
                return Ok(node);
            }
        }
    }

    pub fn primary(&mut self) -> Result<Box<Node>, String> {
        if self.tokenizer.consume(String::from("(")) {
            let node = self.expr()?;
            self.tokenizer.expect(String::from(")"))?;
            return Ok(node);
        }

        if self.tokenizer.current_token().kind == TokenKind::IDENT {
            let node = self.tokenizer.expect_ident()?;
            if self.tokenizer.consume(String::from("(")) {
                let mut args: Vec<Box<Node>> = vec![];
                while self.tokenizer.current_token().str != ")" {
                    args.push(self.unary()?);
                    self.tokenizer.consume(String::from(","));
                }
                
                self.tokenizer.expect(String::from(")"))?;
                return Ok(Box::new(Node::Call {
                  name: node.clone(),
                  args: args,
                }))
            }
            return Ok(Node::new_lvar_node(node.clone()));
        }

        if self.tokenizer.current_token().kind == TokenKind::TEXT {
            let text = self.tokenizer.current_token().str.clone();
            self.tokenizer.consume_kind(TokenKind::TEXT);
            return Ok(Box::new(Node::Text {
                value: text,
            }));
        }

        return Ok(Node::new_num_node(self.tokenizer.expect_number()?));
    }

    pub fn unary(&mut self) -> Result<Box<Node>, String> {
        if self.tokenizer.consume(String::from("+")) {
            return Ok(self.primary()?);
        }
        if self.tokenizer.consume(String::from("-")) {
            return Ok(Node::new_node(
                NodeKind::SUB,
                Node::new_num_node(0),
                self.primary()?,
            ));
        }
        return Ok(self.primary()?);
    }
}
