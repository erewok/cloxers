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
}
