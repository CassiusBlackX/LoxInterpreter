use std::{cell::RefCell, rc::Rc};

use crate::callable::*;
use crate::expr::*;
use crate::stmt::*;
use crate::token::TokenType;
use crate::{environment::Environment, object::Object};

pub enum RuntimeException {
    RuntimeError(RuntimeError),
    Return_(Object),
}

#[derive(Debug)]
pub struct RuntimeError(pub String);

impl From<RuntimeError> for RuntimeException {
    fn from(value: RuntimeError) -> Self {
        Self::RuntimeError(value)
    }
}

// TODO: native function `clock()` not added
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        Self {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement).map_err(|e| {
                if let RuntimeException::RuntimeError(ee) = e {
                    ee
                } else {
                    panic!("impossible!")
                }
            })?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeException> {
        stmt.accept(self)
    }

    fn evalulate(&mut self, expr: &Expr) -> Result<Object, RuntimeException> {
        expr.accept(self)
    }

    pub fn execute_block(
        &mut self,
        stmts: &[Stmt],
        environment: Environment,
    ) -> Result<(), RuntimeException> {
        let previous = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(environment));
        let result = stmts
            .iter()
            .try_for_each(|statement| self.execute(statement));
        self.environment = previous;
        result
    }
}

impl ExprVisitor<Result<Object, RuntimeException>> for Interpreter {
    fn visit_literal(&mut self, expr: &Literal) -> Result<Object, RuntimeException> {
        Ok(expr.value.clone())
    }

    fn visit_variable(&mut self, expr: &Variable) -> Result<Object, RuntimeException> {
        self.environment
            .borrow_mut()
            .get(&expr.name)
            .map_err(RuntimeException::from)
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<Object, RuntimeException> {
        self.evalulate(&expr.expr)
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<Object, RuntimeException> {
        let right_value = self.evalulate(&expr.right)?;
        match expr.operator.get_type() {
            TokenType::Minus => {
                let value = right_value.get_double()?;
                Ok(Object::Number(-value))
            }
            TokenType::Bang => {
                let value = right_value.get_bool();
                Ok(Object::Bool(!value))
            }
            _ => unreachable!(),
        }
    }

    fn visit_binary(&mut self, expr: &Binary) -> Result<Object, RuntimeException> {
        let left_value = self.evalulate(&expr.left)?;
        let right_value = self.evalulate(&expr.right)?;

        match expr.operator.get_type() {
            TokenType::Minus => {
                let res = left_value.get_double()? - right_value.get_double()?;
                Ok(Object::Number(res))
            }
            TokenType::Slash => {
                let res = left_value.get_double()? / right_value.get_double()?;
                Ok(Object::Number(res))
            }
            TokenType::Star => {
                let res = left_value.get_double()? * right_value.get_double()?;
                Ok(Object::Number(res))
            }
            TokenType::Less => {
                let res = left_value.get_double()? < right_value.get_double()?;
                Ok(Object::Bool(res))
            }
            TokenType::LessEqual => {
                let res = left_value.get_double()? <= right_value.get_double()?;
                Ok(Object::Bool(res))
            }
            TokenType::Greater => {
                let res = left_value.get_double()? > right_value.get_double()?;
                Ok(Object::Bool(res))
            }
            TokenType::GreaterEqual => {
                let res = left_value.get_double()? >= right_value.get_double()?;
                Ok(Object::Bool(res))
            }
            TokenType::EqualEqual => {
                let res = left_value.get_double()? == right_value.get_double()?;
                Ok(Object::Bool(res))
            }
            TokenType::BangEqual => {
                let res = left_value.get_double()? != right_value.get_double()?;
                Ok(Object::Bool(res))
            }
            TokenType::Plus => match (left_value, right_value) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                (Object::String_(l), Object::String_(r)) => Ok(Object::String_(l + &r)),
                (_, _) => Err(RuntimeException::RuntimeError(RuntimeError(
                    "Operands must be two Number or two String!".to_string(),
                ))),
            },
            _ => unreachable!(),
        }
    }

    fn visit_assign(&mut self, expr: &Assign) -> Result<Object, RuntimeException> {
        let value_ = self.evalulate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(&expr.target.name, &value_)?;
        Ok(value_)
    }

    fn visit_logical(&mut self, expr: &Logical) -> Result<Object, RuntimeException> {
        let left_value = self.evalulate(&expr.left)?;
        if expr.op.get_type() == TokenType::Or {
            if left_value.get_bool() {
                return Ok(Object::Bool(true));
            }
        } else if !left_value.get_bool() {
            return Ok(Object::Bool(false));
        }
        self.evalulate(&expr.right)
    }

    fn visit_call(&mut self, expr: &Call) -> Result<Object, RuntimeException> {
        let callee = self.evalulate(&expr.callee)?;
        let mut arguments = Vec::new();
        for argument in &expr.arguments {
            arguments.push(self.evalulate(argument)?);
        }
        // TODO: unimplemented callable here
        if let Object::Callables(Callables::Function(mut function)) = callee {
            return Ok(function.call(self, &arguments));
        } else {
            panic!("unimplemented!")
        }
    }
}

impl StmtVisitor<Result<(), RuntimeException>> for Interpreter {
    fn visit_var_decl(&mut self, stmt: &VarDecl) -> Result<(), RuntimeException> {
        let value = if let Expr::Literal(Literal { value: Object::Nil }) = *stmt.initializer {
            Object::Nil
        } else {
            self.evalulate(&stmt.initializer)?
        };
        self.environment
            .borrow_mut()
            .define(stmt.name.get_lexeme(), value);
        Ok(())
    }

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Result<(), RuntimeException> {
        self.evalulate(&stmt.expr).map(|_| ())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), RuntimeException> {
        self.evalulate(&stmt.expr).map(|value| {
            println!("{}", value);
        })
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), RuntimeException> {
        self.execute_block(
            &stmt.statements, // only need a slice here
            Environment::new_with_enclosing(self.environment.clone()),
        )
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), RuntimeException> {
        let condtion_bool = self.evalulate(&stmt.condition)?.get_bool();
        if condtion_bool {
            self.execute(&stmt.then_branch)
        } else if let Some(then_) = stmt.else_branch.as_ref() {
            self.execute(then_)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), RuntimeException> {
        while self.evalulate(&stmt.condition)?.get_bool() {
            self.execute(&stmt.body)?;
        }
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<(), RuntimeException> {
        let func = Callables::Function(Function::new(stmt.clone()));
        self.environment
            .borrow_mut()
            .define(stmt.name.get_lexeme(), Object::Callables(func));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<(), RuntimeException> {
        let value = if *stmt.value != Expr::Literal(Literal { value: Object::Nil }) {
            self.evalulate(&stmt.value)?
        } else {
            Object::Nil
        };
        Err(RuntimeException::Return_(value))
    }
}
