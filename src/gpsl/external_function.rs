use serde::Deserializer;

use crate::gpsl::{permission::Permission, variable::Variable};
use std::{io::Read, net::TcpStream};

#[derive(PartialEq)]
pub enum ExternalFuncStatus {
    SUCCESS,
    NOTFOUND,
    ERROR,
    REJECTED,
}

pub struct ExternalFuncReturn {
    pub status: ExternalFuncStatus,
    pub value: Option<Variable>,
}

pub struct ExternalFuncCallData {
    pub stream: Option<TcpStream>,
}

#[allow(dead_code)]
pub const STD_FUNC: fn(
    String,
    Vec<Variable>,
    Vec<Permission>,
    Vec<Permission>,
    Option<ExternalFuncCallData>,
) -> ExternalFuncReturn = |name, args, accept, reject, data| {
    let name = name.as_str();
    match name {
        "println" => {
            if accept.contains(&Permission::StdIo) && !reject.contains(&Permission::StdIo) {
                match &args[0] {
                    Variable::Text { value } => println!("{}", value),
                    Variable::Number { value } => println!("{}", value),
                    _ => {}
                }
                ExternalFuncReturn {
                    status: ExternalFuncStatus::SUCCESS,
                    value: None,
                }
            } else {
                ExternalFuncReturn {
                    status: ExternalFuncStatus::REJECTED,
                    value: None,
                }
            }
        }
        "print" => {
            if accept.contains(&Permission::StdIo) && !reject.contains(&Permission::StdIo) {
                match &args[0] {
                    Variable::Text { value } => print!("{}", value),
                    Variable::Number { value } => print!("{}", value),
                    _ => {}
                }
                ExternalFuncReturn {
                    status: ExternalFuncStatus::SUCCESS,
                    value: None,
                }
            } else {
                ExternalFuncReturn {
                    status: ExternalFuncStatus::REJECTED,
                    value: None,
                }
            }
        }
        "receive" => {
            let mut buffer = String::default();
            data.unwrap()
                .stream
                .unwrap()
                .read_to_string(&mut buffer)
                .unwrap();
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(serde_json::from_str(&buffer).unwrap()),
            }
        }
        _ => ExternalFuncReturn {
            status: ExternalFuncStatus::NOTFOUND,
            value: None,
        },
    }
};
