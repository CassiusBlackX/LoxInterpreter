use std::collections::{HashMap, VecDeque};

use crate::error::token_error;
use crate::expr::*;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt::*;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
enum FunctionType {
    None,
    Function,
    Method,
    Initializer,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ClassType {
    None,
    Class,
    SubClass,
}

#[derive(Debug)]
pub struct ResolveError(pub String);

pub struct Resolver<'a> {
    scopes: VecDeque<HashMap<String, bool>>,
    interpreter: &'a mut Interpreter,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            scopes: VecDeque::new(),
            interpreter,
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    pub fn resolve_all(&mut self, statements: &[Stmt]) -> Result<(), ResolveError> {
        for statement in statements {
            self.resolve_stmt(statement)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, statement: &Stmt) -> Result<(), ResolveError> {
        statement.accept(self)?;
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolveError> {
        expr.accept(self)?;
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push_back(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop_back();
    }

    fn declare(&mut self, name: &Token) -> Result<(), ResolveError> {
        if self.scopes.is_empty() {
            return Ok(());
        }
        if self.scopes.back().unwrap().contains_key(name.get_lexeme()) {
            token_error(name, "Already a variable with this name in this scope");
            return Err(ResolveError(
                "Already a variable with this name in this scope".to_string(),
            ));
        }
        self.scopes
            .back_mut()
            .unwrap()
            .insert(name.get_lexeme().to_string(), false);
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .back_mut()
            .unwrap()
            .insert(name.get_lexeme().to_string(), true);
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), ResolveError> {
        for (index, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name.get_lexeme()) {
                self.interpreter
                    .resolve(expr, self.scopes.len() - 1 - index);
            }
        }
        Ok(())
    }

    fn resolve_fucntion(
        &mut self,
        function: &FunctionStmt,
        function_type: FunctionType,
    ) -> Result<(), ResolveError> {
        let enclosing_function = self.current_function;
        self.current_function = function_type;
        self.begin_scope();
        for param in &function.params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve_all(&function.body)?;
        self.end_scope();
        self.current_function = enclosing_function;
        Ok(())
    }
}

impl ExprVisitor<Result<(), ResolveError>> for Resolver<'_> {
    fn visit_literal(&mut self, _expr: &Literal) -> Result<(), ResolveError> {
        Ok(())
    }

    fn visit_variable(&mut self, expr: &Variable) -> Result<(), ResolveError> {
        if !self.scopes.is_empty() {
            let cond = self.scopes.back().unwrap().get(expr.name.get_lexeme());
            if let Some(is_init) = cond {
                if !is_init {
                    return Err(ResolveError(
                        "Can't read local variable in its own initializer".to_string(),
                    ));
                }
            }
        }
        self.resolve_local(&Expr::Variable(expr.to_owned()), &expr.name)?;
        Ok(())
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.expr)
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.right)
    }

    fn visit_binary(&mut self, expr: &Binary) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)
    }

    fn visit_assign(&mut self, expr: &Assign) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.value)?;
        self.resolve_local(&Expr::Assign(expr.to_owned()), &expr.name)?;
        Ok(())
    }

    fn visit_logical(&mut self, expr: &Logical) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)
    }

    fn visit_call(&mut self, expr: &Call) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.callee)?;
        for argument in &expr.arguments {
            self.resolve_expr(argument)?;
        }
        Ok(())
    }

    fn visit_get(&mut self, expr: &Get) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.object)
    }

    fn visit_set(&mut self, expr: &Set) -> Result<(), ResolveError> {
        self.resolve_expr(&expr.value)?;
        self.resolve_expr(&expr.object)
    }

    fn visit_this(&mut self, expr: &This) -> Result<(), ResolveError> {
        if self.current_class == ClassType::None {
            Err(ResolveError(
                "Can't use 'this' outside of a class".to_string(),
            ))
        } else {
            self.resolve_local(&Expr::This(expr.to_owned()), &expr.keyword)
        }
    }

    fn visit_super(&mut self, expr: &Super) -> Result<(), ResolveError> {
        match self.current_class {
            ClassType::None => {
                token_error(&expr.keyword, "Can't use 'super' outside of a class");
                Err(ResolveError(
                    "Can't use 'super' outside of a class".to_string(),
                ))
            }
            ClassType::Class => {
                token_error(
                    &expr.keyword,
                    "Can't use 'super' in a class with no superclass",
                );
                Err(ResolveError(
                    "Can't use 'super' in class with no superclass".to_string(),
                ))
            }
            ClassType::SubClass => self.resolve_local(&Expr::Super(expr.to_owned()), &expr.keyword),
        }
    }
}

impl StmtVisitor<Result<(), ResolveError>> for Resolver<'_> {
    fn visit_var_decl(&mut self, stmt: &VarDecl) -> Result<(), ResolveError> {
        self.declare(&stmt.name)?;
        if let Expr::Literal(Literal {
            uuid: _x,
            value: Object::Nil,
        }) = *stmt.initializer
        {
            self.resolve_expr(&stmt.initializer)?;
        }
        self.define(&stmt.name);
        Ok(())
    }

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Result<(), ResolveError> {
        self.resolve_expr(&stmt.expr)
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), ResolveError> {
        self.resolve_expr(&stmt.expr)
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), ResolveError> {
        self.begin_scope();
        self.resolve_all(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), ResolveError> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.then_branch)?;
        if let Some(else_) = &stmt.else_branch {
            self.resolve_stmt(else_)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), ResolveError> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.body)
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<(), ResolveError> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name);
        self.resolve_fucntion(stmt, FunctionType::Function)
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<(), ResolveError> {
        if self.current_function == FunctionType::None {
            token_error(&stmt.keyword, "Can't return from top-level code");
            return Err(ResolveError("Can't return from top-level code".to_string()));
        }
        if self.current_function == FunctionType::Initializer {
            token_error(&stmt.keyword, "Can't return from an initializer");
            return Err(ResolveError("Can't return from an initializer".to_string()));
        }
        if let Expr::Literal(Literal {
            uuid: _x,
            value: Object::Nil,
        }) = *stmt.value
        {
            self.resolve_expr(&stmt.value)?;
        }
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &ClassStmt) -> Result<(), ResolveError> {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        self.declare(&stmt.name)?;
        self.define(&stmt.name);

        if let Some(super_class) = &stmt.super_class {
            if let Expr::Variable(Variable { uuid: _x, name: s }) = &**super_class {
                if s.get_lexeme() == stmt.name.get_lexeme() {
                    token_error(s, "A class can't inherit from itself");
                    return Err(ResolveError(
                        "A class can't inherit from itself".to_string(),
                    ));
                }
            }
            self.current_class = ClassType::SubClass;
            self.resolve_expr(super_class)?;

            self.begin_scope();
            self.scopes
                .back_mut()
                .unwrap()
                .insert("super".to_string(), true);
        }

        self.begin_scope();
        self.scopes
            .back_mut()
            .unwrap()
            .insert("this".to_string(), true);
        for method in &stmt.methods {
            if let Stmt::Function(meth) = method {
                let mut func_type = FunctionType::Method;
                if meth.name.get_lexeme() == "init" {
                    func_type = FunctionType::Initializer;
                }
                self.resolve_fucntion(meth, func_type)?;
            } else {
                unreachable!()
                // return Err(ResolveError("Not a Class method!".to_string()));
            }
        }
        self.end_scope();
        if stmt.super_class.is_some() {
            self.end_scope();
        }
        self.current_class = enclosing_class;
        Ok(())
    }
}
