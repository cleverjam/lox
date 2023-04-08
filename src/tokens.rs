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
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
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
