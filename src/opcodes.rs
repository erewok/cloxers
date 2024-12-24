use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    // takes 0 operands
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    // takes 1 operand
    Constant,
}

impl OpCode {
    pub fn name(&self) -> &'static str {
        match self {
            OpCode::Return => "OP_RETURN",
            OpCode::Negate => "OP_NEGATE",
            OpCode::Add => "OP_ADD",
            OpCode::Subtract => "OP_SUBTRACT",
            OpCode::Multiply => "OP_MULTIPLY",
            OpCode::Divide => "OP_DIVIDE",
            OpCode::Constant => "OP_CONSTANT",
        }
    }

    pub fn operand_offset(&self) -> usize {
        match self {
            OpCode::Return => 0,
            OpCode::Negate => 0,
            OpCode::Constant => 1,
            OpCode::Add => 0,
            OpCode::Subtract => 0,
            OpCode::Multiply => 0,
            OpCode::Divide => 0,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
