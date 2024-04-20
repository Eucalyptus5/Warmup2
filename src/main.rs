mod tokenizer;
mod parser;

use parser::Parser;
fn main() {
    //let mut parser = Parser::new("Computation var i <- 2; var x <- 1 * 3 + i; i + x * ((2 + 3) * 4).");
    let mut parser = Parser::new("computation var i <- 2 * 3; var abracadabra <- 7; (((abracadabra * i))); i - 5 - 1.");
    parser.parse_computation();
}