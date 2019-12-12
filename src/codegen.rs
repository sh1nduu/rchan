use super::parse::*;

pub fn gen_lval(node: &mut Option<Box<Node>>) {
    if let Some(n) = node {
        if n.kind != NodeKind::LocalVariable {
            panic!("Left value is not variable");
        }
        print!("  mov rax, rbp\n");
        print!("  sub rax, {}\n", n.offset.unwrap());
        print!("  push rax\n");
    }
}

pub fn gen(node: &mut Option<Box<Node>>) {
    if let Some(n) = node {
        match n.kind {
            NodeKind::Return => {
                gen(&mut n.lhs);
                print!("  pop rax\n");
                print!("  mov rsp, rbp\n");
                print!("  pop rbp\n");
                print!("  ret\n");
                return;
            }
            NodeKind::Num => {
                print!("  push {}\n", n.val.unwrap());
                return;
            }
            NodeKind::LocalVariable => {
                gen_lval(node);
                print!("  pop rax\n");
                print!("  mov rax, [rax]\n");
                print!("  push rax\n");
                return;
            }
            NodeKind::Assign => {
                gen_lval(&mut n.lhs);
                gen(&mut n.rhs);
                print!("  pop rdi\n");
                print!("  pop rax\n");
                print!("  mov [rax], rdi\n");
                print!("  push rdi\n");
                return;
            }
            _ => (),
        }

        gen(&mut n.lhs);
        gen(&mut n.rhs);

        print!("  pop rdi\n");
        print!("  pop rax\n");
        match n.kind {
            NodeKind::Add => print!("  add rax, rdi\n"),
            NodeKind::Sub => print!("  sub rax, rdi\n"),
            NodeKind::Mul => print!("  imul rax, rdi\n"),
            NodeKind::Div => {
                print!("  cqo\n");
                print!("  idiv rdi\n");
            }
            NodeKind::Equal => {
                print!("  cmp rax, rdi\n");
                print!("  sete al\n");
                print!("  movzb rax, al\n");
            }
            NodeKind::NotEqual => {
                print!("  cmp rax, rdi\n");
                print!("  setne al\n");
                print!("  movzb rax, al\n");
            }
            NodeKind::LessThan => {
                print!("  cmp rax, rdi\n");
                print!("  setl al\n");
                print!("  movzb rax, al\n");
            }
            NodeKind::LessThanEqual => {
                print!("  cmp rax, rdi\n");
                print!("  setle al\n");
                print!("  movzb rax, al\n");
            }
            _ => panic!("Unexpected Node"),
        }
        print!("  push rax\n");
    }
}
