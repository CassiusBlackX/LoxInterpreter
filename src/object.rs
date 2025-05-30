use std::fmt;

use crate::callable::Callables;
use crate::interpreter::RuntimeError;

#[derive(Debug, Clone)]
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

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            false
        } else {
            match (self, other) {
                (Self::Identifier(ss), Self::Identifier(os)) => ss == os,
                (Self::String_(ss), Self::String_(os)) => ss == os,
                (Self::Bool(sb), Self::Bool(ob)) => sb == ob,
                (Self::Nil, Self::Nil) => true,
                (Self::Nil, _) => false,
                (_, Self::Nil) => false,
                (Self::Number(sx), Self::Number(ox)) => {
                    if sx.is_nan() && ox.is_nan() {
                        true
                    } else if sx.is_nan() || ox.is_nan() {
                        false
                    } else {
                        sx == ox
                    }
                }
                (Self::Callables(sc), Self::Callables(oc)) => sc == oc,
                (_, _) => false,
            }
        }
    }
}
