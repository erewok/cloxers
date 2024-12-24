use miette::Result;

use crate::chunk;
use crate::error;
use crate::opcodes::OpCode;
use crate::value;

pub struct VM<'a> {
    chunk: &'a chunk::Chunk,
    ip: usize,
    stack: Vec<value::Value>,
}

impl VM<'_> {
    pub fn new<'a>(chunk: &'a chunk::Chunk) -> VM<'a> {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<()> {
        for bytearray in self.chunk.into_iter() {
            let [op_code_byte, op1_offset, _op2_offset] = bytearray;
            match OpCode::try_from(op_code_byte) {
                Ok(op_code) => match op_code {
                    OpCode::Return => {
                        let val = self.stack.pop().ok_or_else(|| {
                            error::CloxersError::BadInstruction("Stack underflow".to_string())
                        })?;
                        println!("RETURN: {}", val);
                        return Ok(())
                    },
                    OpCode::Constant => {
                        let constant =
                            self.chunk
                                .read_constant(op1_offset as usize)
                                .ok_or_else(|| {
                                    error::CloxersError::BadInstruction(format!(
                                        "Missing constant at index {}",
                                        op1_offset
                                    ))
                                })?;
                        self.stack.push(constant.clone());
                    }
                    OpCode::Negate => {
                        let val = self.stack.pop()
                        .ok_or_else(|| {
                            error::CloxersError::BadInstruction("Stack underflow".to_string())
                        })?;
                        let new_val = val.negate()?;
                        self.stack.push(new_val);
                    }
                    _ => todo!(),
                },
                Err(err) => todo!(),
            }
        }
        Ok(())
    }
}
