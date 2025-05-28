use crate::{
    error::{ParseError, report},
    expr::*,
    object::Object,
    stmt::*,
    token::{Token, TokenType},
};

// program -> declaration* EOF ;
// declaration -> varDecl | statement ;
// varDecl -> "var" IDENTIFIER ( "=" expression )? ";" ;
// statement -> exprStmt | ifStmt | whileStmt | forStmt | printStmt | block ;
// block -> "{" declaration* "}" ;
// exprStmt -> expression ";" ;
// printStmt -> "print" expression ";" ;
// ifStmt -> "if" "(" expression ")" statement ( "else" statement )? ;
// whileStmt -> "while" "(" expression ")" statement ;
// forStmt -> "for" "(" (varDecl | exprStmt | ";")
//                      expression? ";"
//                      expression? ")" statement ;
//
// expression -> assignment ;
// assignment -> IDENTIFIER "=" assignment | logic_or ;
// logic_or -> logic_and ( "or" | logic_and )* ;
// logic_and -> equality ( "and" equality )* ;
// equality -> comparison ( ( "!=" | "==") comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor ( ( "-" | "+" ) factor )* ;
// factor -> unary ( ("/" | "*" ) unary )* ;
// unary -> ("!" | "-" ) unary | primary;
// primary -> NUMBER | STRING | BOOL | NIL | "(" expression ")" | IDENTIFIER ;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.token_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name")?;
        let mut initializer = Expr::Literal(Literal { value: Object::Nil });
        if self.token_match(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }
        let _ = self.consume(
            TokenType::SemiColon,
            "Expect ';' after variable declaration",
        );
        Ok(Stmt::VarDecl(VarDecl {
            name,
            initializer: Box::new(initializer),
        }))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.token_match(&[TokenType::Print]) {
            self.print_statement()
        } else if self.token_match(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }))
        } else if self.token_match(&[TokenType::If]) {
            self.if_statement()
        } else if self.token_match(&[TokenType::While]) {
            self.while_statement()
        } else if self.token_match(&[TokenType::For]) {
            self.for_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        let _ = self.consume(TokenType::SemiColon, "Expect ';' after value")?;
        Ok(Stmt::PrintStmt(PrintStmt {
            expr: Box::new(value),
        }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.at_end() {
            statements.push(self.declaration()?);
        }
        let _ = self.consume(TokenType::RightBrace, "Expect '}' after block");
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
        let condition = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expect ')' after if condition")?;
        let then_branch = self.statement()?;
        let else_branch = if self.token_match(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::IfStmt(IfStmt {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        }))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expect ')' after while condition")?;
        let body = self.statement()?;
        Ok(Stmt::WhileStmt(WhileStmt {
            condition: Box::new(condition),
            body: Box::new(body),
        }))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;

        let initializer = if self.token_match(&[TokenType::SemiColon]) {
            None
        } else if self.token_match(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenType::SemiColon) {
            self.expression()?
        } else {
            Expr::Literal(Literal {
                value: Object::Bool(true),
            })
        };
        let _ = self.consume(TokenType::SemiColon, "Expect ';' after loop condition")?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        let _ = self.consume(TokenType::RightParen, "Expect ')' after loop condition")?;

        let mut body = self.statement()?;
        if let Some(inc) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![
                    body,
                    Stmt::ExprStmt(ExprStmt {
                        expr: Box::new(inc),
                    }),
                ],
            });
        }
        body = Stmt::WhileStmt(WhileStmt {
            condition: Box::new(condition),
            body: Box::new(body),
        });
        if let Some(init) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![init, body],
            });
        }

        Ok(body)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        let _ = self.consume(TokenType::SemiColon, "Expect ';' after value")?;
        Ok(Stmt::ExprStmt(ExprStmt {
            expr: Box::new(expr),
        }))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logic_or();

        if self.token_match(&[TokenType::Equal]) {
            let equal = self.previous();
            let value = self.assignment()?;

            if let Ok(ex) = expr {
                match ex {
                    Expr::Variable(var) => {
                        return Ok(Expr::Assign(Assign {
                            target: var,
                            value: Box::new(value),
                        }));
                    }
                    _ => {
                        self.error(&equal, "invalid assignment target");
                        return Err(ParseError("invalid assignment target".to_string()));
                    }
                }
            }
        }
        expr
    }

    fn logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logic_and();
        while self.token_match(&[TokenType::Or]) {
            let op = self.previous();
            let right = self.logic_and()?;
            expr = Ok(Expr::Logical(Logical {
                left: Box::new(expr?),
                op,
                right: Box::new(right),
            }));
        }
        expr
    }

    fn logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality();

        while self.token_match(&[TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Ok(Expr::Logical(Logical {
                left: Box::new(expr?),
                op,
                right: Box::new(right),
            }))
        }
        expr
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();
        while self.token_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }));
        }
        expr
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();
        while self.token_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }));
        }
        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();
        while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }));
        }
        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();
        while self.token_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }));
        }
        expr
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.token_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.token_match(&[TokenType::False]) {
            Ok(Expr::Literal(Literal {
                value: Object::Bool(false),
            }))
        } else if self.token_match(&[TokenType::True]) {
            Ok(Expr::Literal(Literal {
                value: Object::Bool(true),
            }))
        } else if self.token_match(&[TokenType::Nil]) {
            Ok(Expr::Literal(Literal { value: Object::Nil }))
        } else if self.token_match(&[TokenType::Number, TokenType::String_]) {
            Ok(Expr::Literal(Literal {
                value: self.previous().take_literal().unwrap(),
            }))
        } else if self.token_match(&[TokenType::Identifier]) {
            Ok(Expr::Variable(Variable {
                name: self.previous(),
            }))
        } else if self.token_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            let _token = self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            Ok(Expr::Grouping(Grouping {
                expr: Box::new(expr?),
            }))
        } else {
            token_error(self.peek(), "expected expression");
            Err(ParseError("expected expression".to_string()))
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
            return Err(ParseError(messaage.to_string()));
        }
        self.advance();
        Ok(self.previous())
    }

    fn error(&self, token: &Token, message: &str) {
        token_error(token, message);
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.at_end() {
            if self.previous().get_type() == TokenType::SemiColon {
                return;
            }
            match self.peek().get_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
        }
        self.advance();
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
