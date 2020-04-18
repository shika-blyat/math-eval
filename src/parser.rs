use crate::tok_ast::{Error, Expr, Lit, Op, OpKind, Token, TokenKind};

fn add_infix_op(ast: &mut Vec<Expr>, op: Op) {
    let roperand = ast.pop().unwrap();
    let loperand = ast.pop().unwrap();
    ast.push(Expr::BinOp {
        left: Box::new(loperand),
        op,
        right: Box::new(roperand),
    });
}

fn add_unary_op(ast: &mut Vec<Expr>, op: Op, val: Expr) {
    ast.push(Expr::UnOp {
        val: Box::new(val),
        op,
    });
}
pub fn shunting_yard(tokens: Vec<Token>) -> Result<Expr, Error> {
    let mut op_stack: Vec<Op> = vec![];
    let mut ast: Vec<Expr> = vec![];
    let mut last_was_op = true;
    let mut prefix_op_stack = vec![];
    let mut k = 0;
    'f: while k < tokens.len() {
        let tok = tokens[k].clone();
        match tok.kind {
            TokenKind::Num(mut n) => {
                last_was_op = false;
                loop {
                    match prefix_op_stack.pop() {
                        Some(OpKind::Sub) => n = -n,
                        None => break,
                        _ => unreachable!(),
                    }
                }
                ast.push(Expr::Val(Lit::Num(n)))
            }
            TokenKind::Op(op) => {
                last_was_op = true;
                while let Some(Op {
                    prec: last_prec,
                    kind,
                }) = op_stack.last()
                {
                    // A little hack
                    if let (OpKind::Sub, true) = (kind, last_was_op) {
                        prefix_op_stack.push(OpKind::Sub);
                        continue;
                    }
                    if last_prec >= &op.prec {
                        add_infix_op(&mut ast, op_stack.pop().unwrap());
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
                        op_stack.pop();
                        continue 'f;
                    }
                    add_infix_op(&mut ast, op_stack.pop().unwrap());
                }
                panic!("Something wrong happened, probably a mismatched parenthesis")
            }
            TokenKind::EOF => (),
        }
        k += 1;
    }
    for op in op_stack.into_iter().rev() {
        add_infix_op(&mut ast, op);
    }
    Ok(ast.into_iter().next().unwrap())
}
