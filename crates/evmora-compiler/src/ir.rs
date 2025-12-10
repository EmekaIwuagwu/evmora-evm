use primitive_types::U256;

#[derive(Debug, Clone)]
pub enum IrStatement {
    Push(U256),
    Pop,
    Add,
    Sub,
    Sha3, // For mapping calculations
    Eq,
    IsZero,
    And,
    Or,
    Not,
    Caller,
    CallValue,
    CallDataLoad(usize), // Offset
    Store { offset: usize }, // Memory store
    Load { offset: usize }, // Memory load
    SStore, // Storage store (key, value on stack)
    SLoad, // Storage load (key on stack)
    Return { offset: usize, size: usize },
    Revert { offset: usize, size: usize },
    Jump(String), // Jump to label
    JumpI(String), // Jump if non-zero
    Label(String),
    FunctionCall { name: String, args: Vec<IrStatement> },
    Stop,
}

#[derive(Debug, Clone)]
pub struct IrProgram {
    pub statements: Vec<IrStatement>,
}

impl IrProgram {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }
}
