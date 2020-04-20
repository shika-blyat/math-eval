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
    pub fn parse(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];
        loop {
            match self.next_token()? {
                tok
                @
                Token {
                    kind: TokenKind::EOF,
                    ..
                } => {
                    tokens.push(tok);
                    return Ok(tokens);
                }
                tok => tokens.push(tok),
            }
        }
    }
    fn next_token(&mut self) -> Result<Token, Error> {
        while !self.is_empty() {
            let curr_c = self.curr_c();
            if curr_c.is_ascii_digit() {
                return Ok(self.num());
            } else if curr_c.is_whitespace() {
                if curr_c == '\n' {
                    self.line += 1;
                    // self.advance() call will add one to `char_`
                    self.char_ = 0;
                }
                self.advance();
                continue;
            }
            let tok = match curr_c {
                '+' => self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Add,
                        prec: 5,
                    }),
                    '+'.to_string(),
                ),
                '-' => self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Sub,
                        prec: 5,
                    }),
                    '-'.to_string(),
                ),
                '*' => self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Mul,
                        prec: 10,
                    }),
                    '*'.to_string(),
                ),
                '/' => self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::Div,
                        prec: 10,
                    }),
                    '/'.to_string(),
                ),
                'm' => self.new_token(
                    TokenKind::Op(Op {
                        kind: OpKind::USub,
                        prec: 20,
                    }),
                    '-'.to_string(),
                ),
                '(' => self.new_token(TokenKind::LParen, '('.to_string()),
                ')' => self.new_token(TokenKind::RParen, ')'.to_string()),
                c => {
                    return Err(Error {
                        reason: ErrorReason::UnexpectedChar(c),
                        line: self.line,
                        char_: self.char_,
                        range: 0..1,
                    });
                }
            };
            self.advance();
            return Ok(tok);
        }
        Ok(Token {
            kind: TokenKind::EOF,
            line: self.line,
            char_: self.char_,
            range: 0..0,
            lexeme: "".to_string(),
        })
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
            char_: self.char_ - num.len(),
            range: self.current - num.len()..self.current,
            lexeme: num,
        }
    }
    fn new_token<S: Into<String>>(&self, kind: TokenKind, lexeme: S) -> Token {
        Token {
            kind,
            line: self.line,
            char_: self.char_ - 1,
            lexeme: lexeme.into(),
            range: self.current..self.current + 1,
        }
    }
    fn is_empty(&self) -> bool {
        self.current >= self.chars.len()
    }
    fn advance(&mut self) {
        self.char_ += 1;
        self.current += 1;
    }
    fn curr_c(&mut self) -> char {
        self.chars[self.current]
    }
}
