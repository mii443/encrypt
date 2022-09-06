use clap::Parser;
use encrypt::args::Args;
use encrypt::client::start_client;
use encrypt::server::start_server;
use std::env;

fn main() {
    let args = Args::parse();
    if args.debug {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    if let Some(_) = args.port {
        start_server(args);
    } else {
        start_client(args);
    }
}
