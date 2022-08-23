mod common;
mod config;
mod elliptic_curve;
mod gpsl;
use common::finite_field::FiniteFieldElement;
use elliptic_curve::elliptic_curve::EllipticCurvePoint;
use elliptic_curve::encryption::Encryption;
use gpsl::external_function::ExternalFuncReturn;
use gpsl::external_function::ExternalFuncStatus;
use gpsl::node::Node;
use gpsl::node::NodeKind;
use gpsl::vm::gpsl::ServerFunctionCall;
use gpsl::{external_function::STD_FUNC, source::*, tokenizer::*, vm::gpsl::*};
use log::*;
use primitive_types::U512;
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

fn o_main() {
    let p = U512::from_str_radix("1009", 10).unwrap();

    let secp256_k1_a = FiniteFieldElement::new(U512::from(37u8), p);
    let secp256_k1_b = FiniteFieldElement::new(U512::from_str_radix("0", 10).unwrap(), p);

    let pp = {
        let x = FiniteFieldElement::new(U512::from_str_radix("417", 10).unwrap(), p);
        let y = FiniteFieldElement::new(U512::from_str_radix("952", 10).unwrap(), p);
        EllipticCurvePoint::Point {
            x,
            y,
            a: secp256_k1_a,
            b: secp256_k1_b,
        }
    };
    let pd = {
        let x = FiniteFieldElement::new(U512::from_str_radix("561", 10).unwrap(), p);
        let y = FiniteFieldElement::new(U512::from_str_radix("153", 10).unwrap(), p);
        EllipticCurvePoint::Point {
            x,
            y,
            a: secp256_k1_a,
            b: secp256_k1_b,
        }
    };

    let r = U512::from_str_radix("7", 10).unwrap();

    let f = EllipticCurvePoint::weil(pp, pd, r);

    let s = U512::from(10u8);
    let sd = U512::from(5u8);

    let q = pp * s;
    let qd = pd * sd;

    let ra = U512::from_str_radix("20", 10).unwrap();
    let rad = U512::from_str_radix("26", 10).unwrap();

    let m = U512::from_str_radix("2", 10).unwrap();
    let md = U512::from_str_radix("2", 10).unwrap();

    let s1 = pp * m + q * ra;
    let t1 = pp * ra;
    let s2 = pd * md + qd * rad;
    let t2 = pd * rad;

    let a = EllipticCurvePoint::weil(s1, s2, r);
    let b = EllipticCurvePoint::weil(s1, t2, r);
    let c = EllipticCurvePoint::weil(t1, s2, r);
    let d = EllipticCurvePoint::weil(t1, t2, r);

    let dec = a * d.pow(s * sd) / b.pow(sd) / c.pow(s) * f;

    println!("{} * {} = {}", m, md, search(f, dec));
}

pub fn search(base: FiniteFieldElement, target: FiniteFieldElement) -> U512 {
    let mut i = U512::one();
    let mut b = base;
    println!("{}, {}", base, target);
    while b != target {
        b = b * base;
        i += U512::one();
    }
    if i < U512::from(7u8) {
        i
    } else {
        U512::zero()
    }
}

use clap::Parser;

use crate::config::Config;

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
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
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
    env::set_var("RUST_LOG", "info");
    env_logger::init();
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
    loop {
        info!("GPSL Server listening on port {}", args.port.unwrap());
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
            Encryption::secp256k1(),
            None,
            None,
            vec![STD_FUNC],
        );

        stream.write_fmt(format_args!("received\n")).unwrap();

        loop {
            buf = String::default();
            if let Err(_) = BufReader::new(stream.try_clone().unwrap()).read_line(&mut buf) {
                break;
            }

            let function_call: ServerFunctionCall =
                if let Ok(function_call) = serde_json::from_str(&buf) {
                    function_call
                } else {
                    break;
                };

            let result = gpsl.run(function_call.name, function_call.args);
            let external_function_return = ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(result.unwrap()),
            };

            if let Err(_) = stream.write_fmt(format_args!(
                "{}\n",
                serde_json::to_string(&external_function_return).unwrap()
            )) {
                break;
            }
        }
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
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

    println!("{}", serde_json::to_string(&functions).unwrap());

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
                    let ip = match *arg {
                        Node::Operator { kind, lhs, rhs } => {
                            if kind == NodeKind::ASSIGN {
                                if lhs.extract_string() == String::from("ip") {
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
