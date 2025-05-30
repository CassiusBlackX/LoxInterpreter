/*
* expression -> equality ;
* equality -> comparison ( ( "!=" | "==" ) comparison )* ;
* comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
* term -> factor ( ( "-" | "+" ) factor )* ;
* factor -> unary ( ( "/" | "*" ) unary )* ;
* unary -> ("!" | "-") unary | primary ;
* primary -> NUMBER | STRING | BOOL | NIL | "(" expression ")" ;
*/

use std::hash::Hash;

use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub uuid: usize,
    pub value: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub uuid: usize,
    pub name: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub uuid: usize,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    pub uuid: usize,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    pub uuid: usize,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    pub uuid: usize,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Logical {
    pub uuid: usize,
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub uuid: usize,
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_literal(&mut self, expr: &Literal) -> T;
    fn visit_variable(&mut self, expr: &Variable) -> T;
    fn visit_grouping(&mut self, expr: &Grouping) -> T;
    fn visit_unary(&mut self, expr: &Unary) -> T;
    fn visit_binary(&mut self, expr: &Binary) -> T;
    fn visit_assign(&mut self, expr: &Assign) -> T;
    fn visit_logical(&mut self, expr: &Logical) -> T;
    fn visit_call(&mut self, expr: &Call) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Literal(expr) => visitor.visit_literal(expr),
            Expr::Variable(expr) => visitor.visit_variable(expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Assign(expr) => visitor.visit_assign(expr),
            Expr::Logical(expr) => visitor.visit_logical(expr),
            Expr::Call(expr) => visitor.visit_call(expr),
        }
    }

    fn get_uuid(&self) -> usize {
        match self {
            Self::Literal(e) => e.uuid,
            Self::Variable(e) => e.uuid,
            Self::Grouping(e) => e.uuid,
            Self::Unary(e) => e.uuid,
            Self::Binary(e) => e.uuid,
            Self::Assign(e) => e.uuid,
            Self::Logical(e) => e.uuid,
            Self::Call(e) => e.uuid,
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.get_uuid() == other.get_uuid()
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_uuid().hash(state);
        // we may even directly return the uuid is enough
    }
}
