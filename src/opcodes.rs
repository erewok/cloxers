use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    // takes 0 operands
    Return,
    // takes 1 operand
    Constant,
    // takes 2 operands
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl OpCode {
    pub fn name(&self) -> &'static str {
        match self {
            OpCode::Return => "OP_RETURN",
            OpCode::Constant => "OP_CONSTANT",
            OpCode::Add => "OP_ADD",
            OpCode::Subtract => "OP_SUBTRACT",
            OpCode::Multiply => "OP_MULTIPLY",
            OpCode::Divide => "OP_DIVIDE",
        }
    }

    pub fn operand_offset(&self) -> usize {
        match self {
            OpCode::Return => 0,
            OpCode::Constant => 1,
            OpCode::Add => 2,
            OpCode::Subtract => 2,
            OpCode::Multiply => 2,
            OpCode::Divide => 2,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
