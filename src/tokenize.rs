use super::error::*;

#[derive(PartialEq)]
pub enum TokenKind {
    Reserved,
    Identifier,
    Num,
    Eof,
}

pub struct Token {
    pub kind: TokenKind,
    pub next: Option<Box<Token>>,
    pub val: Option<i64>,
    pub string: Option<String>,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, string: Option<String>) -> Token {
        let len = if string.is_some() {
            string.clone().unwrap().len()
        } else {
            0
        };
        Token {
            kind: kind,
            next: None,
            val: None,
            string: string,
            len: len,
        }
    }
}

pub fn tokenize<'a>(input: &'a str) -> Result<Option<Box<Token>>, RError> {
    let mut head = Some(Box::new(Token {
        kind: TokenKind::Reserved,
        next: None,
        val: None,
        string: None,
        len: 0,
    }));
    let mut current = &mut head;
    let mut iter = input.chars().enumerate();
    let mut is_prev_digit = false;
    loop {
        match iter.next() {
            Some((i, c)) => match c {
                c if c.is_whitespace() => continue,
                '<' | '>' | '=' | '!' if is_next('=', iter.clone()) => {
                    let mut s = c.to_string();
                    s.push(iter.next().unwrap().1);
                    let new_token = Token::new(TokenKind::Reserved, Some(s));
                    current = assign_next_and_replace(current, new_token);
                    is_prev_digit = false;
                }
                '+' | '-' | '*' | '/' | '(' | ')' | '<' | '>' | '=' | ';' => {
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
                c if c.is_ascii_lowercase() => {
                    let new_token = Token::new(TokenKind::Identifier, Some(c.to_string()));
                    current = assign_next_and_replace(current, new_token);
                    is_prev_digit = false;
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

fn is_next(op: char, mut iter: std::iter::Enumerate<std::str::Chars>) -> bool {
    if let Some((_, c)) = iter.next() {
        c == op
    } else {
        false
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
    let t8 = tokenize("1 >= 2").ok().unwrap().unwrap().next;
    assert_eq!(t8.unwrap().string, Some(">=".to_string()));
    let t9 = tokenize("1 != 2").ok().unwrap().unwrap().next;
    assert_eq!(t9.unwrap().string, Some("!=".to_string()));
    let t10 = tokenize("1 < 2").ok().unwrap().unwrap().next;
    assert_eq!(t10.unwrap().string, Some("<".to_string()));
    let t11 = tokenize("a + 1").ok().unwrap();
    assert_eq!(t11.unwrap().string, Some("a".to_string()));
    let t12 = tokenize("a = 1;").ok().unwrap().unwrap().next;
    assert_eq!(t12.unwrap().string, Some("=".to_string()));
    let t13 = tokenize("21;38").ok().unwrap().unwrap().next;
    assert_eq!(t13.unwrap().string, Some(";".to_string()));
}
