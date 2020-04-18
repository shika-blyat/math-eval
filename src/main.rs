mod lexer;
mod parser;
mod tok_ast;
use lexer::Lexer;
use parser::shunting_yard;

fn main() {
    let lexer = Lexer::new("(12 + 2) * 3");
    let tokens = lexer.parse();
    println!("{:#?}", tokens);
    let tokens = tokens.unwrap();
    println!("{:#?}", shunting_yard(tokens));
}
