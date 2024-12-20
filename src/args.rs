use std::path::PathBuf;

use clap::Parser;
use clap::ValueHint;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub port: Option<u16>,

    #[clap(name = "FILE", value_hint = ValueHint::AnyPath)]
    pub file: Option<PathBuf>,

    #[clap(short, long, takes_value = false)]
    pub debug: bool,

    #[clap(short, long, takes_value = false)]
    pub compile: bool,

    #[clap(short = 'e', long, value_parser)]
    pub curve: Option<String>,
}
