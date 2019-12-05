extern crate rchan;

use rchan::codegen::*;
use rchan::parse::*;
use rchan::tokenize::*;
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

fn error(s: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, s))
}

fn main() -> Result<(), std::io::Error> {
    let arg1 = parse_arguments()?;

    match tokenize(&arg1) {
        Ok(mut token) => {
            let node = expr(&mut token);

            println!(".intel_syntax noprefix");
            println!(".global main");
            println!("main:");

            gen(&mut Some(node));

            println!("  pop rax\n");
            println!("  ret");
        }
        Err(err) => {
            return error(&err.build_error_message(&arg1));
        }
    }
    Ok(())
}
