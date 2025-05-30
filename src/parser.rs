use crate::{
    error::{ParseError, token_error},
    expr::*,
    object::Object,
    stmt::*,
    token::{Token, TokenType},
};

use std::sync::atomic::AtomicUsize;
static UUID: AtomicUsize = AtomicUsize::new(0);
fn gen_uuid() -> usize {
    UUID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

// program -> declaration* EOF ;
// declaration -> funDecl |varDecl | statement ;
// funDecl -> "fun" function ;
// function -> IDENTIFIER "(" parameters? ")" block ;
// parameters -> IDENTIFIER ( "," IDENTIFIER )* ;
// varDecl -> "var" IDENTIFIER ( "=" expression )? ";" ;
// statement -> exprStmt | ifStmt | whileStmt | forStmt | printStmt | returnStmt | block ;
// block -> "{" declaration* "}" ;
// exprStmt -> expression ";" ;
// printStmt -> "print" expression ";" ;
// ifStmt -> "if" "(" expression ")" statement ( "else" statement )? ;
// whileStmt -> "while" "(" expression ")" statement ;
// forStmt -> "for" "(" (varDecl | exprStmt | ";")
//                      expression? ";"
//                      expression? ")" statement ;
// returnStmt -> "return" expression? ";" ;
//
// expression -> assignment ;
// assignment -> IDENTIFIER "=" assignment | logic_or ;
// logic_or -> logic_and ( "or" | logic_and )* ;
// logic_and -> equality ( "and" equality )* ;
// equality -> comparison ( ( "!=" | "==") comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor ( ( "-" | "+" ) factor )* ;
// factor -> unary ( ("/" | "*" ) unary )* ;
// unary -> ("!" | "-" ) unary | call ;
// call -> primary ( "(" arguments? ")" )* ;
// arguments -> expression ( "," expression )* ;
// primary -> NUMBER | STRING | BOOL | NIL | "(" expression ")" | IDENTIFIER ;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // parse() will not exit immediately when got a ParseError
    // when finish parsing, if encountered a ParseError brefore, return ParseError
    // else return the parsed statements
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        let mut had_error = false;
        while !self.at_end() {
            let stmt = self.declaration();
            if stmt == Stmt::Invalid {
                had_error = true;
            }
            statements.push(stmt);
        }
        if had_error {
            Err(ParseError("Err when parsing!".to_string()))
        } else {
            Ok(statements)
        }
    }

    // declaration() will only return Stmt instead of Result<Stmt, ParseError>
    // so that even there were several ParseErrors, parser will go on
    // until all tokens are parsed.
    // NOTE: must handle the returning `Stmt` of declaration in case it is Stmt::Invalid
    fn declaration(&mut self) -> Stmt {
        let result;
        if self.token_match(&[TokenType::Var]) {
            result = self.var_declaration();
        } else if self.token_match(&[TokenType::Fun]) {
            result = self.function("function");
        } else if self.token_match(&[TokenType::Return]) {
            result = self.return_statement();
        } else {
            result = self.statement();
        }
        result
            .inspect_err(|_| self.synchronize())
            .unwrap_or(Stmt::Invalid)
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous();
        let value = if !self.check(TokenType::SemiColon) {
            self.expression()?
        } else {
            Expr::Literal(Literal {
                uuid: gen_uuid(),
                value: Object::Nil,
            })
        };
        let _ = self.consume(TokenType::SemiColon, "Expect ';' after return value")?;
        Ok(Stmt::ReturnStmt(ReturnStmt {
            keyword,
            value: Box::new(value),
        }))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {kind} name."))?;
        let _ = self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {kind} name"),
        )?;
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 parameters");
                }
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name")?);
                if !self.token_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let _ = self.consume(TokenType::RightParen, "Expect ')' after parameters")?;

        let _ = self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {kind} body"),
        )?;
        let body = self.block()?;
        Ok(Stmt::Function(FunctionStmt {
            name,
            params: parameters,
            body,
        }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name")?;
        let mut initializer = Expr::Literal(Literal {
            uuid: gen_uuid(),
            value: Object::Nil,
        });
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
        while !self.check(TokenType::RightBrace) && !self.at_end() {
            // blocl() use declaration(), so declaration should not
            // cause block() exit as soon as an Err occur
            let stmt = self.declaration();
            if stmt == Stmt::Invalid {
                self.error(self.peek(), "Error when parsing block statement!");
            } else {
                statements.push(stmt);
            }
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

        let condition = if !self.check(TokenType::SemiColon) {
            self.expression()?
        } else {
            Expr::Literal(Literal {
                uuid: gen_uuid(),
                value: Object::Bool(true),
            })
        };
        let _ = self.consume(TokenType::SemiColon, "Expect ';' after loop condition")?;

        let increment = if !self.check(TokenType::RightParen) {
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
                            uuid: gen_uuid(),
                            name: var.name,
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
                uuid: gen_uuid(),
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
                uuid: gen_uuid(),
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
                uuid: gen_uuid(),
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
                uuid: gen_uuid(),
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
                uuid: gen_uuid(),
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
                uuid: gen_uuid(),
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
                uuid: gen_uuid(),
                operator,
                right: Box::new(right),
            }));
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary();
        loop {
            if self.token_match(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr?);
            } else {
                break;
            }
        }
        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments");
                }
                arguments.push(self.expression()?);
                if !self.token_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments")?;
        Ok(Expr::Call(Call {
            uuid: gen_uuid(),
            callee: Box::new(callee),
            paren,
            arguments,
        }))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.token_match(&[TokenType::False]) {
            Ok(Expr::Literal(Literal {
                uuid: gen_uuid(),
                value: Object::Bool(false),
            }))
        } else if self.token_match(&[TokenType::True]) {
            Ok(Expr::Literal(Literal {
                uuid: gen_uuid(),
                value: Object::Bool(true),
            }))
        } else if self.token_match(&[TokenType::Nil]) {
            Ok(Expr::Literal(Literal {
                uuid: gen_uuid(),
                value: Object::Nil,
            }))
        } else if self.token_match(&[TokenType::Number, TokenType::String_]) {
            Ok(Expr::Literal(Literal {
                uuid: gen_uuid(),
                value: self.previous().take_literal().unwrap(),
            }))
        } else if self.token_match(&[TokenType::Identifier]) {
            Ok(Expr::Variable(Variable {
                uuid: gen_uuid(),
                name: self.previous(),
            }))
        } else if self.token_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            let _token = self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            Ok(Expr::Grouping(Grouping {
                uuid: gen_uuid(),
                expr: Box::new(expr?),
            }))
        } else {
            self.error(self.peek(), "expected expression");
            Err(ParseError("expected expression".to_string()))
        }
    }

    // an "or" match
    // any of the element in the given slice matches, will return true
    // only when no match in the given slice will return false
    fn token_match(&mut self, token_types: &[TokenType]) -> bool {
        token_types.iter().copied().any(|tt| {
            if self.check(tt) {
                self.advance();
                true
            } else {
                false
            }
        })
    }

    fn check(&self, token_type: TokenType) -> bool {
        !self.at_end() && self.peek().get_type() == token_type
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
        if !self.check(token_type) {
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
