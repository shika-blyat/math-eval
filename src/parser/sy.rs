use super::tok_ast::{Error, Expr, Lit, Op, OpKind, Token, TokenKind};

fn add_op(ast: &mut Vec<Expr>, op: Op) {
    match &op.kind {
        OpKind::USub => {
            let val = Box::new(ast.pop().unwrap());
            ast.push(Expr::UnOp { val, op });
        }
        _ => {
            let roperand = ast.pop().unwrap();
            let loperand = ast.pop().unwrap();
            ast.push(Expr::BinOp {
                left: Box::new(loperand),
                op,
                right: Box::new(roperand),
            });
        }
    }
}

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Expr, Error> {
    let mut op_stack: Vec<Op> = vec![];
    let mut ast: Vec<Expr> = vec![];
    let mut k = 0;
    let mut last_was_op = true;
    'f: while k < tokens.len() {
        let tok = tokens[k].clone();
        match tok.kind {
            TokenKind::Num(n) => {
                ast.push(Expr::Val(Lit::Num(n)));
                last_was_op = false;
            }
            TokenKind::Op(mut op) => {
                if last_was_op {
                    op = Op {
                        kind: OpKind::USub,
                        prec: 20,
                    }
                }
                while let Some(Op {
                    prec: last_prec,
                    kind: last_kind,
                }) = op_stack.last()
                {
                    if last_prec >= &op.prec && last_kind != &OpKind::LParen {
                        add_op(&mut ast, op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(op);
                last_was_op = true;
            }
            TokenKind::LParen => {
                last_was_op = true;
                op_stack.push(Op {
                    prec: 0,
                    kind: OpKind::LParen,
                })
            }
            TokenKind::RParen => {
                last_was_op = false;
                while let Some(op) = op_stack.last() {
                    if let OpKind::LParen = op.kind {
                        op_stack.pop().unwrap();
                        k += 1;
                        continue 'f;
                    }
                    add_op(&mut ast, op_stack.pop().unwrap());
                }
                panic!("Something wrong happened, probably a mismatched parenthesis")
            }
            TokenKind::EOF => (),
        }
        k += 1;
    }
    for op in op_stack.into_iter().rev() {
        add_op(&mut ast, op);
    }
    Ok(ast.into_iter().next().unwrap())
}
