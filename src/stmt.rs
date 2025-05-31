use crate::expr::*;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VarDecl(VarDecl),
    Expr(ExprStmt),
    Print(PrintStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Return(ReturnStmt),
    Class(ClassStmt),
    Invalid,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarDecl {
    pub name: Token,
    // if not given a value when declaration

    // all variable will have `nil` as their default value
    pub initializer: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprStmt {
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrintStmt {
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassStmt {
    pub name: Token,
    pub super_class: Option<Box<Expr>>,
    pub methods: Vec<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_var_decl(&mut self, stmt: &VarDecl) -> T;
    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> T;
    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> T;
    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> T;
    fn visit_class_stmt(&mut self, stmt: &ClassStmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Self::VarDecl(stmt) => visitor.visit_var_decl(stmt),
            Self::Expr(stmt) => visitor.visit_expr_stmt(stmt),
            Self::Print(stmt) => visitor.visit_print_stmt(stmt),
            Self::Block(stmt) => visitor.visit_block_stmt(stmt),
            Self::If(stmt) => visitor.visit_if_stmt(stmt),
            Self::While(stmt) => visitor.visit_while_stmt(stmt),
            Self::Function(stmt) => visitor.visit_function_stmt(stmt),
            Self::Return(stmt) => visitor.visit_return_stmt(stmt),
            Self::Class(stmt) => visitor.visit_class_stmt(stmt),
            Self::Invalid => panic!("Attempting to run an Invalid Statement!"),
        }
    }
}
