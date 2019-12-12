use super::tokenize::*;

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Add,           // +
    Sub,           // -
    Mul,           // *
    Div,           // /
    Equal,         // ==
    NotEqual,      // !=
    LessThan,      // <
    LessThanEqual, // <=
    Assign,        // =
    LocalVariable, // local variable
    Num,           // Integer
    Return,        // Return
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i64>,
    pub offset: Option<i32>,
}

impl Node {
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Node {
        Node {
            kind: kind,
            lhs: Some(lhs),
            rhs: Some(rhs),
            val: None,
            offset: None,
        }
    }

    pub fn new_node_num(val: i64) -> Node {
        Node {
            kind: NodeKind::Num,
            lhs: None,
            rhs: None,
            val: Some(val),
            offset: None,
        }
    }

    pub fn new_node_identifier(offset: i32) -> Node {
        Node {
            kind: NodeKind::LocalVariable,
            lhs: None,
            rhs: None,
            val: None,
            offset: Some(offset),
        }
    }
    pub fn new_node_return(lhs: Box<Node>) -> Node {
        Node {
            kind: NodeKind::Return,
            lhs: Some(lhs),
            rhs: None,
            val: None,
            offset: None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct LocalVariable {
    name: String,
    offset: i32,
}

impl LocalVariable {
    fn new(token: &mut Option<Box<Token>>, offset: i32) -> Option<Self> {
        if let Some(t) = token {
            if let Some(s) = &t.string {
                return Some(LocalVariable {
                    name: s.clone(),
                    offset: offset,
                });
            }
        }
        None
    }
}

struct LocalVariables {
    list: Vec<LocalVariable>,
    offset: i32,
}

impl LocalVariables {
    fn new() -> Self {
        Self {
            list: Vec::new(),
            offset: 0,
        }
    }
    fn find(&self, token: &mut Option<Box<Token>>) -> Option<&LocalVariable> {
        if let Some(t) = token {
            if let Some(s) = &t.string {
                for lvar in &self.list {
                    if &lvar.name == s {
                        return Some(lvar);
                    }
                }
            }
        }
        None
    }
}

pub struct Parser<'a> {
    pub token: &'a mut Option<Box<Token>>,
    lvars: LocalVariables,
}

impl<'a> Parser<'a> {
    pub fn new(token: &'a mut Option<Box<Token>>) -> Self {
        let lvars = LocalVariables::new();
        Self {
            token: token,
            lvars: lvars,
        }
    }

    pub fn program(&mut self) -> Vec<Box<Node>> {
        let mut code = Vec::new();
        loop {
            match self.token {
                Some(t) => {
                    if t.kind != TokenKind::Eof {
                        code.push(self.stmt());
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        code
    }

    fn stmt(&mut self) -> Box<Node> {
        let node = if consume("return", &mut self.token) {
            Box::new(Node::new_node_return(self.expr()))
        } else {
            self.expr()
        };
        expect(";", &mut self.token);
        node
    }

    fn expr(&mut self) -> Box<Node> {
        self.assign()
    }

    fn assign(&mut self) -> Box<Node> {
        let mut node = self.equality();
        if consume("=", &mut &mut self.token) {
            node = Box::new(Node::new_node(NodeKind::Assign, node, self.assign()));
        }
        node
    }

    fn equality(&mut self) -> Box<Node> {
        let mut node = self.relational();
        loop {
            if consume("==", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::Equal, node, self.relational()))
            } else if consume("!=", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::NotEqual, node, self.relational()))
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Box<Node> {
        let mut node = self.add();
        loop {
            if consume("<=", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::LessThanEqual, node, self.add()))
            } else if consume(">=", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::LessThanEqual, self.add(), node))
            } else if consume("<", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::LessThan, node, self.add()))
            } else if consume(">", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::LessThan, self.add(), node))
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Box<Node> {
        let mut node = self.mul();
        loop {
            if consume("+", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::Add, node, self.mul()))
            } else if consume("-", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::Sub, node, self.mul()))
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Box<Node> {
        let mut node = self.unary();
        loop {
            if consume("*", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::Mul, node, self.unary()))
            } else if consume("/", &mut self.token) {
                node = Box::new(Node::new_node(NodeKind::Div, node, self.unary()))
            } else {
                return node;
            }
        }
    }

    fn unary(&mut self) -> Box<Node> {
        if consume("+", &mut self.token) {
            self.primary()
        } else if consume("-", &mut self.token) {
            Box::new(Node::new_node(
                NodeKind::Sub,
                Box::new(Node::new_node_num(0)),
                self.primary(),
            ))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<Node> {
        if consume("(", &mut self.token) {
            let node = self.expr();
            expect(")", &mut self.token);
            return node;
        }

        if let Some(_) = get_identifier(&mut self.token) {
            let offset = if let Some(lvar) = self.lvars.find(self.token) {
                lvar.offset
            } else {
                let offset = self.lvars.offset + 8;
                let lvar = LocalVariable::new(self.token, offset.clone()).unwrap();
                self.lvars.offset = offset;
                self.lvars.list.push(lvar);
                offset
            };
            next_token(self.token);
            Box::new(Node::new_node_identifier(offset))
        } else {
            Box::new(Node::new_node_num(expect_number(&mut self.token).unwrap()))
        }
    }
}

fn consume(op: &str, token: &mut Option<Box<Token>>) -> bool {
    if is_expected(op, token) {
        next_token(token);
        return true;
    }
    false
}

fn get_identifier(token: &mut Option<Box<Token>>) -> Option<String> {
    if let Some(t) = token {
        if let Some(s) = t.string.clone() {
            if s.chars().all(|x| x.is_ascii_lowercase()) {
                return Some(s.to_string());
            }
        }
    }
    None
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
        match t.kind {
            TokenKind::Reserved | TokenKind::Return => (),
            _ => return false,
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
    #[test]
    fn get_identifier_test() {
        let t1 = &mut tokenize("a").ok().unwrap();
        assert_eq!(get_identifier(t1), Some("a".to_string()));
        let t2 = &mut tokenize("8").ok().unwrap();
        assert_eq!(get_identifier(t2), None);
        let t3 = &mut tokenize("variable").ok().unwrap();
        assert_eq!(get_identifier(t3), Some("variable".to_string()));
    }

    #[test]
    fn local_variable_find_test() {
        let t1 = &mut tokenize("a=1;b=2;").ok().unwrap();
        let mut lvars = LocalVariables::new();
        assert_eq!(lvars.find(t1), None);

        lvars.list.push(LocalVariable {
            name: "a".to_string(),
            offset: 8,
        });
        assert!(lvars.find(t1).is_some());
    }
}
