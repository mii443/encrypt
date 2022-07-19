mod gpsl;
use gpsl::variable::Variable;
use gpsl::{external_function::STD_FUNC, parser::*, source::*, tokenizer::*, vm::gpsl::*};
use primitive_types::U512;
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
    file: String,
}

fn listen_tcp_server(port: u16) -> TcpStream {
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
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
    let s = serde_json::to_string(&Variable::Number { value: 0 });
    println!("{}", s.unwrap());

    match &*args.mode {
        "gpsl" => {
            let mut source =
                Source::new(fs::read_to_string(&(args.file)).expect("Cannot read file."));

            let mut tokenizer = Tokenizer::new();
            tokenizer.tokenize(&mut source).unwrap();

            let mut parser = gpsl::parser::Parser {
                tokenizer,
                local_vars: HashMap::new(),
            };

            let stream = listen_tcp_server(8080);
            let mut gpsl = GPSL::new(
                source,
                Some(parser.functions().unwrap()),
                Arc::new(Mutex::new(Some(stream))),
                vec![STD_FUNC],
            );
            let res = gpsl.run("main".to_string(), vec![]);
            if let Err(err) = res {
                println!("Error: {:?}", err);
            }
        }
        _ => {}
    }
}
