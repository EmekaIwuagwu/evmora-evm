use super::types::{Type, SemanticError, SemanticResult};
use super::symbol_table::SymbolTable;

pub struct TypeChecker<'a> {
    symbols: &'a SymbolTable,
}

impl<'a> TypeChecker<'a> {
    pub fn new(symbols: &'a SymbolTable) -> Self {
        Self { symbols }
    }

    pub fn check_binary_op(&self, op: &str, left: &Type, right: &Type) -> SemanticResult<Type> {
        match op {
            "+" | "-" | "*" | "/" | "%" => {
                if left.is_numeric() && right.is_numeric() {
                    // Return larger type
                    if left.is_compatible(right) {
                        Ok(right.clone())
                    } else if right.is_compatible(left) {
                        Ok(left.clone())
                    } else {
                        Err(SemanticError::InvalidOperation {
                            op: op.to_string(),
                            left: left.to_string(),
                            right: right.to_string(),
                        })
                    }
                } else {
                    Err(SemanticError::InvalidOperation {
                        op: op.to_string(),
                        left: left.to_string(),
                        right: right.to_string(),
                    })
                }
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                if left.is_compatible(right) || right.is_compatible(left) {
                    Ok(Type::bool())
                } else {
                    Err(SemanticError::TypeMismatch {
                        expected: left.to_string(),
                        found: right.to_string(),
                    })
                }
            }
            "&&" | "||" => {
                if *left == Type::bool() && *right == Type::bool() {
                    Ok(Type::bool())
                } else {
                    Err(SemanticError::InvalidOperation {
                        op: op.to_string(),
                        left: left.to_string(),
                        right: right.to_string(),
                    })
                }
            }
            _ => Err(SemanticError::InvalidOperation {
                op: op.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            }),
        }
    }

    pub fn check_unary_op(&self, op: &str, operand: &Type) -> SemanticResult<Type> {
        match op {
            "!" => {
                if *operand == Type::bool() {
                    Ok(Type::bool())
                } else {
                    Err(SemanticError::TypeMismatch {
                        expected: "bool".to_string(),
                        found: operand.to_string(),
                    })
                }
            }
            "-" | "+" => {
                if operand.is_numeric() {
                    Ok(operand.clone())
                } else {
                    Err(SemanticError::TypeMismatch {
                        expected: "numeric type".to_string(),
                        found: operand.to_string(),
                    })
                }
            }
            _ => Err(SemanticError::InvalidOperation {
                op: op.to_string(),
                left: operand.to_string(),
                right: "".to_string(),
            }),
        }
    }

    pub fn infer_literal_type(&self, literal: &str) -> Type {
        if literal == "true" || literal == "false" {
            Type::bool()
        } else if literal.starts_with("0x") && literal.len() == 42 {
            Type::address()
        } else if literal.starts_with("0x") {
            Type::bytes32()
        } else if literal.parse::<u64>().is_ok() {
            Type::uint256()
        } else if literal.starts_with('"') || literal.starts_with('\'') {
            Type::string()
        } else {
            Type::Unknown
        }
    }

    pub fn check_assignment(&self, target: &Type, value: &Type) -> SemanticResult<()> {
        if target.is_compatible(value) {
            Ok(())
        } else {
            Err(SemanticError::TypeMismatch {
                expected: target.to_string(),
                found: value.to_string(),
            })
        }
    }

    pub fn check_function_call(
        &self,
        func_name: &str,
        arg_types: &[Type],
    ) -> SemanticResult<Option<Type>> {
        // Built-in functions
        match func_name {
            "require" | "assert" => {
                if arg_types.len() >= 1 && arg_types[0] == Type::bool() {
                    return Ok(None);
                }
                return Err(SemanticError::InvalidArgumentCount {
                    expected: 1,
                    found: arg_types.len(),
                });
            }
            "address" => {
                if arg_types.len() == 1 {
                    return Ok(Some(Type::address()));
                }
            }
            "uint256" | "uint8" => {
                if arg_types.len() == 1 && arg_types[0].is_numeric() {
                    return Ok(Some(Type::Simple(func_name.to_string())));
                }
            }
            "safe_add" | "safe_sub" | "safe_mul" | "safe_div" => {
                if arg_types.len() == 2 && arg_types[0].is_numeric() && arg_types[1].is_numeric() {
                    return Ok(Some(arg_types[0].clone()));
                }
            }
            _ => {}
        }

        // User-defined functions
        if let Some(sig) = self.symbols.lookup_function(func_name) {
            if sig.params.len() != arg_types.len() {
                return Err(SemanticError::InvalidArgumentCount {
                    expected: sig.params.len(),
                    found: arg_types.len(),
                });
            }

            for (i, (_, param_type)) in sig.params.iter().enumerate() {
                if !param_type.is_compatible(&arg_types[i]) {
                    return Err(SemanticError::TypeMismatch {
                        expected: param_type.to_string(),
                        found: arg_types[i].to_string(),
                    });
                }
            }

            return Ok(sig.return_type.clone());
        }

        Err(SemanticError::UndefinedFunction(func_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_operations() {
        let symbols = SymbolTable::new();
        let checker = TypeChecker::new(&symbols);

        let result = checker.check_binary_op("+", &Type::uint256(), &Type::uint8());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::uint256());
    }

    #[test]
    fn test_boolean_operations() {
        let symbols = SymbolTable::new();
        let checker = TypeChecker::new(&symbols);

        let result = checker.check_binary_op("&&", &Type::bool(), &Type::bool());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::bool());
    }

    #[test]
    fn test_comparison_operations() {
        let symbols = SymbolTable::new();
        let checker = TypeChecker::new(&symbols);

        let result = checker.check_binary_op("==", &Type::uint256(), &Type::uint256());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::bool());
    }
}
