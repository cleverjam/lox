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
                    // print!("[{} bytes] {}:\t {}", _size, self.line, buf);
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
                '!' => {
                    if self.advance_if_matches(str, '=') {
                        self.add_token(TokenType::BangEqual, None, None)
                    } else {
                        self.add_token(TokenType::Bang, None, None)
                    }
                }
                '=' => {
                    if self.advance_if_matches(str, '=') {
                        self.add_token(TokenType::EqualEqual, None, None)
                    } else {
                        self.add_token(TokenType::Equal, None, None)
                    }
                }
                '<' => {
                    if self.advance_if_matches(str, '=') {
                        self.add_token(TokenType::LessEqual, None, None)
                    } else {
                        self.add_token(TokenType::Less, None, None)
                    }
                }
                '>' => {
                    if self.advance_if_matches(str, '=') {
                        self.add_token(TokenType::GreaterEqual, None, None)
                    } else {
                        self.add_token(TokenType::Greater, None, None)
                    }
                }
                '/' => {
                    if self.advance_if_matches(str, '/') {
                        while self.peek(str) != '\n' && !self.cursor_done(str) {
                            self.advance(str);
                        }
                    } else {
                        self.add_token(TokenType::Slash, None, None)
                    }
                }
                '*' => self.add_token(TokenType::Star, None, None),
                _ => {
                    break;
                }
            }
        }
    }
    /// Returns true if the cursor is at end of line.
    fn cursor_done(&mut self, str: &str) -> bool {
        self.current >= str.len()
    }

    /// Adds a token to the instance's token vector, simplifies token creation.
    fn add_token(&mut self, kind: TokenType, lexeme: Option<&'a str>, literal: Option<&'a str>) {
        self.tokens
            .push(Token::new(kind, lexeme, literal, self.line));
    }

    /// Returns current character and moves cursor to the next
    fn advance(&mut self, str: &str) -> Option<char> {
        self.current = self.current + 1;
        str.chars().nth(self.current - 1)
    }

    /// Returns true if the next char matches given char
    fn advance_if_matches(&mut self, buf: &str, expected: char) -> bool {
        if self.cursor_done(buf) {
            return false;
        }
        let c = buf.chars().nth(self.current).unwrap();
        if c != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    /// Returns current value without advancing into the next character
    fn peek(&mut self, buf: &str) -> char {
        if self.cursor_done(buf) {
            return '\0';
        }
        buf.chars().nth(self.current).unwrap()
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
        writeln!(file, "").expect("Could not write to temp file.");
        let mut buf = BufReader::new(file);
        let scanner = Scanner::new(&mut buf);
        let scanner = scanner.scan();
        let expected = vec![Token::new(TokenType::EOF, None, None, 1)];
        assert_eq!(scanner.tokens, expected)
    }

    #[test]
    fn it_ignores_comments_in_an_empty_file() {
        let mut file = tempfile().expect("Could not create temp file.");
        writeln!(file, "// this file has nothing but a comment")
            .expect("Could not write to temp file.");
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

    #[test]
    fn it_ignores_comments() {
        let mut file = tempfile().expect("Could not create temp file.");
        writeln!(file, "// this is a comment\n print \"hello world\";")
            .expect("Could not write to temp file.");
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
