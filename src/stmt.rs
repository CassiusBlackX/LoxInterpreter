use crate::expr::*;
use crate::token::Token;

pub enum Stmt {
    VarDecl(VarDecl),
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
    Block(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
}

pub struct VarDecl {
    pub name: Token,
    // if not given a value when declaration
    // all variable will have `nil` as their default value
    pub initializer: Box<Expr>,
}

pub struct ExprStmt {
    pub expr: Box<Expr>,
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_var_decl(&mut self, stmt: &VarDecl) -> T;
    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Self::VarDecl(stmt) => visitor.visit_var_decl(stmt),
            Self::ExprStmt(stmt) => visitor.visit_expr_stmt(stmt),
            Self::PrintStmt(stmt) => visitor.visit_print_stmt(stmt),
            Self::Block(stmt) => visitor.visit_block_stmt(stmt),
            Self::IfStmt(stmt) => visitor.visit_if_stmt(stmt),
            Self::WhileStmt(stmt) => visitor.visit_while_stmt(stmt),
        }
    }
}
