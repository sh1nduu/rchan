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
pub enum NodeKind {
    Add,           // +
    Sub,           // -
    Mul,           // *
    Div,           // /
    Equal,         // ==
    NotEqual,      // !=
    LessThan,      // <
    LessThanEqual, // <=
    Num,           // Integer
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i64>,
}

impl Node {
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Node {
        Node {
            kind: kind,
            lhs: Some(lhs),
            rhs: Some(rhs),
            val: None,
        }
    }

    pub fn new_node_num(val: i64) -> Node {
        Node {
            kind: NodeKind::Num,
            lhs: None,
            rhs: None,
            val: Some(val),
        }
    }
}

pub fn expr(token: &mut Option<Box<Token>>) -> Box<Node> {
    equality(token)
}

fn equality(token: &mut Option<Box<Token>>) -> Box<Node> {
    let mut node = relational(token);
    loop {
        if consume("==", token) {
            node = Box::new(Node::new_node(NodeKind::Equal, node, relational(token)))
        } else if consume("!=", token) {
            node = Box::new(Node::new_node(NodeKind::NotEqual, node, relational(token)))
        } else {
            return node;
        }
    }
}

fn relational(token: &mut Option<Box<Token>>) -> Box<Node> {
    let mut node = add(token);
    loop {
        if consume("<=", token) {
            node = Box::new(Node::new_node(NodeKind::LessThanEqual, node, add(token)))
        } else if consume(">=", token) {
            node = Box::new(Node::new_node(NodeKind::LessThanEqual, add(token), node))
        } else if consume("<", token) {
            node = Box::new(Node::new_node(NodeKind::LessThan, node, add(token)))
        } else if consume(">", token) {
            node = Box::new(Node::new_node(NodeKind::LessThan, add(token), node))
        } else {
            return node;
        }
    }
}

fn add(token: &mut Option<Box<Token>>) -> Box<Node> {
    let mut node = mul(token);
    loop {
        if consume("+", token) {
            node = Box::new(Node::new_node(NodeKind::Add, node, mul(token)))
        } else if consume("-", token) {
            node = Box::new(Node::new_node(NodeKind::Sub, node, mul(token)))
        } else {
            return node;
        }
    }
}

fn mul(token: &mut Option<Box<Token>>) -> Box<Node> {
    let mut node = unary(token);
    loop {
        if consume("*", token) {
            node = Box::new(Node::new_node(NodeKind::Mul, node, unary(token)))
        } else if consume("/", token) {
            node = Box::new(Node::new_node(NodeKind::Div, node, unary(token)))
        } else {
            return node;
        }
    }
}

fn unary(token: &mut Option<Box<Token>>) -> Box<Node> {
    if consume("+", token) {
        primary(token)
    } else if consume("-", token) {
        Box::new(Node::new_node(
            NodeKind::Sub,
            Box::new(Node::new_node_num(0)),
            primary(token),
        ))
    } else {
        primary(token)
    }
}

fn primary(token: &mut Option<Box<Token>>) -> Box<Node> {
    if consume("(", token) {
        let node = expr(token);
        expect(")", token);
        return node;
    }
    Box::new(Node::new_node_num(expect_number(token).unwrap()))
}

fn consume(op: &str, token: &mut Option<Box<Token>>) -> bool {
    if is_expected(op, token) {
        next_token(token);
        return true;
    }
    false
}

fn expect(op: &str, token: &mut Option<Box<Token>>) {
    if is_expected(op, token) {
        next_token(token);
    } else {
        panic!("Unexpected char");
    }
}

fn is_expected(op: &str, token: &mut Option<Box<Token>>) -> bool {
    if let Some(t) = token {
        if t.kind != TokenKind::Reserved {
            return false;
        }
        if let Some(s) = &t.string {
            if s.len() != t.len {
                return false;
            }
            if s != op {
                return false;
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

#[cfg(test)]
mod tests {
    use super::super::tokenize::*;
    use super::*;

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
        assert!(consume("+", t1));
        let t2 = &mut tokenize("-").ok().unwrap();
        assert!(consume("-", t2));
    }
}
