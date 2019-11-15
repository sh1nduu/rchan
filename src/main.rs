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
    let arg1 = parse_arguments()?;
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", arg1);
    println!("  ret");
    Ok(())
}
