#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    /*Single chars*/
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    /*One or two chars*/
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    /*Literals*/
    Identifier,
    String,
    Number,
    /*Keywords*/
    And,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    True,
    Var,
    While,
    Eof,
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value {
            "(" => TokenType::LeftParenthesis,
            ")" => TokenType::RightParenthesis,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "," => TokenType::Comma,
            "." => TokenType::Dot,
            "-" => TokenType::Minus,
            "+" => TokenType::Plus,
            ";" => TokenType::Semicolon,
            "/" => TokenType::Slash,
            "*" => TokenType::Star,
            "!" => TokenType::Bang,
            "!=" => TokenType::BangEqual,
            "=" => TokenType::Equal,
            "==" => TokenType::EqualEqual,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            "and" => TokenType::And,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "Nil" => TokenType::Nil,
            "Or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: Option<&'a str>,
    literal: Option<&'a str>,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: Option<&'a str>,
        literal: Option<&'a str>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
