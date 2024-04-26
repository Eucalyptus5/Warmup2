/// Token types representing different elements of a simple programming language.
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Plus,
    Minus,
    Times,
    Divide,
    Assignment,
    Semicolon,
    OpenParen,
    CloseParen,
    Variable,
    Computation,
    EOF,
}

/// A tokenizer that converts a string input into a series of tokens.
pub struct Tokenizer {
    input: Vec<u8>,
    pos: usize,
}

impl Tokenizer {
    /// Create a new tokenizer with the provided input string.
    pub fn new(input_string: String) -> Self {
        Tokenizer { 
            input: input_string.into_bytes(), 
            pos: 0 
        }
    }

    /// Peek at the current character without advancing the tokenizer.
    pub fn peek_char(&self) -> char {
        self.input[self.pos] as char
    }

    /// Peek at the next token without advancing the tokenizer.
    pub fn peek_token(&mut self) -> Token {
        let current_pos = self.pos;
        let next_token = self.next_token();
        self.pos = current_pos;
        next_token
    }

    /// Advance the tokenizer and return the next character.
    fn next_char(&mut self) -> char {
        let c = self.peek_char();
        self.pos += 1;

        c
    }

    /// Consume characters while the provided function returns true.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while test(self.peek_char()) && self.peek_char() != '\0' {
            result.push(self.next_char());
        }
        result
    }

    /// Consume all whitespace characters from the current position.
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    /// Tokenize a sequence of digits into a Number token.
    fn tokenize_number(&mut self) -> Token {
        let number_str = self.consume_while(|c| c.is_digit(10));
        Token::Number(number_str.parse::<i32>().unwrap())
    }

    /// Tokenize an identifier or a keyword into the appropriate Token type.
    fn tokenize_identifier_or_keyword(&mut self) -> Token {
        let identifier = self.consume_while(|c| c.is_alphanumeric() || c == '_');
        match identifier.as_str() {
            "var" => Token::Variable,
            "computation" => Token::Computation,
            _ => Token::Identifier(identifier),
        }
    }

    /// Retrieve the next token from the input, advancing the tokenizer.
    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        let c = self.peek_char();
        match c {
            '0'..='9' => self.tokenize_number(),
            '+' => { self.next_char(); Token::Plus }
            '-' => { self.next_char(); Token::Minus }
            '*' => { self.next_char(); Token::Times }
            '/' => { self.next_char(); Token::Divide }
            ';' => { self.next_char(); Token::Semicolon }
            '(' => { self.next_char(); Token::OpenParen }
            ')' => { self.next_char(); Token::CloseParen }
            '<' => {
                self.next_char();
                if self.peek_char() == '-' {
                    self.next_char();
                    Token::Assignment
                } else {
                    panic!("Unexpected character after '<': {}", self.peek_char());
                }
            }
            _ if c.is_alphabetic() => self.tokenize_identifier_or_keyword(),
            '.' => Token::EOF,
            _ => panic!("Unexpected character: {}", c),
        }
    }
}