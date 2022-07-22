use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Node {
    Function {
        name: String,
        args_name: Vec<String>,
        args_type: Vec<String>,
        body: Vec<Box<Node>>,
        attribute: Option<Box<Node>>,
    },
    Attribute {
        name: String,
        args: Vec<Box<Node>>,
    },
    Mode {
        mode: String,
    },
    Permission {
        accept: Vec<String>,
        reject: Vec<String>,
    },
    Operator {
        kind: NodeKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Number {
        value: i64,
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
    None,
}

impl Node {
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        Box::new(Node::Operator { kind, lhs, rhs })
    }

    pub fn new_num_node(value: i64) -> Box<Node> {
        Box::new(Node::Number { value })
    }

    pub fn new_lvar_node(value: String) -> Box<Node> {
        Box::new(Node::Lvar { value })
    }

    pub fn extract_string(&self) -> String {
        match self {
            Node::Text { value } => value.clone(),
            Node::Number { value } => value.to_string(),
            Node::Lvar { value } => value.clone(),
            _ => String::new(),
        }
    }

    pub fn extract_function_args(&self) -> (Vec<String>, Vec<String>) {
        match self {
            Node::Function {
                args_name,
                args_type,
                ..
            } => (args_name.clone(), args_type.clone()),
            _ => (Vec::new(), Vec::new()),
        }
    }
}
