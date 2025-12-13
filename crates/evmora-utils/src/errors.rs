use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvmError {
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Out of gas")]
    OutOfGas,
    #[error("Invalid opcode: {0:#x}")]
    InvalidOpcode(u8),
    #[error("Invalid jump destination")]
    InvalidJump,
    #[error("Memory access violation")]
    MemoryViolation,
    #[error("Memory limit exceeded")]
    MemoryLimitExceeded,
    #[error("Call depth exceeded")]
    CallDepthExceeded,
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Contract creation failed")]
    ContractCreationFailed,
    #[error("Context error: {0}")]
    ContextError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Execution reverted")]
    Reverted(Vec<u8>),
}
