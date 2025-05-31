use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::interpreter::{RuntimeError, RuntimeException};
use crate::stmt::*;
use crate::token::Token;
use crate::{interpreter::Interpreter, object::Object};

#[derive(Debug, Clone)]
pub enum Callables {
    Function(Function),
    Class(Class),
    Instance(Rc<RefCell<Instance>>),
}

impl fmt::Display for Callables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function(function) => write!(f, "{function}"),
            Self::Class(class) => write!(f, "{class}"),
            Self::Instance(instance) => write!(f, "{}", instance.borrow()),
        }
    }
}

impl PartialEq for Callables {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub trait Callable {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, RuntimeError>;
    fn arity(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Function {
    declaration: Box<FunctionStmt>,
    closure: Rc<RefCell<Environment>>,
    is_initialzer: bool,
}

impl Function {
    pub fn new(
        decl: FunctionStmt,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        Self {
            declaration: Box::new(decl),
            closure,
            is_initialzer: is_initializer,
        }
    }

    pub fn bind(self, instance: Rc<RefCell<Instance>>) -> Self {
        let mut environment = Environment::new_with_enclosing(self.closure);
        environment.define(
            "this",
            Object::Callables(Callables::Instance(instance.clone())),
        );
        Self {
            declaration: self.declaration,
            closure: Rc::new(RefCell::new(environment)),
            is_initialzer: self.is_initialzer,
        }
    }
}

impl Callable for Function {
    fn call(
        &mut self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: &[crate::object::Object],
    ) -> Result<Object, RuntimeError> {
        let mut environment = Environment::new_with_enclosing(self.closure.clone());
        for (index, arg) in self.declaration.params.iter().enumerate() {
            environment.define(arg.get_lexeme(), arguments.get(index).unwrap().clone());
        }
        let result = interpreter.execute_block(&self.declaration.body, environment);
        match result {
            Ok(_) => {}
            Err(RuntimeException::Return_(value)) => {
                if self.is_initialzer {
                    return self.closure.borrow().get_at(0, "this");
                }
                return Ok(value);
            }
            Err(RuntimeException::RuntimeError(e)) => return Err(e),
        }

        if self.is_initialzer {
            self.closure.borrow().get_at(0, "this")
        } else {
            Ok(Object::Nil)
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

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    methods: HashMap<String, Function>,
    superclass: Option<Box<Class>>,
}

impl Class {
    pub fn new(
        name: String,
        methods: HashMap<String, Function>,
        superclass: Option<Box<Class>>,
    ) -> Self {
        Self {
            name,
            methods,
            superclass,
        }
    }

    pub fn find_method(&self, name: &str) -> Result<Function, RuntimeError> {
        if self.methods.contains_key(name) {
            return Ok(self.methods.get(name).unwrap().clone());
        }

        if let Some(super_class) = &self.superclass {
            return super_class.find_method(name);
        }

        Err(RuntimeError(format!("no method found in {}", self)))
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl Callable for Class {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, RuntimeError> {
        let instance = Rc::new(RefCell::new(Instance::new(Rc::new(self.clone()))));

        let initializer = self.find_method("init");
        if let Ok(init) = initializer {
            init.bind(instance).call(interpreter, arguments)
        } else {
            Ok(Object::Callables(Callables::Instance(instance)))
        }
    }

    fn arity(&self) -> usize {
        match self.find_method("init") {
            Ok(initializer) => initializer.arity(),
            Err(_) => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    class: Rc<Class>,
    fields: HashMap<String, Object>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self {
            class: class.clone(),
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        if self.fields.contains_key(name.get_lexeme()) {
            return Ok(self.fields.get(name.get_lexeme()).unwrap().clone());
        }

        if let Ok(method) = self.class.find_method(name.get_lexeme()) {
            let method = method.bind(Rc::new(RefCell::new(self.to_owned())));
            return Ok(Object::Callables(Callables::Function(method)));
        }

        Err(RuntimeError(format!(
            "Undefined property '{}'",
            name.get_lexeme()
        )))
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.get_lexeme().to_string(), value);
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "instance of {}", self.class)
    }
}
