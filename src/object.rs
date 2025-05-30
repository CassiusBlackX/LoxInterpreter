use std::fmt;

use crate::callable::Callables;
use crate::interpreter::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Identifier(String),
    String_(String),
    Bool(bool),
    Nil,
    Number(f64),
    Callables(Callables),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "{ident}"),
            Self::String_(content) => write!(f, "{content}"),
            Self::Number(x) => write!(f, "{x}"),
            Self::Bool(flag) => write!(f, "{}", if *flag { "true" } else { "false" }),
            Self::Nil => write!(f, "Nil"),
            Self::Callables(callable) => write!(f, "{callable}"),
        }
    }
}

impl Object {
    pub fn get_bool(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(x) => *x,
            _ => true,
        }
    }

    pub fn get_double(&self) -> Result<f64, RuntimeError> {
        match self {
            Self::Number(x) => Ok(*x),
            _ => Err(RuntimeError(format!(
                "Cannot cast {} into double!",
                self.to_string()
            ))),
        }
    }
}
