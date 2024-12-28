use crate::error::CloxersError;
use crate::scanner::Scanner;
use crate::vm::VM;

pub struct Interpreter {
    source: String,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            source: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.source.clear();
    }

    pub fn run(&mut self, source: &str) -> Result<(), CloxersError> {
        Ok(())
    }
}
