use super::parser::*;

pub fn code_gen(code: Vec<Node>) {
    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");

    print!("  push rbp\n");
    print!("  mov rbp, rsp\n");
    print!("  sub rsp, 208\n");

    for node in code {
        gen(node);
        print!("  pop rax\n");
    }

    print!("  mov rsp, rbp\n");
    print!("  pop rbp\n");
    print!("  ret\n");
}

fn gen(node: Node) {
    match node.value {
        NodeKind::Return(expr) => {
            gen(*expr);
            print!("  pop rax\n");
            print!("  mov rsp, rbp\n");
            print!("  pop rbp\n");
            print!("  ret\n");
        }
        NodeKind::Int(n) => {
            print!("  push {}\n", n);
        }
        NodeKind::LocalVariable(_) => {
            gen_lval(node);
            print!("  pop rax\n");
            print!("  mov rax, [rax]\n");
            print!("  push rax\n");
        }
        NodeKind::Assign { lhs, rhs } => {
            gen_lval(*lhs);
            gen(*rhs);
            print!("  pop rdi\n");
            print!("  pop rax\n");
            print!("  mov [rax], rdi\n");
            print!("  push rdi\n");
        }
        NodeKind::BinOp { op, lhs, rhs } => {
            gen(*lhs);
            gen(*rhs);
            print!("  pop rdi\n");
            print!("  pop rax\n");
            match op.value {
                BinOpKind::Add => print!("  add rax, rdi\n"),
                BinOpKind::Sub => print!("  sub rax, rdi\n"),
                BinOpKind::Mul => print!("  imul rax, rdi\n"),
                BinOpKind::Quo => {
                    print!("  cqo\n");
                    print!("  idiv rdi\n");
                }
                BinOpKind::EQ => {
                    print!("  cmp rax, rdi\n");
                    print!("  sete al\n");
                    print!("  movzb rax, al\n");
                }
                BinOpKind::NEQ => {
                    print!("  cmp rax, rdi\n");
                    print!("  setne al\n");
                    print!("  movzb rax, al\n");
                }
                BinOpKind::LSS => {
                    print!("  cmp rax, rdi\n");
                    print!("  setl al\n");
                    print!("  movzb rax, al\n");
                }
                BinOpKind::LEQ => {
                    print!("  cmp rax, rdi\n");
                    print!("  setle al\n");
                    print!("  movzb rax, al\n");
                }
            }
            print!("  push rax\n");
        }
    }
}

fn gen_lval(node: Node) {
    match node.value {
        NodeKind::LocalVariable(offset) => {
            print!("  mov rax, rbp\n");
            print!("  sub rax, {}\n", offset);
            print!("  push rax\n");
        }
        _ => panic!(),
    }
}
