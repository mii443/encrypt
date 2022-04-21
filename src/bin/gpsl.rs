use gpsl::{
    gpsl::GPSL,
    source::Source,
    std::*,
    tokenizer::Tokenizer,
    parser::Parser
};
use std::{fs, env, collections::HashMap};
fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut source = Source::new(fs::read_to_string(&(args.last().unwrap())).expect("Cannot read file."));

    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(&mut source);

    let mut parser = Parser {
        tokenizer,
        local_vars: HashMap::new()
    };

    let mut gpsl = GPSL::new(source, Some(parser.functions().unwrap()), vec![STD_FUNC]);
    let res = gpsl.run("main".to_string(), vec![]);
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
}