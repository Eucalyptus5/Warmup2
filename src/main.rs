mod parser;
mod tokenizer;

use parser::Parser;
fn main() {
    let mut parser = Parser::new(
        "computation var i <- 2 * 3; var abracadabra <- 7; (((abracadabra * i))); i - 5 - 1.".to_string(),
    );
    parser.parse_computation();
}
