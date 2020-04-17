use crate::tok_ast::{Error, ErrorReason, Op, OpKind, Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    current: usize,
    char_: usize,
    line: usize,
    pub error: Option<Error>,
}

impl Lexer {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self {
            chars: s.into().chars().collect(),
            current: 0,
            char_: 1,
            line: 0,
            error: None,
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        while !self.is_empty() {
            let curr_c = self.curr_c();
            if curr_c.is_ascii_digit() {
                return Some(self.num());
            } else if curr_c.is_whitespace() {
                if curr_c == '\n' {
                    self.line += 1;
                    self.char_ = 1;
                } else {
                    self.char_ += 1;
                }
                self.advance();
                continue;
            }
            let tok = match curr_c {
                '+' => Some(self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Add,
                        prec: 5,
                    }),
                    '+'.to_string(),
                )),
                '-' => Some(self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Sub,
                        prec: 5,
                    }),
                    '-'.to_string(),
                )),
                '*' => Some(self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Mul,
                        prec: 10,
                    }),
                    '*'.to_string(),
                )),
                '/' => Some(self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Div,
                        prec: 10,
                    }),
                    '/'.to_string(),
                )),
                c => {
                    self.error = Some(Error {
                        reason: ErrorReason::UnexpectedChar(c),
                        line: self.line,
                        char_: self.char_,
                        range: 0..1,
                    });
                    None
                }
            };
            self.advance();
            return tok;
        }
        None
    }
    fn num(&mut self) -> Token {
        let mut num = self.curr_c().to_string();
        self.advance();
        let mut curr_c;
        while !self.is_empty() {
            curr_c = self.curr_c();
            if !curr_c.is_ascii_digit() {
                break;
            }
            num.push(curr_c);
            self.advance();
        }
        Token {
            kind: TokenKind::Num(num.clone().parse().unwrap()),
            line: self.line,
            char_: self.char_,
            range: self.current - num.len()..self.current,
            lexeme: num,
        }
    }
    pub fn contains_err(&self) -> bool {
        self.error.is_some()
    }
    fn new_token<S: Into<String>>(&self, kind: TokenKind, lexeme: S) -> Token {
        Token {
            kind,
            line: self.line,
            char_: self.char_,
            lexeme: lexeme.into(),
            range: self.current - 1..self.current,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.current >= self.chars.len()
    }
    fn advance(&mut self) {
        self.current += 1;
    }
    fn curr_c(&mut self) -> char {
        self.chars[self.current]
    }
}

impl Iterator for Lexer {
    type Item = (Token, Option<Error>);
    fn next(&mut self) -> Option<(Token, Option<Error>)> {
        match self.next_token() {
            Some(tok) => Some((tok, self.error.clone())),
            None => None,
        }
    }
}
