use std::io::{self};
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let mut input = String::new();
    println!("Enter Expression: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut input = input.trim().chars().peekable();
    let result = parse_computation(&mut input);
    println!("Result: {}", result);
}

fn parse_computation(input: &mut Peekable<Chars>) -> i32 {
    let result = parse_expression(input);
    match_char(input, '.');
    result
}

fn parse_expression(input: &mut Peekable<Chars>) -> i32 {
    let mut result = parse_term(input);
    loop {
        let next_char = input.peek();
        match next_char {
            Some('+') => {
                input.next();
                result += parse_term(input);
            }
            Some('-') => {
                input.next();
                result -= parse_term(input);
            }
            Some(' ') => {
                skip_whitespace(input);
            }
            _ => break,
        }
    }

    result
}

fn parse_term(input: &mut Peekable<Chars>) -> i32 {
    let mut result = parse_factor(input);
    loop {
        let next_char = input.peek();
        match next_char {
            Some('*') => {
                input.next();
                result *= parse_factor(input);
            }
            Some('/') => {
                input.next();
                let divisor = parse_factor(input);
                if divisor == 0 {
                    panic!("Divide by zero error");
                }
                result /= divisor;
            }
            Some(' ') => {
                skip_whitespace(input);
            }
            _ => break,
        }
    }

    result
}

fn parse_factor(input: &mut Peekable<Chars>) -> i32 {
    skip_whitespace(input);
    if let Some(&next_char) = input.peek() {
        match next_char {
            '(' => {
                input.next();
                let result = parse_expression(input);
                skip_whitespace(input);
                match_char(input, ')');
                result
            }
            _ => parse_number(input),
        }
    } else {
        panic!("Unexpected end");
    }
}

fn parse_number(input: &mut Peekable<Chars>) -> i32 {
    let mut number = 0;
    // Loop while the next character is a digit, consuming only digits.
    while let Some('0'..='9') = input.peek() {
        // Safely unwrap the digit now, knowing it's there and is a digit.
        if let Some(digit_char) = input.next() {
            // Convert the character to a digit and update the number.
            let digit = digit_char.to_digit(10).unwrap() as i32;
            number = number * 10 + digit;
        }
    }
    number
}

fn match_char(input: &mut Peekable<Chars>, expected: char) {
    match input.next() {
        Some(c) if c == expected => (),
        _ => panic!("Expected '{}' but found something else.", expected),
    }
}

fn skip_whitespace(input: &mut Peekable<Chars>) {
    while let Some(next_char) = input.peek() {
        if next_char.is_whitespace() {
            input.next();
        } else {
            break;
        }
    }
}
