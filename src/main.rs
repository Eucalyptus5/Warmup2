mod tokenizer;
mod parser;

use parser::Parser;
fn main() {
    //let mut parser = Parser::new("Computation var i <- 2; var x <- 1 * 3 + i; i + x * ((2 + 3) * 4).");
    let mut parser = Parser::new("computation var i <- 1; i + 1.");
    let result = parser.parse_computation();
    println!("{}", result);
}
