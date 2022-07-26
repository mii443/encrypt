use crate::elliptic_curve::elliptic_curve::EllipticCurvePoint;
use crate::elliptic_curve::encryption::{EncryptedEllipticCurvePoint, Encryption};
use crate::gpsl::external_function::{
    ExternalFuncReturn, ExternalFuncStatus, ExternalFunctionCallData,
};
use crate::gpsl::gpsl_type::GPSLType;
use crate::gpsl::node::*;
use crate::gpsl::permission::Permission;
use crate::gpsl::source::Source;
use crate::gpsl::variable::*;
use log::*;
use primitive_types::U512;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::string::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Block {
    pub accept: Vec<Permission>,
    pub reject: Vec<Permission>,
    pub variables: HashMap<String, LocalVariable>,
    pub is_split: bool,
}

#[derive(Debug)]
pub struct GPSL {
    pub functions: Option<HashMap<String, Box<Node>>>,
    pub server_functions: Option<HashMap<String, HashMap<String, Box<Node>>>>,
    pub servers: Option<HashMap<String, Arc<Mutex<TcpStream>>>>,
    pub encryption: Encryption,
    pub private_key: Option<U512>,
    pub public_key: Option<EllipticCurvePoint>,
    pub global_variables: Vec<Variable>,
    pub source: Source,
    pub blocks: VecDeque<Block>,
    pub external_func: Vec<
        fn(
            String,
            Vec<Variable>,
            Vec<Permission>,
            Vec<Permission>,
            ExternalFunctionCallData,
        ) -> ExternalFuncReturn,
    >,
}

#[derive(Clone, Debug)]
pub struct LocalVariable {
    pub name: String,
    pub value: Option<Variable>,
    pub status: VariableStatus,
}

#[derive(Clone, Debug)]
pub struct VariableStatus {
    pub initialized: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerFunctionCall {
    pub name: String,
    pub args: HashMap<String, Variable>,
}

impl VariableStatus {
    pub fn default() -> VariableStatus {
        VariableStatus { initialized: false }
    }
}

impl GPSL {
    pub fn new(
        source: Source,
        functions: Option<HashMap<String, Box<Node>>>,
        server_functions: Option<HashMap<String, HashMap<String, Box<Node>>>>,
        servers: Option<HashMap<String, Arc<Mutex<TcpStream>>>>,
        encryption: Encryption,
        private_key: Option<U512>,
        public_key: Option<EllipticCurvePoint>,
        external_func: Vec<
            fn(
                String,
                Vec<Variable>,
                Vec<Permission>,
                Vec<Permission>,
                ExternalFunctionCallData,
            ) -> ExternalFuncReturn,
        >,
    ) -> GPSL {
        GPSL {
            source,
            functions,
            server_functions,
            servers,
            encryption,
            private_key,
            public_key,
            global_variables: vec![],
            blocks: VecDeque::new(),
            external_func,
        }
    }

    pub fn get_local_var_mut(&mut self, name: &String) -> Option<&mut LocalVariable> {
        for x in 0..self.blocks.len() {
            if self.blocks[x].variables.contains_key(name) {
                return self.blocks[x].variables.get_mut(name);
            }

            if self.blocks[x].is_split {
                break;
            }
        }
        None
    }

    pub fn get_local_var(&mut self, name: &String) -> Option<LocalVariable> {
        for x in 0..self.blocks.len() {
            if self.blocks[x].variables.contains_key(name) {
                if let Some(var) = self.blocks[x].variables.get(name).clone() {
                    return Some(var.clone());
                } else {
                    return None;
                }
            }

            if self.blocks[x].is_split {
                break;
            }
        }
        None
    }

    pub fn extract_number(node: Variable) -> Result<i64, String> {
        match node {
            Variable::Number { value } => Ok(value),
            _ => Err(String::from("Not a number")),
        }
    }

    pub fn extract_eep(node: Variable) -> Result<EncryptedEllipticCurvePoint, String> {
        match node {
            Variable::PureEncrypted { value } => Ok(value),
            _ => Err(String::from("Not an encrypted point")),
        }
    }

    pub fn evaluate(&mut self, node: Box<Node>) -> Result<Option<Variable>, String> {
        match *node {
            Node::Call { name, args } => {
                let function_name = name;
                let f = self.external_func.clone();
                let mut args_value: Vec<Variable> = vec![];
                for arg in args {
                    if let Some(val) = self.evaluate(arg).expect("Cannot evaluate") {
                        args_value.push(val);
                    }
                }

                debug!(
                    "Searching server function: {}, ({:?})",
                    &function_name, args_value
                );

                for server in self.server_functions.clone().unwrap() {
                    for function in server.1 {
                        if function.0 == function_name {
                            let mut servers = self.servers.clone().unwrap();
                            let stream = servers.get_mut(&server.0).unwrap();
                            let mut stream = stream.lock().unwrap();

                            let function_args = function.1.extract_function_args();
                            let mut args: HashMap<String, Variable> = HashMap::new();
                            for (i, arg_name) in function_args.0.iter().enumerate() {
                                if function_args.1[i].to_str() == args_value[i].get_type() {
                                    args.insert(arg_name.clone(), args_value[i].clone());
                                } else {
                                    return Err(format!(
                                        "Argument {} type mismatch: {} != {}",
                                        arg_name,
                                        function_args.1[i],
                                        args_value[i].get_type()
                                    ));
                                }
                            }

                            let server_function_call = serde_json::to_string(&ServerFunctionCall {
                                name: function_name.clone(),
                                args: args.clone(),
                            })
                            .unwrap();

                            stream
                                .write_fmt(format_args!("{}\n", server_function_call))
                                .unwrap();
                            let mut buf = String::new();
                            debug!("try clone");
                            BufReader::new(stream.try_clone().unwrap())
                                .read_line(&mut buf)
                                .unwrap();
                            let res: ExternalFuncReturn = serde_json::from_str(&buf).unwrap();
                            if res.status == ExternalFuncStatus::SUCCESS {
                                return Ok(res.value);
                            }
                            if res.status == ExternalFuncStatus::REJECTED {
                                return Err("Server function rejected.".to_string());
                            }
                        }
                    }
                }

                if let Some(functions) = self.functions.clone() {
                    debug!(
                        "functions: {:?}",
                        functions
                            .iter()
                            .map(|f| format!("{},", f.0))
                            .collect::<String>()
                    );
                    debug!(
                        "{}: {}",
                        &function_name,
                        functions.contains_key(&function_name)
                    );
                    if functions.contains_key(&function_name) {
                        if let Node::Function {
                            body,
                            args_name,
                            args_type,
                            ..
                        } = &*(functions[&function_name])
                        {
                            let block = {
                                let blocks = self.blocks.clone();
                                blocks.front().unwrap().clone()
                            };

                            let mut args: HashMap<String, LocalVariable> = HashMap::new();
                            for (i, name) in args_name.iter().enumerate() {
                                if args_type[i].to_str() == args_value[i].get_type() {
                                    args.insert(
                                        name.clone(),
                                        LocalVariable {
                                            name: name.clone(),
                                            value: Some(args_value[i].clone()),
                                            status: VariableStatus::default(),
                                        },
                                    );
                                } else {
                                    return Err(format!(
                                        "Argument {} type mismatch: {} != {}",
                                        name,
                                        args_type[i],
                                        args_value[i].get_type()
                                    ));
                                }
                            }

                            self.blocks.push_front(Block {
                                accept: block.accept.clone(),
                                reject: block.reject.clone(),
                                variables: args,
                                is_split: true,
                            });

                            for program in body {
                                let res = self.evaluate(Box::new(*program.clone()));

                                if let Ok(Some(res)) = res {
                                    match res {
                                        Variable::Return { value } => {
                                            return Ok(Some(*value));
                                        }
                                        _ => {}
                                    }
                                } else if let Err(err) = res {
                                    return Err(err);
                                }

                                self.blocks.pop_front();
                            }
                        }
                        return Ok(None);
                    }
                }

                debug!("Searching external: {}, ({:?})", &function_name, args_value);

                for func in f {
                    let block = self.blocks.front().unwrap();
                    let res = func(
                        function_name.clone(),
                        args_value.clone(),
                        block.accept.clone(),
                        block.reject.clone(),
                        ExternalFunctionCallData {
                            encryption: self.encryption.clone(),
                            private_key: self.private_key,
                            public_key: self.public_key,
                        },
                    );
                    if res.status == ExternalFuncStatus::SUCCESS {
                        return Ok(res.value);
                    }
                    if res.status == ExternalFuncStatus::REJECTED {
                        return Err("External function rejected.".to_string());
                    }
                }

                Err(format!("Function not found: {}", function_name))
            }
            Node::Text { value } => Ok(Some(Variable::Text { value })),
            Node::Number { value } => Ok(Some(Variable::Number { value })),
            Node::Operator { kind, lhs, rhs } => {
                if kind == NodeKind::ASSIGN {
                    debug!("Assign: {:?}", self.blocks.front());

                    let rhs = self.evaluate(rhs);

                    if let Ok(Some(rhs)) = rhs {
                        match *(lhs.clone()) {
                            Node::Lvar { value, index } => {
                                if let Some(index) = index {
                                    let mut val =
                                        self.get_local_var_mut(&value).unwrap().value.clone();
                                    match val {
                                        Some(Variable::Vec {
                                            value: mut v,
                                            gpsl_type,
                                        }) => {
                                            v[self
                                                .evaluate(index)
                                                .unwrap()
                                                .unwrap()
                                                .extract_number()
                                                .unwrap()
                                                as usize] = rhs;
                                            self.get_local_var_mut(&value).unwrap().value =
                                                Some(Variable::Vec {
                                                    value: v,
                                                    gpsl_type,
                                                });
                                        }
                                        _ => {
                                            return Err(format!(
                                                "Variable {} is not a list",
                                                value
                                            ));
                                        }
                                    }
                                } else {
                                    self.get_local_var_mut(&value).unwrap().value = Some(rhs);
                                    self.get_local_var_mut(&value).unwrap().status.initialized =
                                        true;
                                }
                            }
                            _ => {}
                        }
                    }

                    return Ok(None);
                }
                let lhs = self.evaluate(lhs).expect("Cannot evaluate lhs.");
                let rhs = self.evaluate(rhs).expect("Cannot evaluate rhs.");

                if let Some(lhs) = lhs.clone() {
                    if let Some(rhs) = rhs {
                        match kind {
                            NodeKind::ADD => match lhs.clone() {
                                Variable::Number { value: lhs } => match rhs {
                                    Variable::Number { value: rhs } => {
                                        Ok(Some(Variable::Number { value: lhs + rhs }))
                                    }
                                    Variable::Text { value: rhs } => Ok(Some(Variable::Text {
                                        value: format!("{}{}", lhs, rhs),
                                    })),
                                    _ => Err("Cannot add non-number to number.".to_string()),
                                },
                                Variable::PureEncrypted { value: lhs } => match rhs {
                                    Variable::PureEncrypted { value: rhs } => {
                                        Ok(Some(Variable::PureEncrypted { value: lhs + rhs }))
                                    }
                                    Variable::Text { value: rhs } => Ok(Some(Variable::Text {
                                        value: format!("{}{}", lhs, rhs),
                                    })),
                                    _ => Err("Cannot add non-number to number.".to_string()),
                                },
                                Variable::U512 { value: lhs } => match rhs {
                                    Variable::U512 { value: rhs } => {
                                        Ok(Some(Variable::U512 { value: lhs + rhs }))
                                    }
                                    Variable::Text { value: rhs } => Ok(Some(Variable::Text {
                                        value: lhs.to_string() + &rhs,
                                    })),
                                    _ => Err("Cannot add non-number to number.".to_string()),
                                },
                                Variable::Text { value: lhs } => match rhs {
                                    Variable::Text { value: rhs } => Ok(Some(Variable::Text {
                                        value: format!("{}{}", lhs, rhs),
                                    })),
                                    Variable::Number { value: rhs } => Ok(Some(Variable::Text {
                                        value: format!("{}{}", lhs, rhs),
                                    })),
                                    Variable::PureEncrypted { value: rhs } => {
                                        Ok(Some(Variable::Text {
                                            value: format!("{}{}", lhs, rhs),
                                        }))
                                    }
                                    Variable::U512 { value: rhs } => Ok(Some(Variable::Text {
                                        value: format!("{}{}", lhs, rhs),
                                    })),
                                    _ => Err("Cannot add non-number to number.".to_string()),
                                },
                                _ => Err("Cannot add non-number.".to_string()),
                            },
                            NodeKind::DIV => match GPSL::extract_number(lhs) {
                                Ok(lhs) => match GPSL::extract_number(rhs) {
                                    Ok(rhs) => Ok(Some(Variable::Number { value: lhs / rhs })),
                                    Err(err) => Err(err),
                                },
                                Err(err) => Err(err),
                            },
                            NodeKind::MUL => match GPSL::extract_number(lhs) {
                                Ok(lhs) => match GPSL::extract_number(rhs) {
                                    Ok(rhs) => Ok(Some(Variable::Number { value: lhs * rhs })),
                                    Err(err) => Err(err),
                                },
                                Err(err) => Err(err),
                            },
                            NodeKind::SUB => match GPSL::extract_number(lhs) {
                                Ok(lhs) => match GPSL::extract_number(rhs) {
                                    Ok(rhs) => Ok(Some(Variable::Number { value: lhs - rhs })),
                                    Err(err) => Err(err),
                                },
                                Err(err) => Err(err),
                            },

                            NodeKind::EQ => {
                                if lhs == rhs {
                                    Ok(Some(Variable::Number { value: 1 }))
                                } else {
                                    Ok(Some(Variable::Number { value: 0 }))
                                }
                            }
                            NodeKind::NE => {
                                if lhs != rhs {
                                    Ok(Some(Variable::Number { value: 1 }))
                                } else {
                                    Ok(Some(Variable::Number { value: 0 }))
                                }
                            }
                            NodeKind::LT => match GPSL::extract_number(lhs) {
                                Ok(lhs) => match GPSL::extract_number(rhs) {
                                    Ok(rhs) => {
                                        if lhs < rhs {
                                            Ok(Some(Variable::Number { value: 1 }))
                                        } else {
                                            Ok(Some(Variable::Number { value: 0 }))
                                        }
                                    }
                                    Err(err) => Err(err),
                                },
                                Err(err) => Err(err),
                            },
                            NodeKind::LE => match GPSL::extract_number(lhs) {
                                Ok(lhs) => match GPSL::extract_number(rhs) {
                                    Ok(rhs) => {
                                        if lhs <= rhs {
                                            Ok(Some(Variable::Number { value: 1 }))
                                        } else {
                                            Ok(Some(Variable::Number { value: 0 }))
                                        }
                                    }
                                    Err(err) => Err(err),
                                },
                                Err(err) => Err(err),
                            },
                            _ => Ok(None),
                        }
                    } else {
                        Err(String::from("RHS Variable is null."))
                    }
                } else {
                    Err(String::from("LHS Variable is null."))
                }
            }
            Node::Lvar { value, index } => {
                if let Some(index) = index {
                    let val = self.get_local_var_mut(&value).unwrap().clone();
                    match val.value {
                        Some(Variable::Vec { value, gpsl_type }) => {
                            return Ok(Some(
                                value[self
                                    .evaluate(index)
                                    .unwrap()
                                    .unwrap()
                                    .extract_number()
                                    .unwrap() as usize]
                                    .clone(),
                            ));
                        }
                        _ => {
                            return Err(format!("Variable {} is not a list", value));
                        }
                    }
                }
                return Ok(Some(
                    self.get_local_var(&value).unwrap().value.clone().unwrap(),
                ));
            }
            Node::Return { lhs } => {
                if let Ok(Some(lhs)) = self.evaluate(lhs) {
                    return Ok(Some(Variable::Return {
                        value: Box::new(lhs),
                    }));
                } else {
                    return Err(String::from("Cannot evaluate LHS."));
                }
            }
            Node::If {
                condition,
                stmt,
                else_stmt,
            } => {
                if let Ok(Some(condition)) = self.evaluate(condition) {
                    if match condition {
                        Variable::Number { value } => value == 1,
                        _ => false,
                    } {
                        if let Ok(Some(res)) = self.evaluate(stmt) {
                            match res.clone() {
                                Variable::Return { .. } => {
                                    return Ok(Some(res));
                                }
                                _ => {}
                            }
                        }
                    } else {
                        match else_stmt {
                            Some(else_stmt) => {
                                if let Ok(Some(res)) = self.evaluate(else_stmt) {
                                    match res.clone() {
                                        Variable::Return { .. } => {
                                            return Ok(Some(res));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }

                return Ok(None);
            }
            Node::While { condition, stmt } => {
                let mut cond = if let Some(condition) = self.evaluate(condition.clone())? {
                    condition
                } else {
                    Variable::Number { value: 0 }
                };

                while match cond {
                    Variable::Number { value } => value == 1,
                    _ => false,
                } {
                    self.evaluate(stmt.clone())?;
                    cond = if let Some(condition) = self.evaluate(condition.clone())? {
                        condition
                    } else {
                        Variable::Number { value: 0 }
                    };
                }

                return Ok(None);
            }
            Node::For {
                init,
                condition,
                update,
                stmt,
            } => {
                match init {
                    Some(init) => {
                        self.evaluate(init)?;
                    }
                    None => {}
                }

                let mut cond = match condition.clone() {
                    Some(condition) => {
                        if let Some(condition) = self.evaluate(condition)? {
                            condition
                        } else {
                            Variable::Number { value: 0 }
                        }
                    }
                    None => Variable::Number { value: 1 },
                };

                while match cond {
                    Variable::Number { value } => value == 1,
                    _ => false,
                } {
                    self.evaluate(stmt.clone())?;

                    match update.clone() {
                        Some(update) => {
                            self.evaluate(update)?;
                        }
                        None => {}
                    }

                    cond = match condition.clone() {
                        Some(condition) => {
                            if let Some(condition) = self.evaluate(condition)? {
                                condition
                            } else {
                                Variable::Number { value: 0 }
                            }
                        }
                        None => Variable::Number { value: 1 },
                    };
                }

                return Ok(None);
            }
            Node::Block {
                stmts,
                permission,
                mode,
            } => {
                let accept = self.blocks.front().unwrap().accept.clone();
                let reject = self.blocks.front().unwrap().reject.clone();
                let (accept, reject) = if let Node::Permission { accept, reject } =
                    *permission.unwrap_or(Box::new(Node::None))
                {
                    (
                        accept.iter().map(|p| Permission::from_string(p)).collect(),
                        reject.iter().map(|p| Permission::from_string(p)).collect(),
                    )
                } else {
                    (accept, reject)
                };

                let _ = if let Node::Mode { mode } = *mode.unwrap_or(Box::new(Node::None)) {
                    mode
                } else {
                    "".to_string()
                };

                self.blocks.push_front(Block {
                    accept,
                    reject,
                    variables: HashMap::new(),
                    is_split: false,
                });

                for stmt in stmts {
                    let ret = self.evaluate(stmt)?;
                    if let Some(ret) = ret {
                        match ret.clone() {
                            Variable::Return { .. } => {
                                return Ok(Some(ret));
                            }
                            _ => {}
                        }
                    }
                }

                self.blocks.pop_front();

                return Ok(None);
            }
            Node::Define {
                name,
                var_type,
                value,
            } => {
                if let Some(value) = value {
                    let value = self.evaluate(value);
                    if let Ok(Some(value)) = value.clone() {
                        if value.get_type()
                            == var_type
                                .unwrap_or(GPSLType::from_str(&value.get_type()).unwrap())
                                .to_str()
                        {
                            self.blocks.front_mut().unwrap().variables.insert(
                                name.clone(),
                                LocalVariable {
                                    name,
                                    value: Some(value),
                                    status: VariableStatus { initialized: true },
                                },
                            );
                            return Ok(None);
                        } else {
                            return Err(String::from("Type mismatch."));
                        }
                    } else if let Err(err) = value {
                        return Err(err);
                    } else {
                        return Err(String::from("Unexpected error in DEFINE."));
                    }
                } else {
                    if !var_type.unwrap().is_correct() {
                        return Err(String::from("Invalid type."));
                    }
                    self.blocks.front_mut().unwrap().variables.insert(
                        name.clone(),
                        LocalVariable {
                            name,
                            value: None,
                            status: VariableStatus::default(),
                        },
                    );

                    debug!("Define: {:?}", self.blocks.front());

                    return Ok(None);
                }
            }
            _ => Ok(None),
        }
    }

    pub fn run(
        &mut self,
        function_name: String,
        args: HashMap<String, Variable>,
    ) -> Result<Variable, String> {
        debug!("functions: {:?}", self.functions);
        debug!("searching {}", function_name);

        let mut local_variables = HashMap::new();
        for (name, value) in args {
            local_variables.insert(
                name.clone(),
                LocalVariable {
                    name: name.clone(),
                    value: Some(value),
                    status: VariableStatus::default(),
                },
            );
        }

        self.blocks.push_front(Block {
            accept: vec![Permission::Administrator, Permission::StdIo],
            reject: vec![],
            variables: local_variables,
            is_split: true,
        });

        if let Some(functions) = self.functions.clone() {
            if let Node::Function { body, .. } = &*(functions[&function_name]) {
                for program in body {
                    let res = self.evaluate(Box::new(*program.clone()));
                    if let Ok(Some(res)) = res {
                        match res {
                            Variable::Return { value } => {
                                return Ok(*value);
                            }
                            _ => {}
                        }
                    } else if let Err(err) = res {
                        return Err(err);
                    }
                }
            }
        }

        Ok(Variable::None {})
    }
}
