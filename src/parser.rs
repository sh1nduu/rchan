use super::lexer::*;
use std::cell::RefCell;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Int(i32),
    LocalVariable(i32),
    Assign {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Return(Box<Node>),
    UniOp {
        op: UniOp,
        e: Box<Node>,
    },
    BinOp {
        op: BinOp,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

#[derive(Debug, PartialEq)]
pub enum UniOpKind {
    Minus, // -
}

#[derive(Debug, PartialEq)]
pub enum BinOpKind {
    Add, // +
    Sub, // -
    Mul, // *
    Quo, // /
    EQ,  // ==
    NEQ, // !=
    LSS, // <
    LEQ, // <=
}

type Node = Annot<NodeKind>;
impl Node {
    fn new_int(n: i32, loc: Loc) -> Self {
        Self::new(NodeKind::Int(n), loc)
    }
    fn new_lvar(offset: i32, loc: Loc) -> Self {
        Self::new(NodeKind::LocalVariable(offset), loc)
    }
    fn new_return(node: Node, loc: Loc) -> Self {
        Self::new(NodeKind::Return(Box::new(node)), loc)
    }
    fn new_assign(lhs: Node, rhs: Node, loc: Loc) -> Self {
        Self::new(
            NodeKind::Assign {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            loc,
        )
    }
    fn new_uniop(op: UniOp, e: Node, loc: Loc) -> Self {
        Self::new(NodeKind::UniOp { op, e: Box::new(e) }, loc)
    }
    fn new_binop(op: BinOp, lhs: Node, rhs: Node, loc: Loc) -> Self {
        Self::new(
            NodeKind::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            loc,
        )
    }
}

type UniOp = Annot<UniOpKind>;
impl UniOp {
    fn new_minus(loc: Loc) -> Self {
        Self::new(UniOpKind::Minus, loc)
    }
}

type BinOp = Annot<BinOpKind>;
impl BinOp {
    fn new_add(loc: Loc) -> Self {
        Self::new(BinOpKind::Add, loc)
    }
    fn new_sub(loc: Loc) -> Self {
        Self::new(BinOpKind::Sub, loc)
    }
    fn new_mul(loc: Loc) -> Self {
        Self::new(BinOpKind::Mul, loc)
    }
    fn new_quo(loc: Loc) -> Self {
        Self::new(BinOpKind::Quo, loc)
    }
    fn new_eq(loc: Loc) -> Self {
        Self::new(BinOpKind::EQ, loc)
    }
    fn new_neq(loc: Loc) -> Self {
        Self::new(BinOpKind::NEQ, loc)
    }
    fn new_lss(loc: Loc) -> Self {
        Self::new(BinOpKind::LSS, loc)
    }
    fn new_leq(loc: Loc) -> Self {
        Self::new(BinOpKind::LEQ, loc)
    }
}

#[derive(Debug, PartialEq)]
struct LocalVariable {
    name: String,
}

impl LocalVariable {
    fn new(name: &str) -> Self {
        LocalVariable {
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
struct LocalVariables(Vec<LocalVariable>);
impl LocalVariables {
    fn new() -> Self {
        LocalVariables(Vec::<LocalVariable>::new())
    }
    fn push(&mut self, var: LocalVariable) -> i32 {
        self.0.push(var);
        (self.0.len() as i32) * 8
    }
    fn find_and_get_offset(&mut self, s: &str) -> Option<i32> {
        self.0
            .iter()
            .position(|x| x.name == s)
            .map(|x| (x as i32) * 8)
    }
}

thread_local!(
    static LVARS: RefCell<LocalVariables> = { RefCell::new(LocalVariables::new()) };
);

pub enum ParseError {
    Unexpected(Token),
    NotClosingParen(Token),
    Eof,
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let mut code = Vec::<Node>::new();
    loop {
        match tokens.peek() {
            Some(_) => code.push(stmt(&mut tokens)?),
            None => return Ok(code),
        }
    }
}

fn stmt<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let node = match tokens.peek().map(|t| &t.value) {
        Some(TokenKind::Return) => match tokens.next().unwrap() {
            Token {
                value: TokenKind::Return,
                loc,
            } => Node::new_return(expr(tokens)?, loc),
            _ => unreachable!(),
        },
        _ => expr(tokens)?,
    };
    match tokens.peek() {
        Some(Token {
            value: TokenKind::Eof,
            ..
        }) => Ok(node),
        _ => Err(ParseError::Eof),
    }
}

fn expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    assign(tokens)
}

fn assign<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut node = equality(tokens)?;
    node = match tokens.peek().map(|t| &t.value) {
        Some(TokenKind::ASSIGN) => match tokens.next().unwrap() {
            Token {
                value: TokenKind::ASSIGN,
                loc,
            } => Node::new_assign(node, assign(tokens)?, loc),
            _ => unreachable!(),
        },
        _ => node,
    };
    Ok(node)
}

fn equality<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut node = relational(tokens)?;
    loop {
        node = match tokens.peek().map(|t| &t.value) {
            Some(TokenKind::EQ) | Some(TokenKind::NEQ) => match tokens.next().unwrap() {
                Token {
                    value: TokenKind::EQ,
                    loc,
                } => Node::new_binop(BinOp::new_eq(loc), node, relational(tokens)?, loc),
                Token {
                    value: TokenKind::NEQ,
                    loc,
                } => Node::new_binop(BinOp::new_neq(loc), node, relational(tokens)?, loc),
                _ => unreachable!(),
            },
            _ => return Ok(node),
        };
    }
}

fn relational<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut node = add(tokens)?;
    loop {
        node = match tokens.peek().map(|t| &t.value) {
            Some(TokenKind::LEQ) | Some(TokenKind::GEQ) | Some(TokenKind::LSS)
            | Some(TokenKind::GRT) => match tokens.next().unwrap() {
                Token {
                    value: TokenKind::LEQ,
                    loc,
                } => Node::new_binop(BinOp::new_leq(loc), node, add(tokens)?, loc),
                Token {
                    value: TokenKind::GEQ,
                    loc,
                } => Node::new_binop(BinOp::new_leq(loc), add(tokens)?, node, loc),
                Token {
                    value: TokenKind::LSS,
                    loc,
                } => Node::new_binop(BinOp::new_lss(loc), node, add(tokens)?, loc),
                Token {
                    value: TokenKind::GRT,
                    loc,
                } => Node::new_binop(BinOp::new_lss(loc), add(tokens)?, node, loc),
                _ => unreachable!(),
            },
            _ => return Ok(node),
        };
    }
}

fn add<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut node = mul(tokens)?;
    loop {
        node = match tokens.peek().map(|t| &t.value) {
            Some(TokenKind::Add) | Some(TokenKind::Sub) => match tokens.next().unwrap() {
                Token {
                    value: TokenKind::Add,
                    loc,
                } => Node::new_binop(BinOp::new_add(loc), node, mul(tokens)?, loc),
                Token {
                    value: TokenKind::Sub,
                    loc,
                } => Node::new_binop(BinOp::new_sub(loc), node, mul(tokens)?, loc),
                _ => unreachable!(),
            },
            _ => return Ok(node),
        };
    }
}

fn mul<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut node = unary(tokens)?;
    loop {
        node = match tokens.peek().map(|t| &t.value) {
            Some(TokenKind::Mul) | Some(TokenKind::Quo) => match tokens.next().unwrap() {
                Token {
                    value: TokenKind::Mul,
                    loc,
                } => Node::new_binop(BinOp::new_mul(loc), node, unary(tokens)?, loc),
                Token {
                    value: TokenKind::Quo,
                    loc,
                } => Node::new_binop(BinOp::new_quo(loc), node, unary(tokens)?, loc),
                _ => unreachable!(),
            },
            _ => return Ok(node),
        };
    }
}

fn unary<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.peek().map(|t| &t.value) {
        Some(TokenKind::Add) | Some(TokenKind::Sub) => match tokens.next().unwrap() {
            Token {
                value: TokenKind::Add,
                ..
            } => primary(tokens),
            Token {
                value: TokenKind::Sub,
                loc,
            } => Ok(Node::new_uniop(
                UniOp::new_minus(loc),
                primary(tokens)?,
                loc,
            )),
            _ => unreachable!(),
        },
        _ => primary(tokens),
    }
}

fn primary<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.peek().map(|t| &t.value) {
        Some(TokenKind::LParen) => consume_parens(tokens),
        Some(TokenKind::Ident(_)) => match tokens.next().unwrap() {
            Token {
                value: TokenKind::Ident(s),
                loc,
            } => Ok(find_or_create_local_var(&s, loc)),
            _ => unreachable!(),
        },
        Some(TokenKind::Int(_)) => match tokens.next().unwrap() {
            Token {
                value: TokenKind::Int(n),
                loc,
            } => Ok(Node::new_int(n, loc)),
            _ => unreachable!(),
        },
        _ => Err(ParseError::Unexpected(tokens.next().unwrap())),
    }
}

fn consume_parens<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    tokens.next();
    let node = expr(tokens)?;
    match tokens.peek().map(|t| &t.value) {
        Some(TokenKind::RParen) => {
            tokens.next();
            Ok(node)
        }
        _ => Err(ParseError::NotClosingParen(tokens.next().unwrap())),
    }
}

fn find_or_create_local_var(s: &str, loc: Loc) -> Node {
    LVARS.with(|lvars| match lvars.borrow_mut().find_and_get_offset(&s) {
        Some(offset) => Node::new_lvar(offset, loc),
        None => {
            let offset = lvars.borrow_mut().push(LocalVariable::new(&s));
            Node::new_lvar(offset, loc)
        }
    })
}
