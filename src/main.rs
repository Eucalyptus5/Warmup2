mod tokenizer;
mod parser;

use parser::Parser;
fn main() {
    let mut parser = Parser::new("Computation 2 + 1 * ((2 + 3) * 4).");
    let result = parser.parse_computation();
    println!("{}", result);
}
