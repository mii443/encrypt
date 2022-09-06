use primitive_types::U512;
use serde::{Deserialize, Serialize};
use std::{io::{stdout, Write, Read}, fs};

use crate::{
    elliptic_curve::{elliptic_curve::EllipticCurvePoint, encryption::Encryption},
    gpsl::{gpsl_type::GPSLType, permission::Permission, variable::Variable},
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
    pub private_key2: Option<U512>,
    pub public_key: Option<EllipticCurvePoint>,
    pub public_key2: Option<EllipticCurvePoint>,
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
        "read_value" => {
            let file_name = args[0].clone();
            let mut file = fs::File::open(file_name.extract_text().unwrap()).unwrap();
            let mut contents = String::default();
            file.read_to_string(&mut contents).unwrap();
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(serde_json::from_str(&contents).unwrap()),
            }
        }
        "write_value" => {
            if !accept.contains(&Permission::FileWrite) || reject.contains(&Permission::FileWrite) {
                return ExternalFuncReturn {
                    status: ExternalFuncStatus::REJECTED,
                    value: None,
                };
            }
            let file_name = args[0].clone();
            let content = args[1].clone();
            let mut file =
                std::fs::File::create(file_name.extract_text().unwrap().as_str()).unwrap();
            file.write_all(serde_json::to_string(&content).unwrap().as_bytes())
                .unwrap();
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: None,
            }
        }
        "write" => {
            if !accept.contains(&Permission::FileWrite) || reject.contains(&Permission::FileWrite) {
                return ExternalFuncReturn {
                    status: ExternalFuncStatus::REJECTED,
                    value: None,
                };
            }
            let file_name = args[0].clone();
            let content = args[1].clone();
            let mut file =
                std::fs::File::create(file_name.extract_text().unwrap().as_str()).unwrap();
            file.write_all(content.extract_text().unwrap().as_bytes())
                .unwrap();
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: None,
            }
        }
        "length" => {
            let vec = args[0].clone();
            match vec {
                Variable::Vec { value, .. } => ExternalFuncReturn {
                    status: ExternalFuncStatus::SUCCESS,
                    value: Some(Variable::Number {
                        value: value.len() as i64,
                    }),
                },
                _ => ExternalFuncReturn {
                    status: ExternalFuncStatus::ERROR,
                    value: None,
                },
            }
        }
        "push" => {
            let mut args = args;
            let vec = args[0].clone();
            match vec {
                Variable::Vec {
                    mut value,
                    gpsl_type,
                } => {
                    args.remove(0);
                    for arg in args {
                        value.push(arg);
                    }
                    return ExternalFuncReturn {
                        status: ExternalFuncStatus::SUCCESS,
                        value: Some(Variable::Vec { value, gpsl_type }),
                    };
                }
                _ => {
                    println!("push: argument is not a vector");
                    return ExternalFuncReturn {
                        status: ExternalFuncStatus::ERROR,
                        value: None,
                    };
                }
            }
        }
        "vec" => {
            let mut vec = Vec::new();
            let typ = GPSLType::from_str(&args[0].get_type()).unwrap();
            for arg in args {
                if arg.get_type() == typ.to_str() {
                    vec.push(arg);
                } else {
                    println!("{} != {}", arg.get_type(), typ.to_str());
                    return ExternalFuncReturn {
                        status: ExternalFuncStatus::ERROR,
                        value: None,
                    };
                }
            }
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(Variable::Vec {
                    value: vec,
                    gpsl_type: GPSLType {
                        type_str: "Vec".to_string(),
                        child: vec![typ],
                    },
                }),
            }
        }
        "encrypt" => {
            let encryption = data.encryption;
            let plain = match args[0] {
                Variable::Number { value } => U512::from(value),
                Variable::U512 { value } => value,
                _ => panic!("encrypt: first argument must be a number"),
            };
            let ec = encryption.plain_to_ec_point(plain);
            let eep = encryption.encrypt(ec, data.public_key.unwrap(), None);
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(Variable::PureEncrypted { value: eep }),
            }
        }
        "encrypt2" => {
            let encryption = data.encryption;
            let plain = match args[0] {
                Variable::Number { value } => U512::from(value),
                Variable::U512 { value } => value,
                _ => panic!("encrypt: first argument must be a number"),
            };
            let ec = encryption.plain_to_ec_point_sub(plain);
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
        "decrypt_pair" => {
            let encryption = data.encryption;
            let (a, b, c, d) = match args[0] {
                Variable::PairedEncrypted { a, b, c, d } => (a, b, c, d),
                _ => panic!("decrypt: first argument must be a pure encrypted point"),
            };

            let plain = encryption.decrypt_pair(
                a,
                b,
                c,
                d,
                data.private_key.unwrap(),
                data.private_key2.unwrap(),
            );

            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(Variable::U512 { value: plain }),
            }
        }
        "to_num" => {
            let num = match args[0].clone() {
                Variable::Number { value } => value,
                Variable::U512 { value } => value.as_u64() as i64,
                Variable::Text { value } => value.as_str().parse::<i64>().unwrap(),
                _ => panic!("to_num: first argument must be a number"),
            };
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(Variable::Number { value: num }),
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
                Variable::Text { value } => {
                    return ExternalFuncReturn {
                        status: ExternalFuncStatus::SUCCESS,
                        value: Some(Variable::U512 {
                            value: U512::from_dec_str(&value).unwrap(),
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
        "read_line" => {
            if accept.contains(&Permission::StdIn) && !reject.contains(&Permission::StdIn) {
                let mut buffer = String::default();
                std::io::stdin().read_line(&mut buffer).unwrap();
                return ExternalFuncReturn {
                    status: ExternalFuncStatus::SUCCESS,
                    value: Some(Variable::Text {
                        value: String::from(buffer.trim()),
                    }),
                };
            } else {
                return ExternalFuncReturn {
                    status: ExternalFuncStatus::REJECTED,
                    value: None,
                };
            }
        }
        "println" => {
            if accept.contains(&Permission::StdOut) && !reject.contains(&Permission::StdOut) {
                match &args[0] {
                    Variable::Text { value } => println!("{}", value),
                    Variable::Number { value } => println!("{}", value),
                    Variable::U512 { value } => println!("{:x}", value),
                    Variable::PureEncrypted { value } => println!("{}", value),
                    Variable::PairedEncrypted { a, b, c, d } => {
                        println!("{:x}{:x}{:x}{:x}", a.value, b.value, c.value, d.value)
                    }
                    Variable::Vec { value, gpsl_type } => {
                        STD_FUNC(
                            "print".to_string(),
                            vec![Variable::Vec {
                                value: value.clone(),
                                gpsl_type: gpsl_type.clone(),
                            }],
                            accept.clone(),
                            reject.clone(),
                            ExternalFunctionCallData {
                                encryption: data.encryption.clone(),
                                private_key: data.private_key.clone(),
                                private_key2: data.private_key2.clone(),
                                public_key: data.public_key.clone(),
                                public_key2: data.public_key2.clone(),
                            },
                        );
                        println!("");
                    }
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
            if accept.contains(&Permission::StdOut) && !reject.contains(&Permission::StdOut) {
                match &args[0] {
                    Variable::Text { value } => print!("{}", value),
                    Variable::Number { value } => print!("{}", value),
                    Variable::U512 { value } => print!("{:x}", value),
                    Variable::PureEncrypted { value } => print!("{}", value),
                    Variable::PairedEncrypted { a, b, c, d } => {
                        print!("{:x}{:x}{:x}{:x}", a.value, b.value, c.value, d.value)
                    }
                    Variable::Vec { value, .. } => {
                        print!("[");
                        let mut f = false;
                        for val in value {
                            if f {
                                print!(", ");
                            } else {
                                f = true;
                            }
                            STD_FUNC(
                                "print".to_string(),
                                vec![val.clone()],
                                accept.clone(),
                                reject.clone(),
                                ExternalFunctionCallData {
                                    encryption: data.encryption.clone(),
                                    private_key: data.private_key,
                                    private_key2: data.private_key2,
                                    public_key: data.public_key,
                                    public_key2: data.public_key2,
                                },
                            );
                        }
                        print!("]");
                    }
                    _ => {}
                }

                stdout().flush().unwrap();
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
