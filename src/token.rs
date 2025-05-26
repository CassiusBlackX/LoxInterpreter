use std::fmt;

use phf::phf_map;
use strum_macros::Display;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // literals
    Identifier,
    String_,
    Number,
    // keywords
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

    Eof,
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub fn match_keywords(word: &str) -> Option<TokenType> {
    KEYWORDS.get(word).copied()
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Identifier(String),
    String_(String),
    Bool(bool),
    Nil,
    Number(f64),
    // TODO: we should respectively give integer and float!
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "{ident}"),
            Self::String_(content) => write!(f, "{content}"),
            Self::Number(x) => write!(f, "{x}"),
            Self::Bool(flag) => write!(f, "{}", if *flag { "true" } else { "false" }),
            Self::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    pub literal: Option<LiteralType>,
    line: usize,
}

impl Token {
    pub fn new(
        type_: TokenType,
        lexeme: String,
        literal: Option<LiteralType>,
        line: usize,
    ) -> Self {
        Self {
            type_,
            lexeme,
            literal,
            line,
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.type_
    }

    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn get_literal(&self) -> Option<&LiteralType> {
        match &self.literal {
            None => None,
            Some(l) => Some(l),
        }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            Some(literal) => write!(f, "{} {} {}", self.type_, self.lexeme, literal),
            None => write!(f, "{} {}", self.type_, self.lexeme),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_keywords_hashmap() {
        assert_eq!(TokenType::Print, match_keywords("print").unwrap());
        assert_eq!(TokenType::While, match_keywords("while").unwrap());
    }
}
