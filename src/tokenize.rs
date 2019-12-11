use super::error::*;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved,
    Identifier,
    Num,
    Return,
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

#[derive(PartialEq)]
enum TokenizeMode {
    Normal,
    Num,
    Variable,
}

impl TokenizeMode {
    fn normal(&mut self) {
        *self = TokenizeMode::Normal;
    }
    fn number(&mut self) {
        *self = TokenizeMode::Num;
    }
    fn variable(&mut self) {
        *self = TokenizeMode::Variable;
    }
    fn is_number(&self) -> bool {
        self == &TokenizeMode::Num
    }
    fn is_variable(&self) -> bool {
        self == &TokenizeMode::Variable
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
    let mut reader = StringReader::new(input);
    let mut mode = TokenizeMode::Normal;
    loop {
        match reader.next() {
            Some(c) => match c {
                c if c.is_whitespace() => continue,
                '<' | '>' | '=' | '!' if reader.next_is('=') => {
                    let mut s = c.to_string();
                    s.push(reader.next().unwrap());
                    let new_token = Token::new(TokenKind::Reserved, Some(s));
                    current = assign_next_and_replace(current, new_token);
                    mode.normal();
                }
                '+' | '-' | '*' | '/' | '(' | ')' | '<' | '>' | '=' | ';' => {
                    let new_token = Token::new(TokenKind::Reserved, Some(c.to_string()));
                    current = assign_next_and_replace(current, new_token);
                    mode.normal();
                }
                c if is_digit(c) && !mode.is_number() => {
                    let mut new_token = Token::new(TokenKind::Num, Some(c.to_string()));
                    new_token.val = c.to_digit(10).map(|a| i64::from(a));
                    current = assign_next_and_replace(current, new_token);
                    mode.number();
                }
                c if is_digit(c) && mode.is_number() => {
                    if let Some(x) = c.to_digit(10).map(|a| i64::from(a)) {
                        if let Some(cur) = current {
                            cur.val = cur.val.map(|a| a * 10 + x);
                        }
                    }
                }
                c if is_variable_nameable(c) && !mode.is_variable() => {
                    if reader.compare("return", Some(-1)) {
                        if reader.offset_is_map(5, |c| !is_variable_nameable(c)) {
                            println!("Next is map ok");
                            let new_token =
                                Token::new(TokenKind::Return, Some("return".to_string()));
                            current = assign_next_and_replace(current, new_token);
                            mode.normal();
                            reader.foward(5);
                            continue;
                        }
                    }
                    let new_token = Token::new(TokenKind::Identifier, Some(c.to_string()));
                    current = assign_next_and_replace(current, new_token);
                    mode.variable();
                }
                c if is_variable_nameable(c) && mode.is_variable() => {
                    if let Some(cur) = current {
                        if let Some(mut s) = cur.string.clone() {
                            s.push(c);
                            cur.string = Some(s);
                            cur.len += 1;
                        }
                    }
                }
                _ => {
                    return Err(RError::Tokenize(
                        reader.index,
                        "Invalid character".to_string(),
                    ))
                }
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

fn is_variable_nameable(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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
    let t14 = tokenize("ab=38").ok().unwrap();
    assert_eq!(t14.unwrap().string, Some("ab".to_string()));
    let t15 = tokenize("ab-c").ok().unwrap();
    assert_eq!(t15.unwrap().string, Some("ab".to_string()));
    let t16 = tokenize("return 0").ok().unwrap();
    assert_eq!(t16.unwrap().kind, TokenKind::Return);
    let t17 = tokenize("return_ = 0").ok().unwrap();
    assert_ne!(t17.unwrap().kind, TokenKind::Return);
    let t18 = tokenize("return 0; a").ok().unwrap().unwrap().next;
    assert_eq!(t18.unwrap().string, Some("0".to_string()));
}

struct StringReader {
    input: String,
    index: usize,
    len: usize,
}

impl StringReader {
    fn new<'a>(input: &'a str) -> Self {
        StringReader {
            input: input.to_string(),
            index: 0,
            len: input.len(),
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.is_end() {
            return None;
        }

        let ret = self.input.chars().nth(self.index);
        self.index += 1;
        ret
    }

    fn prev(&mut self) -> Option<char> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;
        self.input.chars().nth(self.index)
    }

    fn next_is(&mut self, c: char) -> bool {
        let mut ret = false;
        if let Some(myc) = self.next() {
            ret = myc == c;
        }
        if !self.is_end() {
            self.prev();
        }
        ret
    }
    fn offset_is_map(&mut self, offset: usize, condition: fn(char) -> bool) -> bool {
        let mut ret = false;
        let current = self.index;
        self.index += offset;
        if let Some(c) = self.next() {
            ret = condition(c);
        }
        if !self.is_end() {
            self.prev();
        }
        self.index = current;
        ret
    }

    fn compare<'a>(&mut self, s: &'a str, offset: Option<i32>) -> bool {
        let s = s.to_string();
        let c = self.take(s.len(), offset);
        println!("{:?}", c);
        s == c
    }

    fn take(&mut self, size: usize, offset: Option<i32>) -> String {
        let mut ret = String::new();
        let offset = offset.unwrap_or(0);
        let current = self.index;
        let mut index = self.index as i32;
        index += offset;
        self.index = index as usize;
        for _ in 0..size {
            if let Some(c) = self.next() {
                ret.push(c);
            } else {
                break;
            }
        }
        self.index = current;
        ret
    }

    fn foward(&mut self, size: usize) {
        self.index += size;
    }

    fn is_end(&self) -> bool {
        self.index >= self.len
    }
}

#[test]
fn string_reader_test() {
    let mut reader = StringReader::new("abc");
    assert_eq!(reader.prev(), None);
    assert!(reader.compare("abc", None));
    assert!(reader.offset_is_map(1, |c| c == 'b'));
    assert_eq!(reader.next(), Some('a'));
    assert!(reader.next_is('b'));
    assert!(!reader.next_is('a'));
    assert_eq!(reader.next(), Some('b'));
    assert_eq!(reader.next(), Some('c'));
    assert!(!reader.next_is('a'));
    assert_eq!(reader.next(), None);
    assert_eq!(reader.prev(), Some('c'));
    assert_eq!(reader.prev(), Some('b'));
    assert_eq!(reader.prev(), Some('a'));
    assert_eq!(reader.prev(), None);
}
