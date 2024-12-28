use crate::chunk::{self, Chunk};
use crate::error::{CloxersError, CompilerError};
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::value::Value;
use crate::vm::VM;

enum WhichToken {
    Prev,
    Curr,
}

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    previous: Option<Token>,
    current: Option<Token>,
    panic_mode: bool,
    is_error: bool,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        let scanner = Scanner::new(source);
        Self {
            scanner,
            previous: None,
            current: None,
            panic_mode: false,
            is_error: false,
        }
    }

    pub fn interpret(&mut self) -> Result<(), CloxersError> {
        // let mut chunk = Chunk::new();
        // let compilation_result = self.compile(&mut chunk)?;
        // let mut vm = VM::new(&chunk);
        // vm.run()
        todo!()
    }

    pub fn compile(&mut self) -> Result<Chunk, CloxersError> {
        let mut chunk = Chunk::new();
        Ok(chunk)
    }

    pub fn expression(&mut self, chunk: &mut Chunk) {
        // self.current += 1;
    }

    /// Token management functions: advance, consume, and helpers
    pub fn advance(&mut self) -> Result<(), CloxersError> {
        self.previous = self.current.take();
        self.current = self.scanner.scan_next_token().map_err(|e| {
            self.error_at_token("Next token failure", WhichToken::Prev);
            CloxersError::from(e)
        })?;
        Ok(())
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), CloxersError> {
        if self
            .current
            .as_ref()
            .map_or(false, |t| t.token_type == token_type)
        {
            self.advance()?;
            Ok(())
        } else {
            let msg = format!("Expected {}. {}", token_type, message);
            self.error_at_token(msg.as_str(), WhichToken::Curr);
            Err(CloxersError::from(CompilerError::SyntaxError(msg)))
        }
    }

    ///
    pub fn grouping(&mut self, chunk: &mut Chunk) -> Result<(), CloxersError> {
        self.expression(chunk);
        self.consume(TokenType::RightParen, "Expect ')' after expression")?;
        Ok(())
    }

    pub fn number(&self, chunk: &mut Chunk) -> Result<(), CloxersError> {
        let number = self
            .previous
            .as_ref()
            .and_then(|t| t.lexeme.as_ref())
            .and_then(|s| s.parse::<f64>().ok())
            .map(|n| Value::Number(n));

        let line = self.get_prev_line();
        if let Some(num) = number {
            chunk.write_constant(num, line)
        } else {
            Err(CloxersError::from(CompilerError::SyntaxError(
                "Invalid number".to_string(),
            )))
        }
    }
    /// Chunk writers: raw bytes
    pub fn emit_byte(&mut self, chunk: &mut Chunk, byte: u8) {
        chunk.write(byte, self.get_prev_line());
    }
    pub fn emit_bytes(&mut self, chunk: &mut Chunk, byte1: u8, byte2: u8) {
        self.emit_byte(chunk, byte1);
        self.emit_byte(chunk, byte2);
    }

    fn get_prev_line(&self) -> usize {
        self.previous.as_ref().map_or(0, |t| t.line)
    }

    /// Reporting errors
    ///
    fn error_at_token(&mut self, msg: &str, which: WhichToken) {
        // no error cascades
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.is_error = true;
        let token = match which {
            WhichToken::Prev => &self.previous,
            WhichToken::Curr => &self.current,
        };
        match token {
            None => {
                eprintln!("Error at end: {}", msg);
                return;
            }
            Some(tok) => {
                if tok.token_type == crate::token::TokenType::Eof {
                    eprintln!(
                        "[line {}: col {}] Error at end: {}",
                        tok.line, tok.column, msg
                    );
                } else if tok.token_type == crate::token::TokenType::TokenError {
                    eprintln!("[line {}: col {}] Error: {}", tok.line, tok.column, msg);
                } else {
                    eprintln!(
                        "[line {}: col {}] Error at '{}': {}",
                        tok.line,
                        tok.column,
                        tok.lexeme.as_deref().unwrap_or_default(),
                        msg
                    );
                }
            }
        }
    }
}
