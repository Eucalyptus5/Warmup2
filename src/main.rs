mod tokenizer;
use tokenizer::{Tokenizer, Token};

fn main() {
    let mut tokenizer = Tokenizer::new("var x = 100 + 20 - 2;");
    loop {
        let token = tokenizer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}