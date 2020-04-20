pub mod lexer;
pub mod sy;
pub mod tok_ast;

use lexer::Lexer;
use sy::shunting_yard;
use tok_ast::{Error, Expr};

pub fn parse<S: Into<String>>(s: S) -> Result<Expr, Error> {
    let lexer = Lexer::new(s);
    let tokens = lexer.parse()?;
    shunting_yard(tokens)
}
