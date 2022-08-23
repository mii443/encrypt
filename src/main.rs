mod args;
mod client;
mod common;
mod config;
mod elliptic_curve;
mod gpsl;
mod server;
use args::Args;
use clap::Parser;
use client::start_client;
use server::start_server;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let args = Args::parse();

    if let Some(_) = args.port {
        start_server(args);
    } else {
        start_client(args);
    }
}
