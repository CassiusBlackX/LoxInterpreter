use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::RuntimeError;
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

    pub fn get(&self, name: &str) -> Result<Object, RuntimeError> {
        if self.values.contains_key(name) {
            Ok(self.values.get(name).unwrap().clone())
        } else if let Some(enclose) = self.enclosing.as_ref() {
            Ok(enclose.borrow().get(name)?)
        } else {
            Err(RuntimeError(format!("Undefined variable: {}", name)))
        }
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<Object, RuntimeError> {
        if distance == 0 {
            self.get(name)
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow()
                .get_at(distance - 1, name)
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

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &Token,
        value: &Object,
    ) -> Result<(), RuntimeError> {
        if distance == 0 {
            self.values
                .insert(name.get_lexeme().to_string(), value.to_owned());
            Ok(())
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(distance - 1, name, value)
        }
    }
}
