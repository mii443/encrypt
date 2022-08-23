use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub mode: String,

    #[clap(short, long, value_parser)]
    pub file: Option<String>,

    #[clap(short, long, value_parser)]
    pub ip: Option<String>,

    #[clap(short, long, value_parser)]
    pub port: Option<u16>,
}
