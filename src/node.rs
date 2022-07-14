use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    ASSIGN,
    ADD,
    SUB,
    MUL,
    DIV,
    EQ, // ==
    NE, // !=
    LT, // <
    LE, // <=
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Function {
        name: String,
        args: HashMap<String, String>,
        body: Vec<Box<Node>>
    },
    Mode {
        mode: String
    },
    Permission {
        accept: Vec<String>,
        reject: Vec<String>
    },
    Operator {
        kind: NodeKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Number {
        value: usize,
    },
    Text {
        value: String,
    },
    Lvar {
        value: String,
    },
    Return {
        lhs: Box<Node>,
    },
    If {
        condition: Box<Node>,
        stmt: Box<Node>,
        else_stmt: Option<Box<Node>>,
    },
    While {
        condition: Box<Node>,
        stmt: Box<Node>,
    },
    For {
        init: Option<Box<Node>>,
        condition: Option<Box<Node>>,
        update: Option<Box<Node>>,
        stmt: Box<Node>,
    },
    Block {
        stmts: Vec<Box<Node>>,
        permission: Option<Box<Node>>,
        mode: Option<Box<Node>>,
    },
    Define {
        name: String,
        var_type: String,
    },
    Call {
        name: String,
        args: Vec<Box<Node>>,
    },
    None
}

impl Node {
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        Box::new(Node::Operator { kind, lhs, rhs })
    }

    pub fn new_num_node(value: usize) -> Box<Node> {
        Box::new(Node::Number { value })
    }

    pub fn new_lvar_node(value: String) -> Box<Node> {
        Box::new(Node::Lvar { value })
    }
}
