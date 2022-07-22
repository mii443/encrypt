use primitive_types::U512;
use serde::{Deserialize, Serialize};

use crate::{
    elliptic_curve::{elliptic_curve::EllipticCurvePoint, encryption::Encryption},
    gpsl::{permission::Permission, variable::Variable},
};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ExternalFuncStatus {
    SUCCESS,
    NOTFOUND,
    ERROR,
    REJECTED,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFuncReturn {
    pub status: ExternalFuncStatus,
    pub value: Option<Variable>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFunctionCallData {
    pub encryption: Encryption,
    pub private_key: Option<U512>,
    pub public_key: Option<EllipticCurvePoint>,
}

#[allow(dead_code)]
pub const STD_FUNC: fn(
    String,
    Vec<Variable>,
    Vec<Permission>,
    Vec<Permission>,
    ExternalFunctionCallData,
) -> ExternalFuncReturn = |name, args, accept, reject, data| {
    let name = name.as_str();
    match name {
        "encrypt" => {
            let encryption = data.encryption;
            let plain = match args[0] {
                Variable::Number { value } => U512::from(value),
                _ => panic!("encrypt: first argument must be a number"),
            };
            let ec = encryption.plain_to_ec_point(plain);
            let eep = encryption.encrypt(ec, data.public_key.unwrap(), None);
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(Variable::PureEncrypted { value: eep }),
            }
        }
        "decrypt" => {
            let encryption = data.encryption;
            let eep = match args[0] {
                Variable::PureEncrypted { value } => value,
                _ => panic!("decrypt: first argument must be a pure encrypted point"),
            };
            let plain = Encryption::decrypt(eep, data.private_key.unwrap());
            let plain = encryption.ec_point_to_plain(plain);
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(Variable::Number {
                    value: plain.as_u64() as i64,
                }),
            }
        }
        "to_u512" => {
            if args.len() != 1 {
                return ExternalFuncReturn {
                    status: ExternalFuncStatus::ERROR,
                    value: None,
                };
            }
            let arg = args[0].clone();
            match arg {
                Variable::Number { value } => {
                    return ExternalFuncReturn {
                        status: ExternalFuncStatus::SUCCESS,
                        value: Some(Variable::U512 {
                            value: value.into(),
                        }),
                    };
                }
                _ => {
                    return ExternalFuncReturn {
                        status: ExternalFuncStatus::ERROR,
                        value: None,
                    };
                }
            }
        }
        "println" => {
            if accept.contains(&Permission::StdIo) && !reject.contains(&Permission::StdIo) {
                match &args[0] {
                    Variable::Text { value } => println!("{}", value),
                    Variable::Number { value } => println!("{}", value),
                    Variable::U512 { value } => println!("{:x}", value),
                    Variable::PureEncrypted { value } => println!("{}", value),
                    Variable::PairedEncrypted { value } => println!("{:x}", value.value),
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
                    Variable::U512 { value } => print!("{:x}", value),
                    Variable::PureEncrypted { value } => print!("{}", value),
                    Variable::PairedEncrypted { value } => print!("{:x}", value.value),
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
        } /*
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
        }*/
        _ => ExternalFuncReturn {
            status: ExternalFuncStatus::NOTFOUND,
            value: None,
        },
    }
};
