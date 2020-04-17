use std::ops::Range;
#[derive(Debug)]
pub enum TokenKind {
    Op(Op),
    Num(isize),
}
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub char_: usize,
    pub range: Range<usize>,
    pub lexeme: String,
}
#[derive(Debug, Clone)]
pub enum ErrorReason {
    UnexpectedChar(char),
    NotPrefixOp(OpKind),
    UnexpectedEof,
}
#[derive(Debug, Clone)]
pub struct Error {
    pub reason: ErrorReason,
    pub line: usize,
    pub char_: usize,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub enum Expr {
    BinOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Val(Lit),
}

#[derive(Debug)]
pub enum Lit {
    Num(isize),
}

#[derive(Debug)]
pub struct Op {
    pub kind: OpKind,
    pub prec: usize,
}
#[derive(Debug, Clone)]
pub enum OpKind {
    Add,
    Mul,
    Sub,
    Div,
}
