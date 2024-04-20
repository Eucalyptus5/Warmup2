use crate::tokenizer::{Token, Tokenizer};
use std::collections::HashMap;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    symbol_table: HashMap<String, i32>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(input),
            symbol_table: HashMap::new(),
        }
    }

    fn parse_assignment(&mut self) {

    }

    fn parse_statement(&mut self) {
        while self.tokenizer.peek_token() == Token::Var {

        }
    }
    
    pub fn parse_computation(&mut self) -> i32 {
        self.match_char(Token::Computation);
        let result = self.parse_expression();
        self.match_char(Token::EOF);
        result
    }

    fn parse_expression(&mut self) -> i32 {
        let mut result = self.parse_term();
        loop {
            let next_token = self.tokenizer.peek_token();
            match next_token {
                Token::Plus => {
                    self.tokenizer.next_token();
                    result += self.parse_term();
                }
                Token::Minus => {
                    self.tokenizer.next_token();
                    result -= self.parse_term();
                }
                _ => break,
            }
        }
    
        result
    }
    
    fn parse_term(&mut self) -> i32 {
        let mut result = self.parse_factor();
        loop {
            let next_token = self.tokenizer.peek_token();
            match next_token {
                Token::Star => {
                    self.tokenizer.next_token();
                    result *= self.parse_factor();
                }
                Token::Slash => {
                    self.tokenizer.next_token();
                    let divisor = self.parse_factor();
                    if divisor == 0 {
                        panic!("Divide by zero error");
                    }
                    result /= divisor;
                }
                _ => break,
            }
        }
    
        result
    }
    
    fn parse_factor(&mut self) -> i32 {
        let next_token = self.tokenizer.peek_token();
        match next_token {
            Token::OpenParen => {
                self.tokenizer.next_token();
                let result = self.parse_expression();
                self.match_char(Token::CloseParen);
                result
            },
            Token::Number(value) => {
                self.tokenizer.next_token();
                value
            }
            Token::Identifier(id) => {
                self.tokenizer.next_token();
                *self.symbol_table.get(&id).unwrap_or_else(|| panic!("Undefined variable: {}", id))
            }
            _ => panic!("Unexpected factor"),
        }
    }
    
    fn match_char(&mut self, expected: Token) {
        match self.tokenizer.next_token() {
            t if t == expected => (),
            _ => panic!("Expected '{:?}' but found something else.", expected),
        }
    }
}