use serde::{Deserialize, Serialize};

use super::gpsl_type::GPSLType;

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
        args_type: Vec<GPSLType>,
        body: Vec<Box<Node>>,
        attribute: Option<Box<Node>>,
    },
    GPSLType {
        value: GPSLType,
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
        index: Option<Box<Node>>,
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
        var_type: Option<GPSLType>,
        value: Option<Box<Node>>,
    },
    Call {
        name: String,
        args: Vec<Box<Node>>,
    },
    None,
}

impl Node {
    pub fn expect_function(
        self,
    ) -> Result<
        (
            String,
            Vec<String>,
            Vec<GPSLType>,
            Vec<Box<Node>>,
            Option<Box<Node>>,
        ),
        String,
    > {
        match self {
            Node::Function {
                name,
                args_name,
                args_type,
                body,
                attribute,
            } => Ok((name, args_name, args_type, body, attribute)),
            _ => Err("Expected function".to_string()),
        }
    }

    pub fn expect_gpsltype(self) -> Result<GPSLType, String> {
        match self {
            Node::GPSLType { value } => Ok(value),
            _ => Err("Expected GPSLType".to_string()),
        }
    }

    pub fn expect_attribute(self) -> Result<(String, Vec<Box<Node>>), String> {
        match self {
            Node::Attribute { name, args } => Ok((name, args)),
            _ => Err("Expected Attribute".to_string()),
        }
    }

    pub fn expect_mode(self) -> Result<String, String> {
        match self {
            Node::Mode { mode } => Ok(mode),
            _ => Err("Expected Mode".to_string()),
        }
    }

    pub fn expect_permission(self) -> Result<(Vec<String>, Vec<String>), String> {
        match self {
            Node::Permission { accept, reject } => Ok((accept, reject)),
            _ => Err("Expected Permission".to_string()),
        }
    }

    pub fn expect_operator(self) -> Result<(NodeKind, Box<Node>, Box<Node>), String> {
        match self {
            Node::Operator { kind, lhs, rhs } => Ok((kind, lhs, rhs)),
            _ => Err("Expected Operator".to_string()),
        }
    }

    pub fn expect_number(self) -> Result<i64, String> {
        match self {
            Node::Number { value } => Ok(value),
            _ => Err("Expected Number".to_string()),
        }
    }

    pub fn expect_text(self) -> Result<String, String> {
        match self {
            Node::Text { value } => Ok(value),
            _ => Err("Expected Text".to_string()),
        }
    }

    pub fn expect_lvar(self) -> Result<(String, Option<Box<Node>>), String> {
        match self {
            Node::Lvar { value, index } => Ok((value, index)),
            _ => Err("Expected LVar".to_string()),
        }
    }

    pub fn expect_return(self) -> Result<Box<Node>, String> {
        match self {
            Node::Return { lhs } => Ok(lhs),
            _ => Err("Expected Return".to_string()),
        }
    }

    pub fn expect_if(self) -> Result<(Box<Node>, Box<Node>, Option<Box<Node>>), String> {
        match self {
            Node::If {
                condition,
                stmt,
                else_stmt,
            } => Ok((condition, stmt, else_stmt)),
            _ => Err("Expected If".to_string()),
        }
    }

    pub fn expect_while(self) -> Result<(Box<Node>, Box<Node>), String> {
        match self {
            Node::While { condition, stmt } => Ok((condition, stmt)),
            _ => Err("Expected While".to_string()),
        }
    }

    pub fn expect_for(
        self,
    ) -> Result<
        (
            Option<Box<Node>>,
            Option<Box<Node>>,
            Option<Box<Node>>,
            Box<Node>,
        ),
        String,
    > {
        match self {
            Node::For {
                init,
                condition,
                update,
                stmt,
            } => Ok((init, condition, update, stmt)),
            _ => Err("Expected For".to_string()),
        }
    }

    pub fn expect_block(
        self,
    ) -> Result<(Vec<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>), String> {
        match self {
            Node::Block {
                stmts,
                permission,
                mode,
            } => Ok((stmts, permission, mode)),
            _ => Err("Expected Block".to_string()),
        }
    }

    pub fn expect_define(self) -> Result<(String, Option<GPSLType>, Option<Box<Node>>), String> {
        match self {
            Node::Define {
                name,
                var_type,
                value,
            } => Ok((name, var_type, value)),
            _ => Err("Expected Define".to_string()),
        }
    }

    pub fn expect_call(self) -> Result<(String, Vec<Box<Node>>), String> {
        match self {
            Node::Call { name, args } => Ok((name, args)),
            _ => Err("Expected Call".to_string()),
        }
    }
}

impl Node {
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        Box::new(Node::Operator { kind, lhs, rhs })
    }

    pub fn new_num_node(value: i64) -> Box<Node> {
        Box::new(Node::Number { value })
    }

    pub fn new_lvar_node(value: String) -> Box<Node> {
        Box::new(Node::Lvar { value, index: None })
    }

    pub fn extract_string(&self) -> String {
        match self {
            Node::Text { value } => value.clone(),
            Node::Number { value } => value.to_string(),
            Node::Lvar { value, .. } => value.clone(),
            _ => String::new(),
        }
    }

    pub fn extract_function_args(&self) -> (Vec<String>, Vec<GPSLType>) {
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
