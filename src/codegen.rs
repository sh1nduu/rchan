use super::parse::*;

pub fn gen(node: &mut Option<Box<Node>>) {
    if let Some(n) = node {
        if n.kind == NodeKind::Num {
            println!("  push {}\n", n.val.unwrap());
            return;
        }

        gen(&mut n.lhs);
        gen(&mut n.rhs);

        println!("  pop rdi\n");
        println!("  pop rax\n");
        match n.kind {
            NodeKind::Add => println!("  add rax, rdi\n"),
            NodeKind::Sub => println!("  sub rax, rdi\n"),
            NodeKind::Mul => println!("  imul rax, rdi\n"),
            NodeKind::Div => {
                println!("  cqo\n");
                println!("  idiv rdi\n");
            }
            NodeKind::Equal => {
                println!("  cmp rax, rdi\n");
                println!("  sete al\n");
                println!("  movzb rax, al\n");
            }
            NodeKind::NotEqual => {
                println!("  cmp rax, rdi\n");
                println!("  setne al\n");
                println!("  movzb rax, al\n");
            }
            NodeKind::LessThan => {
                println!("  cmp rax, rdi\n");
                println!("  setl al\n");
                println!("  movzb rax, al\n");
            }
            NodeKind::LessThanEqual => {
                println!("  cmp rax, rdi\n");
                println!("  setle al\n");
                println!("  movzb rax, al\n");
            }
            _ => panic!("Unexpected Node"),
        }
        println!("  push rax\n");
    }
}
