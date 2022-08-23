use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    args::Args,
    config::Config,
    elliptic_curve::encryption::Encryption,
    gpsl::{
        self,
        external_function::STD_FUNC,
        node::{Node, NodeKind},
        source::Source,
        tokenizer::Tokenizer,
        vm::gpsl::GPSL,
    },
};

pub fn start_client(args: Args) {
    let mut source =
        Source::new(fs::read_to_string(&(args.file.unwrap())).expect("Cannot read file."));

    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(&mut source).unwrap();

    let mut parser = gpsl::parser::Parser {
        tokenizer,
        local_vars: HashMap::new(),
    };

    let functions = parser.functions().unwrap();

    let mut server_functions: HashMap<String, HashMap<String, Box<Node>>> = HashMap::new();
    for function in functions.clone() {
        let function_node = function.clone();
        let function = function.clone().1.expect_function();
        if let Err(_) = function {
            continue;
        }
        let function = function.unwrap();
        if let None = function.4 {
            continue;
        }

        let attribute = function.4.unwrap().expect_attribute();
        if let Err(_) = attribute {
            continue;
        }

        let attribute = attribute.unwrap();
        let name = attribute.0;
        let args = attribute.1;

        if name == String::from("server") {
            let ip = {
                let mut t_ip = None;
                for arg in args {
                    let mut ip = None;
                    if let Ok((kind, lhs, rhs)) = arg.expect_operator() {
                        if kind == NodeKind::ASSIGN {
                            if lhs.extract_string() == String::from("ip") {
                                ip = Some(rhs.extract_string());
                            }
                        }
                    }

                    if ip.is_some() {
                        t_ip = ip;
                        break;
                    }
                }
                t_ip.unwrap()
            };

            let t_functions = server_functions.entry(ip).or_insert(HashMap::new());
            t_functions.insert(function_node.clone().0.clone(), function_node.1);
        }
    }

    let mut servers: HashMap<String, Arc<Mutex<TcpStream>>> = HashMap::new();
    for (ip, functions) in server_functions.clone() {
        let mut stream = TcpStream::connect(ip.clone()).unwrap();
        stream
            .write_fmt(format_args!(
                "{}\n",
                serde_json::to_string(&functions).unwrap()
            ))
            .unwrap();
        let mut buf = String::default();
        BufReader::new(stream.try_clone().unwrap())
            .read_line(&mut buf)
            .unwrap();
        if buf != String::from("received\n") {
            panic!("Server did not receive functions");
        }
        servers.insert(ip, Arc::new(Mutex::new(stream)));
    }

    let encryption = Encryption::secp256k1();

    let config = Config::read_or_create();

    let mut gpsl = GPSL::new(
        source,
        Some(functions),
        Some(server_functions),
        Some(servers),
        encryption.clone(),
        config.private_key,
        config.public_key,
        vec![STD_FUNC],
    );
    let res = gpsl.run("main".to_string(), HashMap::new());
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
}
