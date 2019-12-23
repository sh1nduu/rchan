#[derive(Debug, PartialEq)]
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
    EQ,            // ==
    LEQ,           // <=
    GEQ,           // >=
    LSS,           // <
    GRT,           // >
}

#[derive(Debug, PartialEq)]
pub struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Self) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

#[derive(Debug, PartialEq)]
pub struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

type Token = Annot<TokenKind>;

impl Token {
    fn new_ident(s: &str, loc: Loc) -> Self {
        Self::new(TokenKind::Ident(s.to_string()), loc)
    }
    fn new_int(n: i32, loc: Loc) -> Self {
        Self::new(TokenKind::Int(n), loc)
    }
    fn new_return(loc: Loc) -> Self {
        Self::new(TokenKind::Return, loc)
    }
    fn new_eof(loc: Loc) -> Self {
        Self::new(TokenKind::Eof, loc)
    }
    fn new_add(loc: Loc) -> Self {
        Self::new(TokenKind::Add, loc)
    }
    fn new_sub(loc: Loc) -> Self {
        Self::new(TokenKind::Sub, loc)
    }
    fn new_mul(loc: Loc) -> Self {
        Self::new(TokenKind::Mul, loc)
    }
    fn new_quo(loc: Loc) -> Self {
        Self::new(TokenKind::Quo, loc)
    }
    fn new_lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn new_rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
    fn new_eq(loc: Loc) -> Self {
        Self::new(TokenKind::EQ, loc)
    }
    fn new_leq(loc: Loc) -> Self {
        Self::new(TokenKind::LEQ, loc)
    }
    fn new_geq(loc: Loc) -> Self {
        Self::new(TokenKind::GEQ, loc)
    }
    fn new_lss(loc: Loc) -> Self {
        Self::new(TokenKind::LSS, loc)
    }
    fn new_grt(loc: Loc) -> Self {
        Self::new(TokenKind::GRT, loc)
    }
}

#[derive(Debug)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn new_invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn new_eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

pub struct Lexer;

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
            ' ' => pos += 1,
            c if is_number(c) => lex_a_token!(lex_int(&input, pos)),
            '=' | '<' | '>' if input[pos + 1] == '=' => match c {
                '=' => lex_a_token!(lex_eq(&input, pos)?),
                '<' => lex_a_token!(lex_leq(&input, pos)?),
                '>' => lex_a_token!(lex_geq(&input, pos)?),
                _ => unimplemented!(),
            },
            '+' => lex_a_token!(lex_add(&input, pos)?),
            '-' => lex_a_token!(lex_sub(&input, pos)?),
            '*' => lex_a_token!(lex_mul(&input, pos)?),
            '/' => lex_a_token!(lex_quo(&input, pos)?),
            '(' => lex_a_token!(lex_lparen(&input, pos)?),
            ')' => lex_a_token!(lex_rparen(&input, pos)?),
            '<' => lex_a_token!(lex_lss(&input, pos)?),
            '>' => lex_a_token!(lex_grt(&input, pos)?),
            ';' => lex_a_token!(lex_eof(&input, pos)?),
            _ => unimplemented!(),
        }
        println!("input: {:?}", input);
        println!("c: {:?}", c);
        println!("pos: {:?}", pos);
        println!("{:?}", tokens);
    }

    Ok(tokens)
}

fn is_number(c: char) -> bool {
    c.to_digit(10).is_some()
}

fn consume_string(
    input: &Vec<char>,
    pos: usize,
    expected: &str,
) -> Result<(String, usize), LexError> {
    if input.len() <= pos {
        return Err(LexError::new_eof(Loc(pos, pos)));
    }
    let end = pos + expected.len();
    let input_str: String = input[pos..(end)].into_iter().collect();
    if input_str != expected {
        return Err(LexError::new_invalid_char(input[pos], Loc(pos, end)));
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
    (Token::new_int(n, Loc(start, pos)), pos)
}

fn lex_add(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "+").map(|(_, end)| (Token::new_add(Loc(start, end)), end))
}
fn lex_return(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "return").map(|(_, end)| (Token::new_add(Loc(start, end)), end))
}
fn lex_eof(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, ";").map(|(_, end)| (Token::new_eof(Loc(start, end)), end))
}
fn lex_sub(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "-").map(|(_, end)| (Token::new_sub(Loc(start, end)), end))
}
fn lex_mul(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "*").map(|(_, end)| (Token::new_mul(Loc(start, end)), end))
}
fn lex_quo(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "/").map(|(_, end)| (Token::new_quo(Loc(start, end)), end))
}
fn lex_lparen(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "(").map(|(_, end)| (Token::new_lparen(Loc(start, end)), end))
}
fn lex_rparen(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, ")").map(|(_, end)| (Token::new_rparen(Loc(start, end)), end))
}
fn lex_eq(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "==").map(|(_, end)| (Token::new_eq(Loc(start, end)), end))
}
fn lex_leq(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "<=").map(|(_, end)| (Token::new_leq(Loc(start, end)), end))
}
fn lex_geq(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, ">=").map(|(_, end)| (Token::new_geq(Loc(start, end)), end))
}
fn lex_lss(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, "<").map(|(_, end)| (Token::new_lss(Loc(start, end)), end))
}
fn lex_grt(input: &Vec<char>, start: usize) -> Result<(Token, usize), LexError> {
    consume_string(input, start, ">").map(|(_, end)| (Token::new_grt(Loc(start, end)), end))
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
                Token::new_int(1, Loc(0, 1)),
                Token::new_add(Loc(2, 3)),
                Token::new_int(1, Loc(4, 5)),
                Token::new_eof(Loc(5, 6)),
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
                Token::new_lparen(Loc(0, 1)),
                Token::new_int(1, Loc(1, 2)),
                Token::new_sub(Loc(2, 3)),
                Token::new_int(1, Loc(3, 4)),
                Token::new_rparen(Loc(4, 5)),
                Token::new_mul(Loc(5, 6)),
                Token::new_int(1, Loc(6, 7)),
                Token::new_quo(Loc(7, 8)),
                Token::new_int(1, Loc(8, 9)),
                Token::new_eof(Loc(9, 10)),
            )
        );
        Ok(())
    }
    #[test]
    fn test_3() -> Result<(), LexError> {
        let tokens = lex("1>1>=1==1<=1<1")?;
        assert_eq!(
            tokens,
            vec!(
                Token::new_int(1, Loc(0, 1)),
                Token::new_grt(Loc(1, 2)),
                Token::new_int(1, Loc(2, 3)),
                Token::new_geq(Loc(3, 5)),
                Token::new_int(1, Loc(5, 6)),
                Token::new_eq(Loc(6, 8)),
                Token::new_int(1, Loc(8, 9)),
                Token::new_leq(Loc(9, 11)),
                Token::new_int(1, Loc(11, 12)),
                Token::new_lss(Loc(12, 13)),
                Token::new_int(1, Loc(13, 14)),
            )
        );
        Ok(())
    }
}
