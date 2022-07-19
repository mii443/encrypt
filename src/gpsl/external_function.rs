use serde::Deserializer;

use crate::gpsl::{permission::Permission, variable::Variable};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

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
    pub stream: Arc<Mutex<Option<TcpStream>>>,
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
            println!("Waiting for client...");
            let mut buffer = String::default();
            let data = data.unwrap();
            let mut stream = data.stream.lock().unwrap();

            let stream = match &mut *stream {
                Some(stream) => stream,
                None => panic!("Cannot access to tcp stream"),
            };
            let mut reader = BufReader::new(stream);
            reader.read_line(&mut buffer).unwrap();
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(serde_json::from_str(&buffer).unwrap()),
            }
        }
        "send" => {
            let data = data.unwrap();
            let mut stream = data.stream.lock().unwrap();

            let stream = match &mut *stream {
                Some(stream) => stream,
                None => panic!("Cannot access to tcp stream"),
            };

            let value = serde_json::to_string(&args[0]).unwrap();

            stream.write_fmt(format_args!("{}\n", value)).unwrap();

            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: None,
            }
        }
        _ => ExternalFuncReturn {
            status: ExternalFuncStatus::NOTFOUND,
            value: None,
        },
    }
};
