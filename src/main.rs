pub mod evaluation;
pub mod parser;

use evaluation::eval_math::eval_math;
use parser::parse;

fn main() {
    let tree = parse("-12 - 3 * 2");
    println!("{:#?}", tree);
    let tree = tree.unwrap();
    println!("{:#?}", eval_math(&tree));
}
