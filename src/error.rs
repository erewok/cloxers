use std::fmt;
use std::option::Option;
use std::process;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum CloxersError {
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("BadInstruction: {0}")]
    BadInstruction(String),

    #[error("OpCodeError: {code}")]
    OpCodeError { code: u8 },

    #[error("Too Many Constants")]
    ConstantsOverflowed,

    #[error("CompilerError: {0}")]
    CompilerError(#[from] CompilerError),

    #[error("TypeError: {0}")]
    TypeError(String),
}

impl CloxersError {
    pub fn exit(&self) -> i32 {
        match self {
            CloxersError::FmtError(_) => process::exit(70),
            CloxersError::IoError(_) => process::exit(70),
            CloxersError::BadInstruction(_) => process::exit(64),
            CloxersError::OpCodeError { .. } => process::exit(65),
            CloxersError::ConstantsOverflowed => process::exit(70),
            CloxersError::TypeError(_) => process::exit(65),
            CloxersError::CompilerError(inner) => match inner {
                CompilerError::CompileError => process::exit(65),
                CompilerError::RuntimeError => process::exit(70),
                CompilerError::ScannerError(_) => process::exit(65),
                CompilerError::SyntaxError(_) => process::exit(65),
            },
        }
    }
}

#[derive(Error, Diagnostic, Debug)]
pub enum CompilerError {
    CompileError,
    RuntimeError,
    ScannerError(Option<String>),
    SyntaxError(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::CompileError => write!(f, "Compile error"),
            CompilerError::RuntimeError => write!(f, "Runtime error"),
            CompilerError::ScannerError(Some(s)) => write!(f, "Scanner error: {}", s),
            CompilerError::ScannerError(None) => write!(f, "Scanner error"),
            CompilerError::SyntaxError(s) => write!(f, "Syntax error: {}", s),
        }
    }
}
