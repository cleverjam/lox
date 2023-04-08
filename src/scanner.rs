use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::tokens::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a mut BufReader<File>,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a mut BufReader<File>) -> Scanner<'a> {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(mut self) -> Self {
        let mut buf = String::new();
        loop {
            match self.source.read_line(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(_size) => {
                    // print!("[{} bytes] {}:\t {}", size, self.line, buf);
                    self.current = 0;
                    self.start = self.current;
                    self.scan_tokens(&buf);
                }
                Err(_) => {
                    println!("There was an error reading the file, line: {}", self.line);
                    break;
                }
            }

            buf.clear();
            self.line = self.line + 1;
        }

        // EOF
        self.tokens
            .push(Token::new(TokenType::EOF, None, None, self.line));
        self
    }

    fn scan_tokens(&mut self, str: &str) {
        while let Some(c) = self.advance(str) {
            match c {
                '(' => self.add_token(TokenType::LeftParenthesis, None, None),
                ')' => self.add_token(TokenType::RightParenthesis, None, None),
                '{' => self.add_token(TokenType::LeftBrace, None, None),
                '}' => self.add_token(TokenType::RightBrace, None, None),
                ',' => self.add_token(TokenType::Comma, None, None),
                '.' => self.add_token(TokenType::Dot, None, None),
                '-' => self.add_token(TokenType::Minus, None, None),
                '+' => self.add_token(TokenType::Plus, None, None),
                ';' => self.add_token(TokenType::Semicolon, None, None),
                '/' => {
                    todo!("Check if it has a matching '/' which would make it a comment.")
                }
                '*' => self.add_token(TokenType::Star, None, None),
                _ => {
                    break;
                }
            }
        }
    }

    fn add_token(&mut self, kind: TokenType, lexeme: Option<&'a str>, literal: Option<&'a str>) {
        self.tokens
            .push(Token::new(kind, lexeme, literal, self.line));
    }

    fn advance(&mut self, str: &str) -> Option<char> {
        self.current = self.current + 1;
        str.chars().nth(self.current - 1)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Write};

    use tempfile::tempfile;

    use crate::scanner::Scanner;
    use crate::tokens::{Token, TokenType};

    #[test]
    fn it_creates_new_scanner() {
        let result = std::panic::catch_unwind(|| {
            let file = tempfile().expect("Could not create temp file.");
            let mut buf = BufReader::new(file);
            let _scanner = Scanner::new(&mut buf);
        });
        assert!(!result.is_err())
    }

    #[test]
    fn it_scans_an_empty_file() {
        let mut file = tempfile().expect("Could not create temp file.");
        writeln!(file, "print \"hello world\";").expect("Could not write to temp file.");
        let mut buf = BufReader::new(file);
        let scanner = Scanner::new(&mut buf);
        let scanner = scanner.scan();
        let expected = vec![Token::new(TokenType::EOF, None, None, 1)];
        assert_eq!(scanner.tokens, expected)
    }

    #[test]
    fn it_scans_a_hello_world_file() {
        let mut file = tempfile().expect("Could not create temp file.");
        writeln!(file, "print \"hello world\";").expect("Could not write to temp file.");
        let mut buf = BufReader::new(file);
        let scanner = Scanner::new(&mut buf);
        let expected = vec![
            Token::new(TokenType::Print, None, None, 1),
            Token::new(
                TokenType::String,
                Some("hello world".into()),
                Some("\"hello world\"".into()),
                1,
            ),
        ];

        assert_eq!(scanner.tokens, expected)
    }
}
