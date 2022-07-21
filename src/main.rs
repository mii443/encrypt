mod gpsl;
use gpsl::external_function::ExternalFuncReturn;
use gpsl::external_function::ExternalFuncStatus;
use gpsl::node::Node;
use gpsl::node::NodeKind;
use gpsl::variable::Variable;
use gpsl::vm::gpsl::ServerFunctionCall;
use gpsl::{external_function::STD_FUNC, parser::*, source::*, tokenizer::*, vm::gpsl::*};
use log::*;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fs};
/*
[6139062701328441600,
[258929920560, 23709360],
[[Mod(3308825380872319861, 6139062703770505681), Mod(4839630718792142583, 6139062703770505681)],
[Mod(4767914906170010398, 6139062703770505681), Mod(2445476831433994309, 6139062703770505681)]]]
 */
/*
fn main() {
    let p = U512::from_str_radix("6717051393902806321", 10).unwrap();

    let secp256_k1_a = FiniteFieldElement::new(U512::from(0u8), p);
    let secp256_k1_b = FiniteFieldElement::new(U512::from_str_radix("1603830326921046894", 10).unwrap(), p);

    let P = {
        let x = FiniteFieldElement::new(U512::from_str_radix("3410381082791005532", 10).unwrap(), p);
        let y = FiniteFieldElement::new(U512::from_str_radix("3959394867921462649", 10).unwrap(), p);
        EllipticCurvePoint::Point { x, y, a: secp256_k1_a, b: secp256_k1_b }
    };
    let Q = {
        let x = FiniteFieldElement::new(U512::from_str_radix("6030658041738565471", 10).unwrap(), p);
        let y = FiniteFieldElement::new(U512::from_str_radix("34549622697239310", 10).unwrap(), p);
        EllipticCurvePoint::Point { x, y, a: secp256_k1_a, b: secp256_k1_b }
    };
    let r = U512::from_str_radix("1135596179020030", 10).unwrap();

    let f = EllipticCurvePoint::weil(P, Q, r);

    println!("{}", f);
}

pub fn search(base: FiniteFieldElement, target: FiniteFieldElement) -> U512 {
    let mut i = U512::one();
    let mut b = base;
    println!("{}, {}", base, target);
    while b != target {
        b = b * base;
        i += U512::one();
    }
    i
}*/

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    mode: String,

    #[clap(short, long, value_parser)]
    file: Option<String>,

    #[clap(short, long, value_parser)]
    ip: Option<String>,

    #[clap(short, long, value_parser)]
    port: Option<u16>,
}

fn listen_tcp_server(port: u16) -> TcpStream {
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                return stream;
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
    panic!("Cannot connect to client");
}

fn main() {
    let args = Args::parse();

    match &*args.mode {
        "server" => {
            server(args);
        }
        "client" => {
            client(args);
        }
        _ => {
            println!("Unknown mode");
        }
    }
}

fn server(args: Args) {
    let mut stream = listen_tcp_server(args.port.unwrap());

    debug!("Receiving functions...");
    let mut buf = String::default();
    BufReader::new(stream.try_clone().unwrap())
        .read_line(&mut buf)
        .unwrap();

    let functions: HashMap<String, Box<Node>> = serde_json::from_str(&buf).unwrap();
    debug!("Received: {:?}", functions);

    let mut gpsl = GPSL::new(
        Source::new(String::default()),
        Some(functions),
        Some(HashMap::new()),
        Some(HashMap::new()),
        vec![STD_FUNC],
    );

    debug!("Receiving function call...");
    BufReader::new(stream.try_clone().unwrap())
        .read_line(&mut buf)
        .unwrap();
    debug!("Received");
    debug!("{}", buf);

    let function_call: ServerFunctionCall = serde_json::from_str(&buf).unwrap();

    let result = gpsl.run(function_call.name, function_call.args);
    let external_function_return = ExternalFuncReturn {
        status: ExternalFuncStatus::SUCCESS,
        value: Some(result.unwrap()),
    };

    debug!("Sending result...");
    stream
        .write_fmt(format_args!(
            "{}\n",
            serde_json::to_string(&external_function_return).unwrap()
        ))
        .unwrap();
}

fn client(args: Args) {
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
        match *function.clone().1 {
            Node::Function { attribute, .. } => match *(attribute.unwrap()) {
                Node::Attribute { name, args } => {
                    if name == String::from("server") {
                        println!("{:?}", function);
                        let ip = {
                            let mut t_ip = None;
                            for arg in args {
                                let ip = match *arg {
                                    Node::Operator { kind, lhs, rhs } => {
                                        if kind == NodeKind::ASSIGN {
                                            if lhs.extract_string() == String::from("ip") {
                                                println!("IP: {}", rhs.extract_string());
                                                Some(rhs.extract_string())
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                };
                                if ip.is_some() {
                                    t_ip = ip;
                                    break;
                                }
                            }
                            t_ip.unwrap()
                        };

                        let t_functions = server_functions.entry(ip).or_insert(HashMap::new());
                        t_functions.insert(function.clone().0.clone(), function.clone().1.clone());
                    }
                }
                _ => {}
            },
            _ => {}
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
        servers.insert(ip, Arc::new(Mutex::new(stream)));
    }

    let mut gpsl = GPSL::new(
        source,
        Some(functions),
        Some(server_functions),
        Some(servers),
        vec![STD_FUNC],
    );
    let res = gpsl.run("main".to_string(), HashMap::new());
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
}
