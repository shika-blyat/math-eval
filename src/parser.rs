use crate::tok_ast::{Error, Expr, Lit, Op, OpKind, Token, TokenKind};

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
    'f: while k < tokens.len() {
        let tok = tokens[k].clone();
        match tok.kind {
            TokenKind::Num(n) => ast.push(Expr::Val(Lit::Num(n))),
            TokenKind::Op(op) => {
                while let Some(Op {
                    prec: last_prec,
                    kind,
                }) = op_stack.last()
                {
                    // A little hack
                    if kind == &OpKind::USub {
                        add_op(&mut ast, op_stack.pop().unwrap());
                    } else if last_prec >= &op.prec && kind != &OpKind::LParen {
                        add_op(&mut ast, op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(op);
            }
            TokenKind::LParen => op_stack.push(Op {
                prec: 0,
                kind: OpKind::LParen,
            }),
            TokenKind::RParen => {
                while let Some(op) = op_stack.last() {
                    if let OpKind::LParen = op.kind {
                        op_stack.pop().unwrap();
                        k += 1;
                        continue 'f;
                    }
                    println!("a");
                    add_op(&mut ast, op_stack.pop().unwrap());
                    println!("b");
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
