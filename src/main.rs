mod tokenizer;
mod parser;

use parser::Parser;
fn main() {
    let mut parser = Parser::new("2 + 1 * ((2 + 3) * 4)");
    let result = parser.parse_computation();
    println!("{}", result);

    // let mut tokenizer = Tokenizer::new("var i <- 2 * 3; var abracadabra <- 7; (((abracadabra * i))); i - 5 - 1;");
    // loop {
    //     let token = tokenizer.next_token();
    //     println!("{:?}", token);
    //     if token == Token::EOF {
    //         break;
    //     }
    // }
}
