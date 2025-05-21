use crate::token::{KEYWORDS, Literals, Token, TokenType};

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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
        self.tokens.clone()
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

    fn peek(&mut self) -> char {
        if self.at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, type_: TokenType, literal: Option<Literals>) {
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
        self.add_token(TokenType::String_, Some(Literals::String_(value)));
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
        self.add_token(TokenType::Number, Some(Literals::Number(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let value = self.source[self.start..self.current].to_string();
        let type_ = KEYWORDS.get(&value);
        if let Some(&ky_type) = type_ {
            self.add_token(ky_type, None);
        } else {
            self.add_token(TokenType::Identifier, Some(Literals::Identifier(value)));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_scan_operator() {
        let content = "(*!) != <=".to_string();
        let mut scanner = Scanner::new(content);
        let tokens = scanner.scan_tokens();
        assert_eq!(TokenType::LeftParen, tokens[0].get_type());
        assert_eq!(TokenType::Star, tokens[1].get_type());
        assert_eq!(TokenType::Bang, tokens[2].get_type());
        assert_eq!(TokenType::RightParen, tokens[3].get_type());
        assert_eq!(TokenType::BangEqual, tokens[4].get_type());
        assert_eq!(TokenType::LessEqual, tokens[5].get_type());
    }

    #[test]
    fn test_special_ascii() {
        let content = "a\r\t\nb".to_string();
        let mut scanner = Scanner::new(content);
        let tokens = scanner.scan_tokens();
        let token_a = Token::new(
            TokenType::Identifier,
            "a".to_string(),
            Some(Literals::Identifier("a".to_string())),
            1,
        );
        assert_eq!(token_a.get_type(), tokens[0].get_type());
        assert_eq!(token_a.get_lexeme(), tokens[0].get_lexeme());
        assert_eq!(token_a.get_literal(), tokens[0].get_literal());
        assert_eq!(token_a.get_line(), tokens[0].get_line());
        let token_b = Token::new(
            TokenType::Identifier,
            "b".to_string(),
            Some(Literals::Identifier("b".to_string())),
            2,
        );
        assert_eq!(token_b.get_type(), tokens[1].get_type());
        assert_eq!(token_b.get_lexeme(), tokens[1].get_lexeme());
        assert_eq!(token_b.get_literal(), tokens[1].get_literal());
        assert_eq!(token_b.get_line(), tokens[1].get_line());
        let token_eof = Token::new(TokenType::Eof, "".to_string(), None, 2);
        assert_eq!(token_eof.get_type(), tokens[2].get_type());
        assert_eq!(token_eof.get_lexeme(), tokens[2].get_lexeme());
        assert_eq!(token_eof.get_literal(), tokens[2].get_literal());
        assert_eq!(token_eof.get_line(), tokens[2].get_line());
    }

    #[test]
    fn test_number() {
        let content = "123456\r\n 123.456".to_string();
        let mut scanner = Scanner::new(content);
        let tokens = scanner.scan_tokens();
        let token_int = Token::new(
            TokenType::Number,
            "123456".to_string(),
            Some(Literals::Number(123456 as f64)),
            1,
        );
        assert_eq!(token_int.get_type(), tokens[0].get_type());
        assert_eq!(token_int.get_lexeme(), tokens[0].get_lexeme());
        assert_eq!(token_int.get_literal(), tokens[0].get_literal());
        assert_eq!(token_int.get_line(), tokens[0].get_line());
        let token_float = Token::new(
            TokenType::Number,
            "123.456".to_string(),
            Some(Literals::Number(123.456)),
            2,
        );
        assert_eq!(token_float.get_type(), tokens[1].get_type());
        assert_eq!(token_float.get_lexeme(), tokens[1].get_lexeme());
        assert_eq!(token_float.get_literal(), tokens[1].get_literal());
        assert_eq!(token_float.get_line(), tokens[1].get_line());
    }

    #[test]
    fn test_keyword() {
        let content = "fun if \n false class\rreturn".to_string();
        let mut scanner = Scanner::new(content);
        let tokens = scanner.scan_tokens();
        let expected = vec![
            Token::new(TokenType::Fun, "fun".to_string(), None, 1),
            Token::new(TokenType::If, "if".to_string(), None, 1),
            Token::new(TokenType::False,"false".to_string(), None, 2),
            Token::new(TokenType::Class, "class".to_string(), None, 2),
            Token::new(TokenType::Return, "return".to_string(), None, 2),
        ];
        for (index, token) in expected.iter().enumerate() {
            assert_eq!(token.get_type(), tokens[index].get_type());
            assert_eq!(token.get_lexeme(), tokens[index].get_lexeme());
            assert_eq!(token.get_literal(), tokens[index].get_literal());
            assert_eq!(token.get_line(), tokens[index].get_line());
        } 
    }
}
