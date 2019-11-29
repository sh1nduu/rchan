use std::env;

#[derive(PartialEq)]
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
    fn new(kind: TokenKind, string: Option<String>) -> Token {
        Token {
            kind: kind,
            next: None,
            val: None,
            string: string,
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
    let mut iter = input.chars();
    let mut is_prev_digit = false;
    loop {
        match iter.next() {
            Some(c) => match c {
                c if c.is_whitespace() => continue,
                '+' | '-' => {
                    let new_token = Token::new(TokenKind::Reserved, Some(c.to_string()));
                    current = assign_next_and_replace(current, new_token);
                    is_prev_digit = false;
                }
                c if is_digit(c) && !is_prev_digit => {
                    let mut new_token = Token::new(TokenKind::Num, Some(c.to_string()));
                    new_token.val = c.to_digit(10).map(|a| i64::from(a));
                    current = assign_next_and_replace(current, new_token);
                    is_prev_digit = true;
                }
                c if is_digit(c) && is_prev_digit => {
                    if let Some(x) = c.to_digit(10).map(|a| i64::from(a)) {
                        if let Some(cur) = current {
                            cur.val = cur.val.map(|a| a * 10 + x);
                        }
                    }
                }
                _ => break,
            },
            _ => break,
        }
    }
    let eof_token = Token::new(TokenKind::Eof, None);
    let _ = assign_next_and_replace(current, eof_token);
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
    let t4 = tokenize("34+5");
    assert!(t4.unwrap().val.unwrap() == 34);
    let t5 = tokenize("67 - 8").unwrap().next.unwrap().next;
    assert!(t5.unwrap().val.unwrap() == 8);
    let t6 = tokenize("91 + 2").unwrap().next.unwrap().next.unwrap().next;
    assert!(t6.unwrap().kind == TokenKind::Eof);
}

#[test]
fn expect_number_test() {
    let t1 = tokenize("1");
    assert!(expect_number(&t1) == Some(1));
    let t2 = tokenize("32");
    assert!(expect_number(&t2) == Some(32));
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

fn error(s: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, s))
}

fn expect_number(token: &Option<Box<Token>>) -> Option<i64> {
    if let Some(t) = token {
        t.val
    } else {
        None
    }
}

fn next_token(token: Option<Box<Token>>) -> Option<Box<Token>> {
    if let Some(t) = token {
        t.next
    } else {
        None
    }
}

fn main() -> Result<(), std::io::Error> {
    let arg1 = parse_arguments()?;

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut token = tokenize(&arg1);
    if let Some(v) = expect_number(&token) {
        println!("  mov rax, {}", v);
        token = next_token(token);
    } else {
        return error("Unexpected char");
    }

    loop {
        match &token {
            Some(t) => match t.kind {
                TokenKind::Eof => break,
                TokenKind::Reserved => {
                    if let Some(op) = &t.string {
                        match op.as_str() {
                            "+" => {
                                token = next_token(token);
                                let v = expect_number(&token);
                                println!("  add rax, {}", v.unwrap());
                                token = next_token(token);
                            }
                            "-" => {
                                token = next_token(token);
                                let v = expect_number(&token);
                                println!("  sub rax, {}", v.unwrap());
                                token = next_token(token);
                            }
                            _ => return error("Unexpected char"),
                        }
                    }
                }
                _ => return error("Unexpected char"),
            },
            _ => break,
        }
    }
    println!("  ret");
    Ok(())
}
