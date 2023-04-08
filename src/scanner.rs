// use crate::tokens::;
use crate::tokens::Token;
use std::fs::File;
use std::io::BufReader;

pub struct Scanner {
    source: BufReader<File>,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: BufReader<File>) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
        }
    }

    pub fn scan(self) -> Vec<Token> {
        return self.tokens;
    }
}

#[cfg(test)]
mod scanner_tests {
    use crate::scanner::Scanner;
    use crate::tokens::{Token, TokenType};
    use std::io::{BufReader, Write};
    use tempfile::tempfile;

    #[test]
    fn it_creates_new_scanner() {
        let result = std::panic::catch_unwind(|| {
            let file = tempfile().expect("Could not create temp file.");
            Scanner::new(BufReader::new(file))
        });
        assert!(!result.is_err())
    }

    #[test]
    fn it_scans_an_empty_file() {
        let result = std::panic::catch_unwind(|| {
            let mut file = tempfile().expect("Could not create temp file.");
            writeln!(file, "print \"hello world\";").expect("Could not write to temp file.");
            Scanner::new(BufReader::new(file)).scan()
        });
        assert!(!result.is_err());
        let result = result.unwrap();
        let expected = vec![];
        let eq_count = result
            .iter()
            .zip(expected.iter())
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(result.len(), eq_count)
    }

    #[test]
    fn it_scans_a_hello_world_file() {
        let mut file = tempfile().expect("Could not create temp file.");
        writeln!(file, "print \"hello world\";").expect("Could not write to temp file.");
        let result = Scanner::new(BufReader::new(file)).scan();
        let expected = vec![
            Token::new(TokenType::Print, None, None, 1),
            Token::new(
                TokenType::String,
                Some("hello world".into()),
                Some("\"hello world\"".into()),
                1,
            ),
        ];

        assert_eq!(result, expected);
    }
}
