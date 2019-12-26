#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Ident(String), // main
    Int(i32),      // 1
    Return,        // return
    Eof,           // ;
    Add,           // +
    Sub,           // -
    Mul,           // *
    Quo,           // /
    LParen,        // (
    RParen,        // )
    ASSIGN,        // =
    EQ,            // ==
    NEQ,           // !=
    LEQ,           // <=
    GEQ,           // >=
    LSS,           // <
    GRT,           // >
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Loc(usize, usize);

impl Loc {
    pub fn merge(&self, other: &Self) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

#[derive(Debug, PartialEq)]
pub struct Annot<T> {
    pub value: T,
    pub loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

pub type Token = Annot<TokenKind>;

impl Token {
    fn ident(s: &str, loc: Loc) -> Self {
        Self::new(TokenKind::Ident(s.to_string()), loc)
    }
    fn int(n: i32, loc: Loc) -> Self {
        Self::new(TokenKind::Int(n), loc)
    }
    fn return_(loc: Loc) -> Self {
        Self::new(TokenKind::Return, loc)
    }
    fn eof(loc: Loc) -> Self {
        Self::new(TokenKind::Eof, loc)
    }
    fn add(loc: Loc) -> Self {
        Self::new(TokenKind::Add, loc)
    }
    fn sub(loc: Loc) -> Self {
        Self::new(TokenKind::Sub, loc)
    }
    fn mul(loc: Loc) -> Self {
        Self::new(TokenKind::Mul, loc)
    }
    fn quo(loc: Loc) -> Self {
        Self::new(TokenKind::Quo, loc)
    }
    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
    fn assign(loc: Loc) -> Self {
        Self::new(TokenKind::ASSIGN, loc)
    }
    fn eq(loc: Loc) -> Self {
        Self::new(TokenKind::EQ, loc)
    }
    fn neq(loc: Loc) -> Self {
        Self::new(TokenKind::NEQ, loc)
    }
    fn leq(loc: Loc) -> Self {
        Self::new(TokenKind::LEQ, loc)
    }
    fn geq(loc: Loc) -> Self {
        Self::new(TokenKind::GEQ, loc)
    }
    fn lss(loc: Loc) -> Self {
        Self::new(TokenKind::LSS, loc)
    }
    fn grt(loc: Loc) -> Self {
        Self::new(TokenKind::GRT, loc)
    }
}

fn is_number(c: char) -> bool {
    c.to_digit(10).is_some()
}

fn is_identifier_nameable(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_match(input: &Vec<char>, pos: usize, expected: &str) -> bool {
    let end = pos + expected.len();
    if input.len() <= pos || input.len() <= end {
        return false;
    }
    let input_str: String = input[pos..(end)].into_iter().collect();
    input_str == expected
}

fn consume(input: &Vec<char>, pos: usize, expected: &str) -> Result<(String, usize), LexError> {
    if input.len() <= pos {
        return Err(LexError::eof(Loc(pos, pos)));
    }
    let end = pos + expected.len();
    let input_str: String = input[pos..(end)].into_iter().collect();
    if input_str != expected {
        return Err(LexError::invalid_char(input[pos], Loc(pos, end)));
    }
    Ok((expected.to_string(), end))
}

fn lex_int(input: &Vec<char>, mut pos: usize) -> (Token, usize) {
    let start = pos;
    while pos < input.len() && is_number(input[pos]) {
        pos += 1;
    }
    let n_str: String = input[start..pos].into_iter().collect();
    let n: i32 = n_str.parse().unwrap();
    (Token::int(n, Loc(start, pos)), pos)
}

fn lex_identifier(input: &Vec<char>, mut pos: usize) -> (Token, usize) {
    let start = pos;
    while pos < input.len() && is_identifier_nameable(input[pos]) {
        pos += 1;
    }
    let n_str: String = input[start..pos].into_iter().collect();
    (Token::ident(&n_str, Loc(start, pos)), pos)
}

fn lex_add(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "+").map(|(_, end)| (Token::add(Loc(start, end)), end))
}
fn lex_return(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "return").map(|(_, end)| (Token::return_(Loc(start, end)), end))
}
fn lex_eof(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, ";").map(|(_, end)| (Token::eof(Loc(start, end)), end))
}
fn lex_sub(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "-").map(|(_, end)| (Token::sub(Loc(start, end)), end))
}
fn lex_mul(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "*").map(|(_, end)| (Token::mul(Loc(start, end)), end))
}
fn lex_quo(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "/").map(|(_, end)| (Token::quo(Loc(start, end)), end))
}
fn lex_lparen(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "(").map(|(_, end)| (Token::lparen(Loc(start, end)), end))
}
fn lex_rparen(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, ")").map(|(_, end)| (Token::rparen(Loc(start, end)), end))
}
fn lex_eq(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "==").map(|(_, end)| (Token::eq(Loc(start, end)), end))
}
fn lex_neq(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "!=").map(|(_, end)| (Token::neq(Loc(start, end)), end))
}
fn lex_leq(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "<=").map(|(_, end)| (Token::leq(Loc(start, end)), end))
}
fn lex_geq(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, ">=").map(|(_, end)| (Token::geq(Loc(start, end)), end))
}
fn lex_assign(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "=").map(|(_, end)| (Token::assign(Loc(start, end)), end))
}
fn lex_lss(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, "<").map(|(_, end)| (Token::lss(Loc(start, end)), end))
}
fn lex_grt(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume(input, start, ">").map(|(_, end)| (Token::grt(Loc(start, end)), end))
}

#[derive(Debug)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::<Token>::new();
    let input: Vec<char> = input.chars().collect();
    let mut pos = 0;

    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer;
            tokens.push(tok);
            pos = p;
        }};
    }

    while pos < input.len() {
        let c = input[pos];
        match c {
            ' ' | '\n' => pos += 1,
            c if is_number(c) => lex_a_token!(lex_int(&input, pos)),
            c if is_identifier_nameable(c) => {
                if is_match(&input, pos, "return") {
                    lex_a_token!(lex_return(&input, pos)?)
                } else {
                    lex_a_token!(lex_identifier(&input, pos))
                }
            }
            '=' | '<' | '>' | '!' if input[pos + 1] == '=' => match c {
                '=' => lex_a_token!(lex_eq(&input, pos)?),
                '<' => lex_a_token!(lex_leq(&input, pos)?),
                '>' => lex_a_token!(lex_geq(&input, pos)?),
                '!' => lex_a_token!(lex_neq(&input, pos)?),
                _ => unimplemented!(),
            },
            '+' => lex_a_token!(lex_add(&input, pos)?),
            '-' => lex_a_token!(lex_sub(&input, pos)?),
            '*' => lex_a_token!(lex_mul(&input, pos)?),
            '/' => lex_a_token!(lex_quo(&input, pos)?),
            '(' => lex_a_token!(lex_lparen(&input, pos)?),
            ')' => lex_a_token!(lex_rparen(&input, pos)?),
            '=' => lex_a_token!(lex_assign(&input, pos)?),
            '<' => lex_a_token!(lex_lss(&input, pos)?),
            '>' => lex_a_token!(lex_grt(&input, pos)?),
            ';' => lex_a_token!(lex_eof(&input, pos)?),
            _ => unimplemented!(),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), LexError> {
        let tokens = lex("1 + 1;")?;
        assert_eq!(
            tokens,
            vec!(
                Token::int(1, Loc(0, 1)),
                Token::add(Loc(2, 3)),
                Token::int(1, Loc(4, 5)),
                Token::eof(Loc(5, 6)),
            )
        );
        Ok(())
    }
    #[test]
    fn test_2() -> Result<(), LexError> {
        let tokens = lex("(1-1)*1/1;")?;
        assert_eq!(
            tokens,
            vec!(
                Token::lparen(Loc(0, 1)),
                Token::int(1, Loc(1, 2)),
                Token::sub(Loc(2, 3)),
                Token::int(1, Loc(3, 4)),
                Token::rparen(Loc(4, 5)),
                Token::mul(Loc(5, 6)),
                Token::int(1, Loc(6, 7)),
                Token::quo(Loc(7, 8)),
                Token::int(1, Loc(8, 9)),
                Token::eof(Loc(9, 10)),
            )
        );
        Ok(())
    }
    #[test]
    fn test_3() -> Result<(), LexError> {
        let tokens = lex("1>1>=1==1<=1<1!=1")?;
        assert_eq!(
            tokens,
            vec!(
                Token::int(1, Loc(0, 1)),
                Token::grt(Loc(1, 2)),
                Token::int(1, Loc(2, 3)),
                Token::geq(Loc(3, 5)),
                Token::int(1, Loc(5, 6)),
                Token::eq(Loc(6, 8)),
                Token::int(1, Loc(8, 9)),
                Token::leq(Loc(9, 11)),
                Token::int(1, Loc(11, 12)),
                Token::lss(Loc(12, 13)),
                Token::int(1, Loc(13, 14)),
                Token::neq(Loc(14, 16)),
                Token::int(1, Loc(16, 17)),
            )
        );
        Ok(())
    }
    #[test]
    fn test_4() -> Result<(), LexError> {
        let tokens = lex("return 1;")?;
        assert_eq!(
            tokens,
            vec!(
                Token::return_(Loc(0, 6)),
                Token::int(1, Loc(7, 8)),
                Token::eof(Loc(8, 9)),
            )
        );
        Ok(())
    }
    #[test]
    fn test_5() -> Result<(), LexError> {
        let tokens = lex("a = 1;")?;
        assert_eq!(
            tokens,
            vec!(
                Token::ident("a", Loc(0, 1)),
                Token::assign(Loc(2, 3)),
                Token::int(1, Loc(4, 5)),
                Token::eof(Loc(5, 6)),
            )
        );
        Ok(())
    }
}
