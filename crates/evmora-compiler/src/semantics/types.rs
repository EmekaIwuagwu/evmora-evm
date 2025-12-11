use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Simple(String),           // uint256, address, bool, etc.
    Mapping(Box<Type>, Box<Type>), // mapping[key => value]
    List(Box<Type>),          // list[T]
    Tuple(Vec<Type>),         // (T1, T2, ...)
    Optional(Box<Type>),      // T?
    Unknown,                  // For type inference
}

impl Type {
    pub fn uint256() -> Self { Type::Simple("uint256".to_string()) }
    pub fn uint8() -> Self { Type::Simple("uint8".to_string()) }
    pub fn address() -> Self { Type::Simple("address".to_string()) }
    pub fn bool() -> Self { Type::Simple("bool".to_string()) }
    pub fn bytes32() -> Self { Type::Simple("bytes32".to_string()) }
    pub fn string() -> Self { Type::Simple("string".to_string()) }

    pub fn is_numeric(&self) -> bool {
        match self {
            Type::Simple(s) => s.starts_with("uint") || s.starts_with("int"),
            _ => false,
        }
    }

    pub fn is_compatible(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            (Type::Simple(a), Type::Simple(b)) => {
                if a == b { return true; }
                // Numeric promotion: smaller -> larger
                if self.is_numeric() && other.is_numeric() {
                    return self.numeric_size() <= other.numeric_size();
                }
                false
            }
            (Type::Mapping(k1, v1), Type::Mapping(k2, v2)) => {
                k1.is_compatible(k2) && v1.is_compatible(v2)
            }
            (Type::List(t1), Type::List(t2)) => t1.is_compatible(t2),
            (Type::Tuple(ts1), Type::Tuple(ts2)) => {
                ts1.len() == ts2.len() && 
                ts1.iter().zip(ts2.iter()).all(|(a, b)| a.is_compatible(b))
            }
            (Type::Optional(t1), Type::Optional(t2)) => t1.is_compatible(t2),
            _ => false,
        }
    }

    fn numeric_size(&self) -> u32 {
        match self {
            Type::Simple(s) => {
                if s.starts_with("uint") || s.starts_with("int") {
                    s.trim_start_matches("uint")
                        .trim_start_matches("int")
                        .parse::<u32>()
                        .unwrap_or(256)
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Simple(s) => write!(f, "{}", s),
            Type::Mapping(k, v) => write!(f, "mapping[{} => {}]", k, v),
            Type::List(t) => write!(f, "list[{}]", t),
            Type::Tuple(ts) => {
                write!(f, "(")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Optional(t) => write!(f, "{}?", t),
            Type::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch { expected: String, found: String },
    
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    
    #[error("Undefined function: {0}")]
    UndefinedFunction(String),
    
    #[error("Duplicate definition: {0}")]
    DuplicateDefinition(String),
    
    #[error("Invalid decorator '{decorator}' on {target}")]
    InvalidDecorator { decorator: String, target: String },
    
    #[error("View function {0} cannot modify state")]
    ViewFunctionModifiesState(String),
    
    #[error("Missing required decorator '{decorator}' on {target}")]
    MissingDecorator { decorator: String, target: String },
    
    #[error("Invalid number of arguments: expected {expected}, found {found}")]
    InvalidArgumentCount { expected: usize, found: usize },
    
    #[error("Return type mismatch in function {function}: expected {expected}, found {found}")]
    ReturnTypeMismatch { function: String, expected: String, found: String },
    
    #[error("Variable {0} used before initialization")]
    UninitializedVariable(String),
    
    #[error("Invalid operation {op} on types {left} and {right}")]
    InvalidOperation { op: String, left: String, right: String },
}

pub type SemanticResult<T> = Result<T, SemanticError>;
