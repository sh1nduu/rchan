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

fn read_number(input: &str) -> Option<(usize, String)> {
    let mut iter = input.chars();
    let mut s = "".to_string();
    let mut i = 0;
    loop {
        match iter.next() {
            Some(c) => {
                if let Some(n) = c.to_digit(10) {
                    i += 1;
                    s += &n.to_string();
                } else {
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }
    if i == 0 {
        None
    } else {
        Some((i, s))
    }
}

fn main() -> Result<(), std::io::Error> {
    let arg1 = parse_arguments()?;

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut iter = arg1.chars();
    if let Some(t) = read_number(iter.as_str()) {
        iter.nth(t.0 - 1);
        println!("  mov rax, {}", t.1);
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unexpected char",
        ));
    }

    loop {
        if let Some(c) = iter.next() {
            match c {
                '+' | '-' => {
                    if let Some(t) = read_number(iter.as_str()) {
                        iter.nth(t.0 - 1);
                        match c {
                            '+' => println!("  add rax, {}", t.1),
                            '-' => println!("  sub rax, {}", t.1),
                            _ => unreachable!(),
                        }
                    }
                }
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Unexpected char",
                    ));
                }
            }
        } else {
            break;
        }
    }

    println!("  ret");
    Ok(())
}
