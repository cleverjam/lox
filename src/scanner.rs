use crate::tokens::{Token, TokenType};

pub struct Scanner<'a> {
    file: &'a str,
    pub tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(file: &'a str) -> Scanner<'a> {
        Scanner {
            file,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) {
        while !self.cursor_done() {
            self.start = self.current;
            self.scan_tokens()
        }

        // EOF
        self.tokens
            .push(Token::new(TokenType::Eof, None, None, self.line));
    }

    fn scan_tokens(&mut self) {
        while let Some(c) = self.advance() {
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
                    if self.advance_if_matches('=') {
                        self.add_token(TokenType::BangEqual, None, None)
                    } else {
                        self.add_token(TokenType::Bang, None, None)
                    }
                }
                '=' => {
                    if self.advance_if_matches('=') {
                        self.add_token(TokenType::EqualEqual, None, None)
                    } else {
                        self.add_token(TokenType::Equal, None, None)
                    }
                }
                '<' => {
                    if self.advance_if_matches('=') {
                        self.add_token(TokenType::LessEqual, None, None)
                    } else {
                        self.add_token(TokenType::Less, None, None)
                    }
                }
                '>' => {
                    if self.advance_if_matches('=') {
                        self.add_token(TokenType::GreaterEqual, None, None)
                    } else {
                        self.add_token(TokenType::Greater, None, None)
                    }
                }
                '/' => {
                    if self.advance_if_matches('/') {
                        while self.peek() != '\n' && !self.cursor_done() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None, None)
                    }
                }
                '*' => self.add_token(TokenType::Star, None, None),
                ' ' | '\r' | '\t' | '\n' => {
                    break;
                }
                '"' => {
                    self.string();
                    break;
                }
                _ => {
                    if c.is_ascii_digit() {
                        self.number();
                    } else if c.is_alphabetic() {
                        self.identifier();
                    } else {
                        panic!("Woah woah dude.")
                    }
                    break;
                }
            }
        }
    }

    /// Adds a token to the instance's token vector, simplifies token creation.
    fn add_token(&mut self, kind: TokenType, lexeme: Option<&'a str>, literal: Option<&'a str>) {
        self.tokens
            .push(Token::new(kind, lexeme, literal, self.line));
    }

    /// Returns current character and moves cursor to the next
    fn advance(&mut self) -> Option<char> {
        self.current += 1;

        self.file.chars().nth(self.current - 1)
    }

    /// Returns true if the next char matches given char
    fn advance_if_matches(&mut self, expected: char) -> bool {
        if self.cursor_done() {
            return false;
        }
        let c = self.file.chars().nth(self.current).unwrap();
        if c == expected {
            self.current += 1;
        }

        c == expected
    }

    /// Returns true if the cursor is at end of line.
    fn cursor_done(&self) -> bool {
        self.current >= self.file.len()
    }

    // Scans all the way until the end of a word to create a token
    fn identifier(&mut self) {
        while self
            .file
            .chars()
            .nth(self.current)
            .unwrap()
            .is_alphanumeric()
        {
            self.advance();
        }
        let text = &self.file[self.start..self.current];
        self.add_token(TokenType::from(text), Some(text), None);
    }

    /// Extracts a number and creates a token for it.
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance(); // whole part
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // decimal point
        }

        while self.peek().is_ascii_digit() {
            self.advance(); // fractional part
        }
        let lexeme = &self.file[self.start..self.current];
        self.add_token(TokenType::Number, Some(lexeme), None)
    }

    /// Returns current value without advancing into the next character
    fn peek(&self) -> char {
        if self.cursor_done() {
            return '\0';
        }
        self.file.chars().nth(self.current).unwrap()
    }

    /// Returns next value without advancing at all
    fn peek_next(&self) -> char {
        if self.cursor_done() {
            return '\0';
        }
        self.file.chars().nth(self.current + 1).unwrap()
    }

    /// Extracts a string and creates a token for it.
    fn string(&mut self) {
        while self.peek() != '"' && !self.cursor_done() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.cursor_done() {
            panic!("unterminated string somewhere, figure it out.");
        }

        self.advance();

        let literal = &self.file[self.start..self.current];
        let lexeme = &self.file[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(lexeme), Some(literal));
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use crate::tokens::{Token, TokenType};

    #[test]
    fn it_creates_new_scanner() {
        let result = std::panic::catch_unwind(|| {
            let _scanner = Scanner::new("Some file, it does not matter.");
        });
        assert!(result.is_ok())
    }

    #[test]
    fn it_scans_an_empty_file() {
        let mut scanner = Scanner::new("      ");
        scanner.scan();
        let expected = vec![Token::new(TokenType::Eof, None, None, 1)];
        assert_eq!(
            scanner.tokens, expected,
            "\n\nreceived:\n{:#?}\n\nexpected:\n{:#?}",
            scanner.tokens, expected
        )
    }

    #[test]
    fn it_ignores_comments_in_an_empty_file() {
        let mut scanner = Scanner::new("// this file has nothing but a comment");
        scanner.scan();
        let expected = vec![Token::new(TokenType::Eof, None, None, 1)];
        assert_eq!(
            scanner.tokens, expected,
            "\n\nreceived:\n{:#?}\n\nexpected:\n{:#?}",
            scanner.tokens, expected
        )
    }

    #[test]
    fn it_scans_a_hello_world_file() {
        let mut scanner = Scanner::new("print \"hello world\";");
        scanner.scan();
        let expected = vec![
            Token::new(TokenType::Print, Some("print"), None, 1),
            Token::new(
                TokenType::String,
                Some("hello world"),
                Some("\"hello world\""),
                1,
            ),
            Token::new(TokenType::Semicolon, None, None, 1),
            Token::new(TokenType::Eof, None, None, 1),
        ];

        assert_eq!(
            scanner.tokens, expected,
            "\n\nreceived:\n{:#?}\n\nexpected:\n{:#?}",
            scanner.tokens, expected
        )
    }

    #[test]
    fn it_scans_a_number() {
        let mut scanner = Scanner::new("12345");
        scanner.scan();
        let expected = vec![
            Token::new(TokenType::Number, Some("12345"), None, 1),
            Token::new(TokenType::Eof, None, None, 1),
        ];

        assert_eq!(
            scanner.tokens, expected,
            "\n\nreceived:\n{:#?}\n\nexpected:\n{:#?}",
            scanner.tokens, expected
        )
    }

    #[test]
    fn it_ignores_comments() {
        let mut scanner = Scanner::new("// this is a comment\n print \"hello world\";");
        scanner.scan();
        let expected = vec![
            Token::new(TokenType::Print, Some("print"), None, 1),
            Token::new(
                TokenType::String,
                Some("hello world"),
                Some("\"hello world\""),
                1,
            ),
            Token::new(TokenType::Semicolon, None, None, 1),
            Token::new(TokenType::Eof, None, None, 1),
        ];

        assert_eq!(
            scanner.tokens, expected,
            "\n\nreceived:\n{:#?}\n\nexpected:\n{:#?}",
            scanner.tokens, expected
        )
    }

    #[test]
    fn it_correctly_identifies_numeric_values_and_identifiers() {
        let mut scanner = Scanner::new("var pi = 3.14159");
        scanner.scan();
        let expected = vec![
            Token::new(TokenType::Var, Some("var"), None, 1),
            Token::new(TokenType::Identifier, Some("pi"), None, 1),
            Token::new(TokenType::Equal, None, None, 1),
            Token::new(TokenType::Number, Some("3.14159"), None, 1),
            Token::new(TokenType::Eof, None, None, 1),
        ];
        assert_eq!(
            scanner.tokens, expected,
            "\n\nreceived:\n{:#?}\n\nexpected:\n{:#?}",
            scanner.tokens, expected
        )
    }
}
