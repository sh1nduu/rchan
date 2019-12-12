use super::error::*;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved,
    Identifier,
    Num,
    Return,
    Eof,
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn print_token(token: &Option<Box<Token>>) -> String {
        match &token {
            Some(token) => {
                let this = match token.kind {
                    TokenKind::Num => token.val.unwrap().to_string() + "|",
                    TokenKind::Eof => "Eof".to_string(),
                    _ => token.string.clone().unwrap() + "|",
                };

                this + &print_token(&token.next)
            }
            None => String::new(),
        }
    }

    #[test]
    fn tokenize_test() {
        let t = tokenize("+").ok().unwrap();
        assert_eq!(print_token(&t), "+|Eof".to_string());
        let t = tokenize("+-").ok().unwrap();
        assert_eq!(print_token(&t), "+|-|Eof".to_string());
        let t = tokenize("1+2").ok().unwrap();
        assert_eq!(print_token(&t), "1|+|2|Eof".to_string());
        let t = tokenize("34+5").ok().unwrap();
        assert_eq!(print_token(&t), "34|+|5|Eof".to_string());
        let t = tokenize("67 - 8").ok().unwrap();
        assert_eq!(print_token(&t), "67|-|8|Eof".to_string());
        let t = tokenize("1*(2 + 3)/(4-2)").ok().unwrap();
        assert_eq!(print_token(&t), "1|*|(|2|+|3|)|/|(|4|-|2|)|Eof".to_string());
        let t = tokenize("1 >= 2").ok().unwrap();
        assert_eq!(print_token(&t), "1|>=|2|Eof".to_string());
        let t = tokenize("1 != 2").ok().unwrap();
        assert_eq!(print_token(&t), "1|!=|2|Eof".to_string());
        let t = tokenize("1 < 2").ok().unwrap();
        assert_eq!(print_token(&t), "1|<|2|Eof".to_string());
        let t = tokenize("a + 1").ok().unwrap();
        assert_eq!(print_token(&t), "a|+|1|Eof".to_string());
        let t = tokenize("a = 1;").ok().unwrap();
        assert_eq!(print_token(&t), "a|=|1|;|Eof".to_string());
        let t = tokenize("21;38").ok().unwrap();
        assert_eq!(print_token(&t), "21|;|38|Eof".to_string());
        let t = tokenize("ab=38").ok().unwrap();
        assert_eq!(print_token(&t), "ab|=|38|Eof".to_string());
        let t = tokenize("ab-c").ok().unwrap();
        assert_eq!(print_token(&t), "ab|-|c|Eof".to_string());
        let t = tokenize("return 0;").ok().unwrap();
        assert_eq!(t.unwrap().kind, TokenKind::Return);
        let t = tokenize("return_ = 0").ok().unwrap();
        assert_ne!(t.unwrap().kind, TokenKind::Return);
        let t = tokenize("return 0; a").ok().unwrap();
        assert_eq!(print_token(&t), "return|0|;|a|Eof".to_string());
    }
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
