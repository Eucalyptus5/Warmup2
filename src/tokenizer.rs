#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Identifier(String),
    Assignment,
    Semicolon,
    Var,
    Computation,
    OpenParen,
    CloseParen,
    EOF,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input, pos: 0 }
    }

    pub fn peek(&self) -> char {
        self.input.chars().nth(self.pos).unwrap_or('\0')
    }

    pub fn peek_token(&mut self) -> Token {
        let current_pos = self.pos;
        let next_token = self.next_token();
        self.pos = current_pos;
        next_token
    }

    fn next_char(&mut self) -> char {
        let c = self.input.chars().nth(self.pos).unwrap_or('\0');
        self.pos += 1;

        c
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while test(self.peek()) && self.peek() != '\0' {
            result.push(self.next_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn tokenize_number(&mut self) -> Token {
        let number_str = self.consume_while(|c| c.is_digit(10));
        Token::Number(number_str.parse::<i32>().unwrap())
    }

    fn tokenize_identifier_or_keyword(&mut self) -> Token {
        let identifier = self.consume_while(|c| c.is_alphanumeric() || c == '_');
        match identifier.as_str() {
            "var" => Token::Var,
            "computation" => Token::Computation,
            _ => Token::Identifier(identifier),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        let c = self.peek();
        match c {
            '0'..='9' => self.tokenize_number(),
            '+' => {
                self.next_char();
                Token::Plus
            }
            '-' => {
                self.next_char();
                Token::Minus
            }
            '*' => {
                self.next_char();
                Token::Star
            }
            '/' => {
                self.next_char();
                Token::Slash
            }
            ';' => {
                self.next_char();
                Token::Semicolon
            }
            '(' => {
                self.next_char();
                Token::OpenParen
            }
            ')' => {
                self.next_char();
                Token::CloseParen
            }
            '<' => {
                self.next_char();
                if self.peek() == '-' {
                    self.next_char();
                    Token::Assignment
                } else {
                    panic!("Unexpected character after '<': {}", self.peek());
                }
            }
            _ if c.is_alphabetic() => self.tokenize_identifier_or_keyword(),
            '.' => Token::EOF,
            _ => panic!("Unexpected character: {}", c),
        }
    }
}