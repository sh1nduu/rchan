use std::env;

pub enum TokenKind {
    Reserved,
    Num,
    Eof,
}

pub struct Token {
    pub kind: TokenKind,
    pub next: Option<Box<Token>>,
    pub val: Option<i64>,
    pub string: Option<String>,
}

impl Token {
    fn new(kind: TokenKind, string: String) -> Token {
        Token {
            kind: kind,
            next: None,
            val: None,
            string: Some(string),
        }
    }
}

fn tokenize<'a>(input: &'a str) -> Option<Box<Token>> {
    let mut head = Some(Box::new(Token {
        kind: TokenKind::Reserved,
        next: None,
        val: None,
        string: None,
    }));
    let mut current = &mut head;
    for c in input.chars() {
        match c {
            '+' | '-' => {
                let new_token = Token::new(TokenKind::Reserved, c.to_string());
                current = assign_next_and_replace(current, new_token);
            }
            c if is_digit(c) => {
                let mut new_token = Token::new(TokenKind::Num, c.to_string());
                new_token.val = c.to_digit(10).map(|a| i64::from(a));
                current = assign_next_and_replace(current, new_token);
            }
            _ => continue,
        }
    }
    match head {
        Some(head) => head.next,
        None => None,
    }
}

fn assign_next_and_replace(
    mut current: &mut Option<Box<Token>>,
    new_token: Token,
) -> &mut Option<Box<Token>> {
    let mut ptr = current as *mut Option<Box<Token>>;
    unsafe {
        if let Some(cur) = &mut *ptr {
            cur.next = Some(Box::new(new_token));
            let next = &mut cur.next as *mut Option<Box<Token>>;
            ptr = next;
        }
        current = ptr.as_mut().unwrap();
    }
    current
}

fn is_digit(c: char) -> bool {
    c.to_digit(10).is_some()
}

#[test]
fn tokenize_test() {
    let t1 = tokenize("+");
    assert!(t1.unwrap().string.unwrap() == '+'.to_string());
    let t2 = tokenize("+-");
    assert!(t2.unwrap().string.unwrap() == '+'.to_string());
    let t3 = tokenize("1+2").unwrap().next.unwrap().next;
    assert!(t3.unwrap().val.unwrap() == 2);
}

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
