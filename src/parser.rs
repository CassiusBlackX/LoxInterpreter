use crate::{
    error::report,
    expression::{Binary, Expr, Grouping, Literal, Unary},
    token::{LiteralType, Token, TokenType},
};

pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.token_match(&[TokenType::BangEqual, TokenType::Equal]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.token_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.token_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.token_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.token_match(&[TokenType::False]) {
            Expr::Literal(Literal {
                value: LiteralType::Bool(false),
            })
        } else if self.token_match(&[TokenType::True]) {
            Expr::Literal(Literal {
                value: LiteralType::Bool(true),
            })
        } else if self.token_match(&[TokenType::Nil]) {
            Expr::Literal(Literal {
                value: LiteralType::Nil,
            })
        } else if self.token_match(&[TokenType::Number, TokenType::String_]) {
            Expr::Literal(Literal {
                value: *self.previous().get_literal().unwrap(),
            })
        } else if self.token_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            Expr::Grouping(Grouping {
                expr: Box::new(expr),
            })
        } else {
            panic!("reaching the end of primary but no matches!")
        }
    }

    // an "or" match
    // any of the element in the given slice matches, will return true
    // only when no match in the given slice will return false
    fn token_match(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            self.peek().get_type() == *token_type
        }
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous();
    }

    fn at_end(&self) -> bool {
        self.peek().get_type() == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: TokenType, messaage: &str) -> Result<Token, ParseError> {
        if !self.check(&token_type) {
            self.error(&self.previous(), messaage);
            return Err(ParseError);
        }
        self.advance();
        Ok(self.previous())
    }

    fn error(&self, token: &Token, message: &str) {
        token_error(token, &message);
    }
}

pub fn token_error(token: &Token, message: &str) {
    if token.get_type() == TokenType::Eof {
        report(token.get_line(), &("at end ".to_owned() + message));
    } else {
        report(
            token.get_line(),
            &("at '".to_owned() + token.get_lexeme() + "'. " + message),
        );
    }
}
