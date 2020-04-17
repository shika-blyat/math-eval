use crate::tok_ast::{Error, ErrorReason, Expr, Lit, Op, OpKind, Token, TokenKind};

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Expr, Error> {
    if tokens.is_empty() {
        return Err(Error {
            reason: ErrorReason::UnexpectedEof,
            line: 0,
            char_: 0,
            range: 0..0,
        });
    }
    let mut op_stack: Vec<Op> = vec![];
    let mut output_stack = vec![];
    let mut ast = None;
    for tok in tokens {
        match tok.kind {
            TokenKind::Num(val) => output_stack.push(val),
            TokenKind::Op(op) => {
                while let Some(last_op) = op_stack.last() {
                    if last_op.prec >= op.prec {
                        ast = Some(Expr::BinOp {
                            left: Box::new(Expr::Val(Lit::Num(output_stack.pop().unwrap()))),
                            op: op_stack.pop().unwrap(),
                            right: Box::new(Expr::Val(Lit::Num(output_stack.pop().unwrap()))),
                        });
                    } else {
                        break;
                    }
                }
                op_stack.push(op);
            }
            TokenKind::EOF => (),
        }
    }
    Ok(ast.unwrap())
}
