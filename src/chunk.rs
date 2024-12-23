use crate::value::Value;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub struct Chunk {
    // The book also has counts for allocated and used capacity.
    // We will use a Vec<u8> to store the bytecode instead.
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
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
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn disassemble(&self, name: &str) -> Result<String, fmt::Error> {
        let mut output = String::new();
        writeln!(&mut output, "== {} ==", name)?;
        for (i, byte) in self.code.iter().enumerate() {
            self.disassemble_instruction(&mut output, i, byte)?;
        }
        Ok(output)
    }

    pub fn disassemble_instruction(
        &self,
        output: &mut dyn Write,
        offset: usize,
        byte: &u8,
    ) -> Result<(), fmt::Error> {
        write!(output, "{:04} ", offset)?;
        if offset > 0 && self.code[offset - 1] == *byte {
            write!(output, "   | ")?;
        } else {
            write!(output, "{:04} ", offset)?;
        }

        match *byte {
            0 => self.simple_instruction(output, "OP_RETURN"),
            1 => self.simple_instruction(output, "OP_CONSTANT"),
            2 => self.simple_instruction(output, "OP_NIL"),
            3 => self.simple_instruction(output, "OP_TRUE"),
            4 => self.simple_instruction(output, "OP_FALSE"),
            5 => self.simple_instruction(output, "OP_ADD"),
            6 => self.simple_instruction(output, "OP_SUBTRACT"),
            7 => self.simple_instruction(output, "OP_MULTIPLY"),
            8 => self.simple_instruction(output, "OP_DIVIDE"),
            _ => writeln!(output, "Unknown opcode {}", byte),
        }
    }

    pub fn simple_instruction(&self, output: &mut dyn Write, name: &str) -> Result<(), fmt::Error> {
        writeln!(output, "{}", name)
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
        chunk.write(0);
        assert_eq!(chunk.code.len(), 1);
        assert_eq!(chunk.code[0], 0);
    }

    #[test]
    fn test_chunk_disassemble() {
        let mut chunk = Chunk::new();
        chunk.write(0);
        chunk.write(1);
        chunk.write(2);
        chunk.write(3);
        chunk.write(4);
        chunk.write(5);
        chunk.write(6);
        chunk.write(7);
        let result = chunk.disassemble("test");
        assert!(result.is_ok());
        let result = result.unwrap();

        let expected = concat!(
            "== test ==\n",
            "0000 0000 OP_RETURN\n",
            "0001 0001 OP_CONSTANT\n",
            "0002 0002 OP_NIL\n",
            "0003 0003 OP_TRUE\n",
            "0004 0004 OP_FALSE\n",
            "0005 0005 OP_ADD\n",
            "0006 0006 OP_SUBTRACT\n",
            "0007 0007 OP_MULTIPLY\n",
        );
        println!("{}", result);
        assert_eq!(expected, result);
    }
}
