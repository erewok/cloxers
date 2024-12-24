use std::fmt;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum CloxersError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("BadInstruction: {0}")]
    BadInstruction(String),

    #[error("OpCodeError: {code}")]
    OpCodeError { code: u8 },

    #[error("Too Many Constants")]
    ConstantsOverflowed,

    #[error("InterpreterError: {0}")]
    InterpreterError(#[from] InterpreterError),

    #[error("TypeError: {0}")]
    TypeError(String),
}

#[derive(Error, Diagnostic, Debug)]
pub enum InterpreterError {
    CompileError,
    RuntimeError,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::CompileError => write!(f, "Compile error"),
            InterpreterError::RuntimeError => write!(f, "Runtime error"),
        }
    }
}
