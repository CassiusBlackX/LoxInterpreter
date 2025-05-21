/*
* expression -> equality ;
* equality -> comparison ( ( "!=" | "==" ) comparison )* ;
* comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
* term -> factor ( ( "-" | "+" ) factor )* ;
* factor -> unary ( ( "/" | "*" ) unary )* ;
* unary -> ("!" | "-") unary | primary ;
* primary -> NUMBER | STRING | BOOL | NIL | "(" expression ")" ;
*/

use crate::token::{LiteralType, Token};

pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
}

pub struct Literal {
    pub value: LiteralType,
}

pub struct Grouping {
    pub expr: Box<Expr>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait Visitor<T> {
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
    fn visit_unary(&self, expr: &Unary) -> T;
    fn visit_binary(&self, expr: &Binary) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Self::Literal(literal) => visitor.visit_literal(literal),
            Self::Grouping(grouping) => visitor.visit_grouping(grouping),
            Self::Unary(unary) => visitor.visit_unary(unary),
            Self::Binary(binary) => visitor.visit_binary(binary),
        }
    }
}
