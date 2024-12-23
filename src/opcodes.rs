#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    Nil,
    True,
    False,

    Add,
    Subtract,
    Multiply,
    Divide,
}