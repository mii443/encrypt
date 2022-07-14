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
    pub fn functions(&mut self) -> Result<HashMap<String, Box<Node>>, String> {
        let mut nodes: HashMap<String, Box<Node>> = HashMap::new();
        loop {
            if self.tokenizer.current_token().kind != TokenKind::EOF {
                let function = self.function()?;
                if let Node::Function{name, .. } = *function.clone() {
                    nodes.insert(name, function);
                }
            } else {
                return Ok(nodes);
            }
        }
    }

    /*
        function: FN IDENT LPAREN (IDENT COLON IDENT COMMA?)* RPAREN (ARROW IDENT)? block ;
    */
    pub fn function(&mut self) -> Result<Box<Node>, String> {
        if self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from("fn")) {
            debug!("parsing function");
            let func_name = self.tokenizer.current_token().clone();
            self.tokenizer.expect_kind(TokenKind::IDENT)?;
            let mut args = HashMap::new();
            self.tokenizer.expect(String::from("("))?;
            debug!("parsing args");
            while !self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from(")")) {
                debug!("consume argument");
                let name = self.tokenizer.expect_ident()?;
                self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from(":"));
                let type_str = self.tokenizer.expect_ident()?;
                self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from(","));
                args.insert(name, type_str);
            }

            let mut nodes: Vec<Box<Node>> = vec![];
            debug!("parsing body node");
            loop {
                nodes.push(self.stmt()?);
                debug!("body nodes parsed");
                //self.tokenizer.expect(String::from("}"))?;
                return Ok(Box::new(Node::Function {
                    name: func_name.str,
                    args,
                    body: nodes
                }));
            }
        } else {
            println!("{:?}", self.tokenizer.current_token());
            Err(String::from("Unexpected token."))
        }
    }

    /*
        program: stmt* ;
    */
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

    /*
        stmt: let
            | block
            | return
            | if
            | while
            | for
            | expr SEMICOLON
            ;
    */
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

        debug!("parsing permission");
        let permission = if self.tokenizer.current_token().str == "$" {
            Some(self.permission()?)
        } else {
            None
        };

        if self
            .tokenizer
            .consume_kind_str(TokenKind::RESERVED, String::from("{")) || permission != None
        {
            let mut stmts: Vec<Box<Node>> = vec![];
            loop {
                if self
                    .tokenizer
                    .consume_kind_str(TokenKind::RESERVED, String::from("}"))
                {
                    return Ok(Box::new(Node::Block { stmts, permission: permission }));
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

    /*
        permission: DOLLER LPAREN ( IDENT LBRACKET ( IDENT COMMA? )* RBRACKET COMMA? )* RPAREN ;
    */
    pub fn permission(&mut self) -> Result<Box<Node>, String> {
        self.tokenizer.expect(String::from("$"))?;
        self.tokenizer.expect(String::from("("))?;

        let mut accept: Vec<String> = vec![];
        let mut reject: Vec<String> = vec![];

        while !self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from(")")) {
            let name = self.tokenizer.expect_ident()?;
            if name != "accept" && name != "reject" {
                return Err(String::from(format!("Unexpected: {}", name)));
            }
            self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from("["));
            while !self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from("]")) {
                let permission = self.tokenizer.expect_ident()?;
                self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from(","));

                if name == "accept" {
                    accept.push(permission);
                } else if name == "reject" {
                    reject.push(permission);
                }
            }

            self.tokenizer.consume_kind_str(TokenKind::RESERVED, String::from(","));
        }

        Ok(Box::new(Node::Permission { accept, reject }))
    }

    /*
        expr: assign ;
    */
    pub fn expr(&mut self) -> Result<Box<Node>, String> {
        Ok(self.assign()?)
    }

    /*
        assign: equality (EQ assign)? ;
    */
    pub fn assign(&mut self) -> Result<Box<Node>, String> {
        let mut node = self.equality()?;

        if self.tokenizer.consume(String::from("=")) {
            node = Node::new_node(NodeKind::ASSIGN, node, self.assign()?);
        }

        Ok(node)
    }

    /*
        equality: relational (EQEQ relational | NE relational | CONJ)* ;
    */
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

    /*
        relational: add (LE add | LT add | BE add | BT add)* ;
    */
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

    /*
        add: mul (ADD mul | SUB mul | SUB_ASSIGNMENT mul | ADD_ASSIGNMENT mul)* ;
    */
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

    /*
        mul: unary (MUL unary | DIV unary | DIV_ASSIGNMENT unary | MUL_ASSIGNMENT unary)* ;
    */
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

    /*
        primary: LPAREN expr RPAREN | function_call | TEXT | NUM ;
    */
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

    /*
        unary: ADD primary
            | SUB primary
            | primary
            ;
    */
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
