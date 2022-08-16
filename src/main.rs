mod common;
mod elliptic_curve;
mod gpsl;
use common::finite_field::FiniteFieldElement;
use elliptic_curve::elliptic_curve::EllipticCurve;
use elliptic_curve::elliptic_curve::EllipticCurvePoint;
use elliptic_curve::encryption::Encryption;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use gpsl::external_function::ExternalFuncReturn;
use gpsl::external_function::ExternalFuncStatus;
use gpsl::node::Node;
use gpsl::node::NodeKind;
use gpsl::vm::gpsl::ServerFunctionCall;
use gpsl::{external_function::STD_FUNC, source::*, tokenizer::*, vm::gpsl::*};
use log::*;
use primitive_types::U512;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fs};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConfigFile {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

impl ConfigFile {
    pub fn from_config(config: Config) -> Self {
        let private_key = {
            if let Some(private_key) = config.private_key {
                let s = private_key.to_string();
                let encode = base64::encode(&s);
                Some(encode)
            } else {
                None
            }
        };

        let public_key = {
            if let Some(public_key) = config.public_key {
                let s = serde_json::to_string(&public_key).unwrap();
                let encode = base64::encode(&s);
                Some(encode)
            } else {
                None
            }
        };

        Self {
            private_key,
            public_key,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    pub private_key: Option<U512>,
    pub public_key: Option<EllipticCurvePoint>,
}

impl Config {
    fn read_file(file: &str) -> String {
        let mut file = fs::File::open(file).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        let mut d = ZlibDecoder::new(&contents[..]);
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();
        s
    }
    pub fn from_file(file: &str) -> Self {
        let file = Config::read_file(file);
        let config: ConfigFile = toml::from_str(&file).unwrap();

        let private_key = {
            if let Some(private_key) = config.private_key {
                let decoded = base64::decode(&private_key).unwrap();
                let s = std::str::from_utf8(&decoded).unwrap();
                Some(U512::from_str_radix(s, 10).unwrap())
            } else {
                None
            }
        };

        let public_key = {
            if let Some(public_key) = config.public_key {
                let decoded = base64::decode(&public_key).unwrap();
                let s = std::str::from_utf8(&decoded).unwrap();
                let r = EllipticCurvePoint::from_str(s).unwrap();
                Some(r)
            } else {
                None
            }
        };

        Config {
            private_key,
            public_key,
        }
    }
}

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

fn generate_encryption() -> Encryption {
    let p = U512::from_str_radix(
        "115792089237316195423570985008687907853269984665640564039457584007908834671663",
        10,
    )
    .unwrap();

    let secp256_k1_a = FiniteFieldElement::new(U512::from(0u8), p);
    let secp256_k1_b = FiniteFieldElement::new(U512::from(7u8), p);
    let secp256_k1_base_x = FiniteFieldElement::new(
        U512::from_str_radix(
            "55066263022277343669578718895168534326250603453777594175500187360389116729240",
            10,
        )
        .unwrap(),
        p,
    );
    let secp256_k1_base_y = FiniteFieldElement::new(
        U512::from_str_radix(
            "32670510020758816978083085130507043184471273380659243275938904335757337482424",
            10,
        )
        .unwrap(),
        p,
    );
    let secp256_k1_order = FiniteFieldElement::new(
        U512::from_str_radix(
            "115792089237316195423570985008687907852837564279074904382605163141518161494337",
            10,
        )
        .unwrap(),
        p,
    );
    let ec = EllipticCurve {
        a: secp256_k1_a,
        b: secp256_k1_b,
    };

    Encryption {
        ellictic_curve: ec,
        base_point: ec.point(secp256_k1_base_x, secp256_k1_base_y),
        order: secp256_k1_order,
        plain_mapping: vec![],
    }
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
            generate_encryption(),
            None,
            None,
            vec![STD_FUNC],
        );

        stream.write_fmt(format_args!("received\n")).unwrap();

        loop {
            debug!("Receiving function call...");
            buf = String::default();
            if let Err(_) = BufReader::new(stream.try_clone().unwrap()).read_line(&mut buf) {
                break;
            }
            debug!("Received");
            debug!("{}", buf);

            let function_call: ServerFunctionCall =
                if let Ok(function_call) = serde_json::from_str(&buf) {
                    function_call
                } else {
                    break;
                };

            trace!("Running function: {}", function_call.name);

            let result = gpsl.run(function_call.name, function_call.args);
            let external_function_return = ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: Some(result.unwrap()),
            };

            debug!("Sending result...");
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
        match *function.clone().1 {
            Node::Function { attribute, .. } => match *(attribute.unwrap()) {
                Node::Attribute { name, args } => {
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
        let mut buf = String::default();
        BufReader::new(stream.try_clone().unwrap())
            .read_line(&mut buf)
            .unwrap();
        if buf != String::from("received\n") {
            panic!("Server did not receive functions");
        }
        servers.insert(ip, Arc::new(Mutex::new(stream)));
    }

    let encryption = generate_encryption();

    let config = if Path::new("gpsl_conf.toml").exists() {
        Config::from_file("gpsl_conf.toml")
    } else {
        let private_key = Encryption::get_private_key();
        let config = Config {
            private_key: Some(private_key),
            public_key: Some(encryption.get_public_key(private_key)),
        };

        let mut file = File::create("gpsl_conf.toml").unwrap();
        let config_file = ConfigFile::from_config(config.clone());
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(toml::to_string(&config_file).unwrap().as_bytes())
            .unwrap();
        file.write_all(&e.finish().unwrap()).unwrap();

        config
    };

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
