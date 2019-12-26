extern crate rchan;

use rchan::generator;
use rchan::lexer;
use rchan::parser;
use std::env;

fn parse_arguments() -> Result<String, std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let e = std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Arguments should have 1 parameters".to_string(),
        );
        return Err(e);
    }

    Ok(args[1].clone())
}

fn main() -> Result<(), std::io::Error> {
    let input = parse_arguments()?;
    let tokens = lexer::lex(&input).unwrap();
    // println!("{:?}", tokens);
    let code = parser::parse(tokens).unwrap();
    // println!("{:?}", code);
    generator::code_gen(code);

    Ok(())
}
