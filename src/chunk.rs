use std::fmt::Write;

use miette::{miette, Context, IntoDiagnostic, Result};

use crate::error::CloxersError;
use crate::opcodes::OpCode;
use crate::value::Value;

/// A chunk of bytecode.
/// This struct implements a custom IntoIterator so we can iterate over OpCodes *only*
/// and not the operands.
#[derive(Debug, Clone)]
pub struct Chunk {
    // The book also has counts for allocated and used capacity.
    // We will use a Vec<u8> to store the bytecode instead.
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>, // line numbers for debugging
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// Writes a byte to the chunk: may be opcode or operand.
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Writes a constant to the chunk: opcode followed by operand's index (as u8) in constants vec.
    pub fn write_constant(&mut self, value: Value, line: usize) -> Result<()> {
        // Is this an off-by-one error?
        if self.constants.len() > (u8::MAX as usize) {
            return Err(CloxersError::ConstantsOverflowed).into_diagnostic();
        }
        let index = self.add_constant(value);
        let index = index
            .try_into()
            .map_err(|_| CloxersError::OpCodeError { code: index as u8 })
            .into_diagnostic()
            .wrap_err("Cannot convert constant index to u8")?;
        self.write(OpCode::Constant.into(), line);
        self.write(index, line);
        Ok(())
    }

    /// Adds a constant to the chunk and returns the index of the constant.
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) -> Result<String> {
        let mut output = String::new();
        writeln!(&mut output, "== {} ==", name)
            .map_err(|_| miette!("Cannot write disassembly header for {}", name))?;
        for (idx, bytearray) in self.into_iter().enumerate() {
            // instruction index starts at 1 for disassembly
            self.disassemble_instruction(&mut output, idx + 1, &bytearray)?;
        }
        Ok(output)
    }

    pub fn disassemble_instruction(
        &self,
        output: &mut dyn Write,
        idx: usize,
        bytearray: &[u8; 3],
    ) -> Result<()> {
        let [op_code_byte, op1_offset, _op2_offset] = bytearray;
        write!(output, "{}. {:04} ", idx, op_code_byte)
            .map_err(|_| miette!("Cannot write offset at 0"))?;

        match OpCode::try_from(*op_code_byte) {
            Ok(op_code) => match op_code {
                OpCode::Return => self.simple_instruction(output, op_code.name()),
                OpCode::Constant => self.constant_instruction(output, op_code.name(), op1_offset),
                OpCode::Add => self.simple_instruction(output, op_code.name()),
                OpCode::Subtract => self.simple_instruction(output, op_code.name()),
                OpCode::Multiply => self.simple_instruction(output, op_code.name()),
                OpCode::Divide => self.simple_instruction(output, op_code.name()),
            },
            Err(e) => Err(CloxersError::OpCodeError {
                code: *op_code_byte,
            })
            .into_diagnostic()
            .context(e)
            .wrap_err("Cannot disassemble instruction"),
        }
    }

    /// Writes a simple instruction to the output.
    pub fn simple_instruction(&self, output: &mut dyn Write, name: &str) -> Result<()> {
        writeln!(output, "{}", name).map_err(|_| miette!("Cannot write simple instruction"))
    }

    /// Writes a constant instruction to the output.
    pub fn constant_instruction(
        &self,
        output: &mut dyn Write,
        name: &str,
        offset: &u8, // this byte refers to an index in the constants vec
    ) -> Result<()> {
        let offset = *offset as usize;
        if offset >= self.code.len() {
            return Err(CloxersError::BadInstruction(format!(
                "Missing constant index {}",
                offset
            )))
            .into_diagnostic()
            .wrap_err("Cannot disassemble constant instruction");
        }
        self.constants
            .get(offset)
            .map(|value| {
                let _ = writeln!(output, "{:<16}\t{} => {}", name, offset, value)
                    .map_err(|_| miette!("Cannot write constant instruction header"));
            })
            .ok_or_else(|| {
                let _ = write!(output, "{:<16} {} <invalid constant>", name, offset);
                miette!("Cannot write constant instruction")
            })?;
        Ok(())
    }
}

/// We will implement an iterator for the Chunk struct so we can iterate
/// over the bytecode *and* operands in the chunk.
pub struct ChunkIter<'a> {
    // Program counter should always point to an OppCode u8.
    pc: usize,
    code: &'a [u8],
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = [u8; 3]; // OpCode + up to 2 operands

    fn next(&mut self) -> Option<Self::Item> {
        if self.pc < self.code.len() {
            let op = self.code[self.pc];
            let op_code = OpCode::try_from(op).ok()?;
            let offset = op_code.operand_offset();
            self.pc += 1; // move to next byte
            match offset {
                0 => Some([op, 0, 0]),
                1 => {
                    let operand = self.code[self.pc];
                    self.pc += 1; // advance beyond the operand
                    Some([op, operand, 0])
                }
                2 => {
                    let operand1 = self.code[self.pc];
                    let operand2 = self.code[self.pc + 1];
                    self.pc += 2; // advance beyond the two operands
                    Some([op, operand1, operand2])
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = [u8; 3];
    type IntoIter = ChunkIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIter {
            pc: 0,
            code: &self.code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_new() {
        let chunk = Chunk::new();
        assert_eq!(chunk.code.len(), 0);
    }

    #[test]
    fn test_chunk_write() {
        let mut chunk = Chunk::new();
        chunk.write(OpCode::Return.into(), 0);
        assert_eq!(chunk.code.len(), 1);
        assert_eq!(chunk.code[0], OpCode::Return.try_into().unwrap());
    }

    #[test]
    fn test_chunk_disassemble() {
        let mut chunk = Chunk::new();
        chunk.write(OpCode::Return.into(), 5);
        let _ = chunk.write_constant(Value::Number(1.2), 1);
        let _ = chunk.write_constant(Value::Number(-5.0),1 );
        let _ = chunk.write(OpCode::Add.into(), 2);
        let _ = chunk.write(OpCode::Subtract.into(), 3);
        let _ = chunk.write(OpCode::Multiply.into(), 4);
        let result = chunk.disassemble("test");
        println!("{:?}", result);
        assert!(result.is_ok());

        let result = result.unwrap();

        let expected = concat!(
            "== test ==\n",
            "1. 0000 OP_RETURN\n",
            "2. 0001 OP_CONSTANT     	0 => 1.2\n",
            "3. 0001 OP_CONSTANT     	1 => -5\n",
            "4. 0002 OP_ADD\n",
            // we haven't implemented these yet so pc counter will go too far; no panics though!
            // "5. 0006 OP_SUBTRACT\n",
            // "6. 0007 OP_MULTIPLY\n",
        );
        println!("{}", result);
        assert_eq!(expected, result);
    }
}
