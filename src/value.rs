use std::fmt;

use crate::error::CloxersError;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        matches!(self, Value::Nil | Value::Bool(false))
    }
    pub fn negate(&self) -> Result<Value, CloxersError> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Nil => Err(CloxersError::TypeError("Cannot negate Nul".to_string())),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}
