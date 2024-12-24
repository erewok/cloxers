use std::fmt;

use crate::error::CloxersError;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
            _ => Err(CloxersError::TypeError("Cannot negate Nul".to_string())),
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, CloxersError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err(CloxersError::TypeError("Operands must be numbers".to_string())),
        }
    }
    pub fn subtract(&self, other: &Value) -> Result<Value, CloxersError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(CloxersError::TypeError("Operands must be numbers".to_string())),
        }
    }
    pub fn multiply(&self, other: &Value) -> Result<Value, CloxersError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(CloxersError::TypeError("Operands must be numbers".to_string())),
        }
    }
    pub fn divide(&self, other: &Value) -> Result<Value, CloxersError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err(CloxersError::TypeError("Operands must be numbers".to_string())),
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
