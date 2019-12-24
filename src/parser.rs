use super::lexer::*;

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Int(i32),
    LocalVariable(String),
    Assign,
    Return,
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
    Plus,  // +
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
    fn new_lvar(name: &str, loc: Loc) -> Self {
        Self::new(NodeKind::LocalVariable(name), loc)
    }
    fn new_return(loc: Loc) -> Self {
        Self::new(NodeKind::Return(n), loc)
    }
    fn new_uniop(op: UniOp, e: Node, loc: Loc) -> Self {
        Self::new(NodeKind::UniOp { op, e: Box::new(e) }, loc)
    }
    fn new_binop(op: BinOp, lhs: Node, rhs: ,loc: Loc) -> Self {
        Self::new(NodeKind::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) }, loc)
    }
}

type UniOp = Annot<UniOpKind>;
impl UniOp {
    fn new_plus(loc: Loc) -> Self {
        Self::new(UniOpKind::Plus, loc)
    }
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

pub enum ParseError {
    Unexpected(Token)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let mut code = Vec::new();
    loop {
        match tokens.peek() {
            Some(_) => unimplemented!(),
            None => return Ok(code),
        }
    }
}