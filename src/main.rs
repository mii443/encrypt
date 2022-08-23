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
