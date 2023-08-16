use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    // book-keeping tokens
    EndFile,
    Error(String),
    // reserved words
    If,
    Then,
    Else,
    End,
    Repeat,
    Until,
    Read,
    Write,
    // multicharacter tokens
    Id(String),
    Num(String),

    // special symbols
    Assign,
    Eq,
    Lt,
    Plus,
    Minus,
    Times,
    Over,
    Lparen,
    Rparen,
    Semi,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Token::If => write!(f, "reserved word: if"),
            Token::Then => write!(f, "reserved word: then"),
            Token::Else => write!(f, "reserved word: else"),
            Token::End => write!(f, "reserved word: end"),
            Token::Repeat => write!(f, "reserved word: repeat"),
            Token::Until => write!(f, "reserved word: until"),
            Token::Read => write!(f, "reserved word: read"),
            Token::Write => write!(f, "reserved word: write"),

            Token::Assign => write!(f, ":="),
            Token::Lt => write!(f, "<"),
            Token::Eq => write!(f, "="),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Semi => write!(f, ";"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Times => write!(f, "*"),
            Token::Over => write!(f, "/"),
            Token::EndFile => write!(f, "EOF"),
            Token::Num(num) => write!(f, "NUM, val= {}", num),
            Token::Id(id) => write!(f, "ID, name= {}", id),
            Token::Error(msg) => write!(f, "ERROR: {}", msg),
        }
    }
}
