/// Import required modules and types from external crates and other parts of the program.
use crate::tokenizer::{Token, Tokenizer};
use std::collections::HashMap;

/// Parser struct which uses a tokenizer for token processing and a symbol table for variable management.
pub struct Parser {
    tokenizer: Tokenizer,
    symbol_table: HashMap<String, i32>,
}

impl Parser {
    /// Constructs a new Parser with the given input string.
    pub fn new(input: String) -> Self {
        Parser {
            tokenizer: Tokenizer::new(input),
            symbol_table: HashMap::new(),
        }
    }

    /// Parses an assignment statement, expecting an identifier followed by an assignment token and an expression.
    fn parse_assignment(&mut self) {
        match self.tokenizer.next_token() {
            Token::Identifier(name) => {
                self.match_char(Token::Assignment);
                let value = self.parse_expression();
                self.symbol_table.insert(name, value);
            }
            _ => panic!("Expected identifier"),
        }
        self.match_char(Token::Semicolon);
    }

    /// Parses statements, specifically variable declarations followed by assignments.
    fn parse_statement(&mut self) {
        while self.tokenizer.peek_token() == Token::Variable {
            self.tokenizer.next_token();
            self.parse_assignment();
        }
    }

    /// Evaluates multiple expressions, printing their results, until it encounters an EOF or unexpected token.
    fn multiple_expression(&mut self) {
        loop {
            println!("{}", self.parse_expression());
            match self.tokenizer.next_token() {
                Token::Semicolon => (),
                Token::EOF => break,
                _ => panic!(
                    "Unexpected end to expression, symbol: {:?}",
                    self.tokenizer.peek_token()
                ),
            }
        }
    }

    /// Parses the computation section of a script, starting with a computation token, followed by statements and expressions.
    pub fn parse_computation(&mut self) {
        self.match_char(Token::Computation);
        self.parse_statement();
        self.multiple_expression();
    }

    /// Parses an expression, which can consist of terms added or subtracted together.
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

    /// Parses a term, which can consist of factors multiplied or divided.
    fn parse_term(&mut self) -> i32 {
        let mut result = self.parse_factor();
        loop {
            let next_token = self.tokenizer.peek_token();
            match next_token {
                Token::Times => {
                    self.tokenizer.next_token();
                    result *= self.parse_factor();
                }
                Token::Divide => {
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

    /// Parses a factor, which can be a number, a variable, or a nested expression within parentheses.
    fn parse_factor(&mut self) -> i32 {
        let next_token = self.tokenizer.peek_token();
        match next_token {
            Token::OpenParen => {
                self.tokenizer.next_token();
                let result = self.parse_expression();
                self.match_char(Token::CloseParen);
                result
            }
            Token::Number(value) => {
                self.tokenizer.next_token();
                value
            }
            Token::Identifier(id) => {
                self.tokenizer.next_token();
                *self
                    .symbol_table
                    .get(&id)
                    .unwrap_or_else(|| panic!("Undefined variable: {}", id))
            }
            _ => panic!("Unexpected factor, got {:?}", next_token),
        }
    }

    /// Ensures that the next token matches the expected token, panics otherwise.
    fn match_char(&mut self, expected: Token) {
        match self.tokenizer.next_token() {
            t if t == expected => (),
            _ => panic!("Expected '{:?}' but found something else.", expected),
        }
    }
}