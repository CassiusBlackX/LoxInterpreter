use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::interpreter::{RuntimeError, RuntimeException};
use crate::stmt::*;
use crate::{interpreter::Interpreter, object::Object};

#[derive(Debug, PartialEq, Clone)]
pub enum Callables {
    Function(Function),
}

impl fmt::Display for Callables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function(function) => write!(f, "{function}"),
        }
    }
}

pub trait Callable {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object;
    fn arity(&self) -> usize;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    declaration: Box<FunctionStmt>,
}

impl Function {
    pub fn new(decl: FunctionStmt) -> Self {
        Self {
            declaration: Box::new(decl),
        }
    }
}

impl Callable for Function {
    fn call(
        &mut self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: &[crate::object::Object],
    ) -> crate::object::Object {
        let mut environment = Environment::new_with_enclosing(Rc::clone(&interpreter.globals));
        for (index, arg) in self.declaration.params.iter().enumerate() {
            environment.define(arg.get_lexeme(), arguments.get(index).unwrap().clone());
        }
        let result = interpreter.execute_block(&self.declaration.body, environment);
        match result {
            Ok(_) => Object::Nil,
            Err(RuntimeException::Return_(value)) => value,
            Err(RuntimeException::RuntimeError(RuntimeError(e))) => {
                eprintln!("{}", e);
                panic!("Err when calling function");
            }
        }
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}>", self.declaration.name.get_lexeme())
    }
}
