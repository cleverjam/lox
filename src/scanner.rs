use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::tokens::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a mut BufReader<File>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a mut BufReader<File>) -> Scanner<'a> {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
        }
    }

    pub fn scan(mut self) -> Self {
        let mut buf = String::new();
        let mut line = 1;
        loop {
            buf.clear();
            match self.source.read_line(&mut buf) {
                Ok(size) => {
                    if size == 0 {
                        break;
                    }
                    print!("{}:\t {}", line, buf);
                }
                Err(_) => {
                    println!("There was an error reading the file, line: {}", line);
                    break;
                }
            }

            line = line + 1;
        }

        // EOF
        self.tokens
            .push(Token::new(TokenType::EOF, None, None, line));
        self
    }
}

#[cfg(test)]
mod scanner_tests {
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
