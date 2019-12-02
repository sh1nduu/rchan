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

#[derive(PartialEq)]
enum NodeKind {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Num, // Integer
}

struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: Option<i64>,
}

impl Node {
    fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Node {
        Node {
            kind: kind,
            lhs: Some(lhs),
            rhs: Some(rhs),
            val: None,
        }
    }

    fn new_node_num(val: i64) -> Node {
        Node {
            kind: NodeKind::Num,
            lhs: None,
            rhs: None,
            val: Some(val),
        }
    }
}

fn expr(token: &mut Option<Box<Token>>) -> Box<Node> {
    let mut node = mul(token);
    loop {
        if consume('+', token) {
            node = Box::new(Node::new_node(NodeKind::Add, node, mul(token)))
        } else if consume('-', token) {
            node = Box::new(Node::new_node(NodeKind::Sub, node, mul(token)))
        } else {
            return node;
        }
    }
}

fn mul(token: &mut Option<Box<Token>>) -> Box<Node> {
    let mut node = primary(token);
    loop {
        if consume('*', token) {
            node = Box::new(Node::new_node(NodeKind::Mul, node, primary(token)))
        } else if consume('/', token) {
            node = Box::new(Node::new_node(NodeKind::Div, node, primary(token)))
        } else {
            return node;
        }
    }
}

fn primary(token: &mut Option<Box<Token>>) -> Box<Node> {
    if consume('(', token) {
        let node = expr(token);
        expect(')', token);
        return node;
    }
    Box::new(Node::new_node_num(expect_number(token).unwrap()))
}

fn gen(node: &mut Option<Box<Node>>) {
    if let Some(n) = node {
        if n.kind == NodeKind::Num {
            println!("  push {}\n", n.val.unwrap());
            return;
        }

        gen(&mut n.lhs);
        gen(&mut n.rhs);

        println!("  pop rdi\n");
        println!("  pop rax\n");
        match n.kind {
            NodeKind::Add => println!("  add rax, rdi\n"),
            NodeKind::Sub => println!("  sub rax, rdi\n"),
            NodeKind::Mul => println!("  imul rax, rdi\n"),
            NodeKind::Div => {
                println!("  cqo\n");
                println!("  idiv rdi\n");
            }
            _ => panic!("Unexpected Node"),
        }
        println!("  push rax\n");
    }
}

enum RError {
    Nothing,
    Tokenize(usize, String),
}

impl RError {
    fn build_error_message<'a>(&self, input: &'a str) -> String {
        match self {
            Self::Tokenize(i, s) => {
                let mut out = format!("{}\n", input);
                for _ in 0..*i {
                    out += " ";
                }
                out += format!("^ {}\n", s).as_str();
                out
            }
            _ => "Unexpected Error".to_string(),
        }
    }
}

fn tokenize<'a>(input: &'a str) -> Result<Option<Box<Token>>, RError> {
    let mut head = Some(Box::new(Token {
        kind: TokenKind::Reserved,
        next: None,
        val: None,
        string: None,
    }));
    let mut current = &mut head;
    let mut iter = input.chars().enumerate();
    let mut is_prev_digit = false;
    loop {
        match iter.next() {
            Some((i, c)) => match c {
                c if c.is_whitespace() => continue,
                '+' | '-' | '*' | '/' | '(' | ')' => {
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
                _ => return Err(RError::Tokenize(i, "Invalid character".to_string())),
            },
            _ => break,
        }
    }
    let eof_token = Token::new(TokenKind::Eof, None);
    let _ = assign_next_and_replace(current, eof_token);
    match head {
        Some(head) => Ok(head.next),
        None => Err(RError::Nothing),
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
    let t1 = tokenize("+").ok().unwrap();
    assert!(t1.unwrap().string.unwrap() == '+'.to_string());
    let t2 = tokenize("+-").ok().unwrap();
    assert!(t2.unwrap().string.unwrap() == '+'.to_string());
    let t3 = tokenize("1+2").ok().unwrap().unwrap().next.unwrap().next;
    assert!(t3.unwrap().val.unwrap() == 2);
    let t4 = tokenize("34+5").ok().unwrap();
    assert!(t4.unwrap().val.unwrap() == 34);
    let t5 = tokenize("67 - 8").ok().unwrap().unwrap().next.unwrap().next;
    assert!(t5.unwrap().val.unwrap() == 8);
    let t6 = tokenize("91 + 2")
        .ok()
        .unwrap()
        .unwrap()
        .next
        .unwrap()
        .next
        .unwrap()
        .next;
    assert!(t6.unwrap().kind == TokenKind::Eof);
    let t7 = tokenize("1*(2 + 3)/(4-2)").ok().unwrap();
    assert_eq!(t7.unwrap().val, Some(1));
}

#[test]
fn expect_number_test() {
    let t1 = &mut tokenize("1").ok().unwrap();
    assert!(expect_number(t1) == Some(1));
    let t2 = &mut tokenize("32").ok().unwrap();
    assert!(expect_number(t2) == Some(32));
}

#[test]
fn consume_test() {
    let t1 = &mut tokenize("+").ok().unwrap();
    assert!(consume('+', t1));
    let t2 = &mut tokenize("-").ok().unwrap();
    assert!(consume('-', t2));
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

fn consume(op: char, token: &mut Option<Box<Token>>) -> bool {
    if is_expected(op, token) {
        next_token(token);
        return true;
    }
    false
}

fn expect(op: char, token: &mut Option<Box<Token>>) {
    if is_expected(op, token) {
        next_token(token);
    } else {
        panic!("Unexpected char");
    }
}

fn is_expected(op: char, token: &mut Option<Box<Token>>) -> bool {
    if let Some(t) = token {
        if t.kind != TokenKind::Reserved {
            return false;
        }
        if let Some(s) = &t.string {
            if let Some(c) = s.chars().next() {
                if c != op {
                    return false;
                }
            }
        }
    }
    true
}

fn expect_number(token: &mut Option<Box<Token>>) -> Option<i64> {
    if let Some(t) = token {
        let ret = t.val;
        next_token(token);
        ret
    } else {
        None
    }
}

fn next_token(token: &mut Option<Box<Token>>) {
    unsafe {
        if let Some(t) = token {
            let next = &mut t.next as *mut Option<Box<Token>>;
            std::ptr::swap(token, next);
        } else {
            let next = &mut None as *mut Option<Box<Token>>;
            std::ptr::swap(token, next);
        }
    }
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
