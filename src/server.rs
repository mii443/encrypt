use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use log::{debug, info};

use crate::args::Args;
use crate::elliptic_curve::encryption::Encryption;
use crate::gpsl::external_function::{ExternalFuncReturn, ExternalFuncStatus, STD_FUNC};
use crate::gpsl::node::Node;
use crate::gpsl::vm::gpsl::{ServerFunctionCall, GPSL};

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

pub fn start_server(args: Args) {
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
