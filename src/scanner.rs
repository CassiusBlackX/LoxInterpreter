use crate::object::Object;
use crate::token::{Token, TokenType, match_keywords};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // scanner will no longer live after `scan_tokens`
    // so we return the ownership of tokens out,
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            '*' => self.add_token(TokenType::Star, None),
            ';' => self.add_token(TokenType::SemiColon, None),
            '!' => {
                let is_equal = self.next('=');
                self.add_token(
                    if is_equal {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    None,
                );
            }
            '=' => {
                let is_equal = self.next('=');
                self.add_token(
                    if is_equal {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    },
                    None,
                );
            }
            '<' => {
                let is_equal = self.next('=');
                self.add_token(
                    if is_equal {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    None,
                );
            }
            '>' => {
                let is_equal = self.next('=');
                self.add_token(
                    if is_equal {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    None,
                );
            }
            '/' => {
                if self.next('/') {
                    // a comment line
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            x => {
                if x.is_ascii_digit() {
                    self.number();
                } else if x.is_ascii_alphabetic() || x == '_' {
                    self.identifier();
                } else {
                    eprintln!("unexpected character: {x}");
                }
            }
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn next(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, type_: TokenType, literal: Option<Object>) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(type_, lexeme, literal, self.line));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.at_end() {
            eprintln!("Unterminated string at {}", self.line);
            return;
        }
        self.advance(); // consume the closing '"'
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String_, Some(Object::String_(value)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume the "."
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value = self.source[self.start..self.current].to_string();
        let value = value.parse::<f64>().unwrap();
        self.add_token(TokenType::Number, Some(Object::Number(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let value = &self.source[self.start..self.current];
        let type_ = match_keywords(value);
        match type_ {
            Some(TokenType::True) => self.add_token(TokenType::True, Some(Object::Bool(true))),
            Some(TokenType::False) => self.add_token(TokenType::False, Some(Object::Bool(false))),
            Some(TokenType::Nil) => self.add_token(TokenType::Nil, Some(Object::Nil)),
            Some(x) => self.add_token(x, None),
            None => self.add_token(
                TokenType::Identifier,
                Some(Object::Identifier(
                    self.source[self.start..self.current].to_string(),
                )),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::*;

    fn scanner_helper(content: String, expected: &[Token]) {
        let scanner = Scanner::new(content);
        let got = scanner.scan_tokens();
        assert_eq!(got.len(), expected.len());
        for (index, tk_e) in expected.iter().enumerate() {
            let tk_g = &got[index];
            assert_eq!(tk_e.get_line(), tk_g.get_line());
            assert_eq!(tk_e.get_type(), tk_g.get_type());
            assert_eq!(tk_e.get_lexeme(), tk_g.get_lexeme());
            assert_eq!(tk_e.get_literal(), tk_g.get_literal());
        }
    }

    #[test]
    fn test_scan_operator() {
        let content = "(*!) != <= == =;".to_string();
        let expected = &[
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Bang, "!".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
            Token::new(TokenType::LessEqual, "<=".to_string(), None, 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::SemiColon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, String::new(), None, 1),
        ];
        scanner_helper(content, expected);
    }

    #[test]
    fn test_special_ascii() {
        let content = "a\r\t\nb  \"happy\"//nothing\nc".to_string();
        let expected = &[
            Token::new(
                TokenType::Identifier,
                "a".to_string(),
                Some(Object::Identifier("a".to_string())),
                1,
            ),
            Token::new(
                TokenType::Identifier,
                "b".to_string(),
                Some(Object::Identifier("b".to_string())),
                2,
            ),
            Token::new(
                TokenType::String_,
                "\"happy\"".to_string(),
                Some(Object::String_("happy".to_string())),
                2,
            ),
            Token::new(
                TokenType::Identifier,
                "c".to_string(),
                Some(Object::Identifier("c".to_string())),
                3,
            ),
            Token::new(TokenType::Eof, String::new(), None, 3),
        ];
        scanner_helper(content, expected);
    }

    #[test]
    fn test_number() {
        let content = "123456\r\n 123.456".to_string();
        let expected = &[
            Token::new(
                TokenType::Number,
                "123456".to_string(),
                Some(Object::Number(123456f64)),
                1,
            ),
            Token::new(
                TokenType::Number,
                "123.456".to_string(),
                Some(Object::Number(123.456)),
                2,
            ),
            Token::new(TokenType::Eof, String::new(), None, 2),
        ];
        scanner_helper(content, expected);
    }

    #[test]
    fn test_keyword() {
        let content = "fun if funny \n false classifier class \rreturn".to_string();
        let expected = &[
            Token::new(TokenType::Fun, "fun".to_string(), None, 1),
            Token::new(TokenType::If, "if".to_string(), None, 1),
            Token::new(
                TokenType::Identifier,
                "funny".to_string(),
                Some(Object::Identifier("funny".to_string())),
                1,
            ),
            Token::new(
                TokenType::False,
                "false".to_string(),
                Some(Object::Bool(false)),
                2,
            ),
            Token::new(
                TokenType::Identifier,
                "classifier".to_string(),
                Some(Object::Identifier("classifier".to_string())),
                2,
            ),
            Token::new(TokenType::Class, "class".to_string(), None, 2),
            Token::new(TokenType::Return, "return".to_string(), None, 2),
            Token::new(TokenType::Eof, String::new(), None, 2),
        ];
        scanner_helper(content, expected);
    }
}
