use std::ops::Range;
#[derive(Debug, Clone)]
pub enum TokenKind {
    Op(Op),
    Num(isize),
    LParen,
    RParen,
    EOF,
}
#[derive(Debug, Clone)]
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
    UnOp {
        op: Op,
        val: Box<Expr>,
    },
    Val(Lit),
}

#[derive(Debug)]
pub enum Lit {
    Num(isize),
}

#[derive(Debug, Clone)]
pub struct Op {
    pub kind: OpKind,
    pub prec: usize,
}
#[derive(Debug, PartialEq, Clone)]
pub enum OpKind {
    Add,
    Mul,
    Sub,
    /// Unary substraction
    USub,
    Div,
    LParen,
    RParen,
}
