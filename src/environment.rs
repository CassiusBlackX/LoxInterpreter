use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        if self.values.contains_key(name.get_lexeme()) {
            Ok(self.values.get(name.get_lexeme()).unwrap().clone())
        } else if let Some(enclose) = self.enclosing.as_ref() {
            Ok(enclose.borrow().get(name)?)
        } else {
            Err(RuntimeError(format!(
                "Undefined variable: {}",
                name.get_lexeme()
            )))
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(name.get_lexeme()) {
            self.values
                .insert(name.get_lexeme().to_string(), value.clone());
            Ok(())
        } else if let Some(enclose) = self.enclosing.as_ref() {
            enclose.borrow_mut().assign(name, value)?;
            Ok(())
        } else {
            Err(RuntimeError(format!(
                "Undefined variable: {}",
                name.get_lexeme()
            )))
        }
    }
}
