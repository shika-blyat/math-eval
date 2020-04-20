use crate::parser::tok_ast::{Error, Expr, Lit, Op, OpKind};

pub fn eval_math(e: &Expr) -> isize {
    match e {
        Expr::Val(Lit::Num(n)) => *n,
        Expr::UnOp { op, val } => match op.kind {
            OpKind::USub => -eval_math(val),
            _ => unreachable!(),
        },
        Expr::BinOp { op, left, right } => match op.kind {
            OpKind::Sub => eval_math(left) - eval_math(right),
            OpKind::Add => eval_math(left) + eval_math(right),
            OpKind::Div => eval_math(left) / eval_math(right),
            OpKind::Mul => eval_math(left) * eval_math(right),
            _ => unreachable!(),
        },
    }
}
