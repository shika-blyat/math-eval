mod lexer;
mod parser;
mod tok_ast;
use lexer::Lexer;
fn main() {
    let lexer = Lexer::new("12 + 2 * 3");
    for (i, err) in lexer {
        println!("tok: {:#?}, err: {:#?}", i, err);
    }
}
