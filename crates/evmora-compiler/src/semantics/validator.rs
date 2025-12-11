use super::types::{SemanticError, SemanticResult};
use std::collections::HashSet;

pub struct Validator {
    errors: Vec<SemanticError>,
}

impl Validator {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn validate_decorators(
        &mut self,
        func_name: &str,
        decorators: &[String],
    ) -> SemanticResult<ValidatedDecorators> {
        let mut is_constructor = false;
        let mut is_external = false;
        let mut is_view = false;
        let mut is_payable = false;
        let mut is_internal = false;

        let valid_decorators: HashSet<&str> = 
            ["constructor", "external", "view", "payable", "internal"].iter().copied().collect();

        for dec in decorators {
            if !valid_decorators.contains(dec.as_str()) {
                return Err(SemanticError::InvalidDecorator {
                    decorator: dec.clone(),
                    target: func_name.to_string(),
                });
            }

            match dec.as_str() {
                "constructor" => is_constructor = true,
                "external" => is_external = true,
                "view" => is_view = true,
                "payable" => is_payable = true,
                "internal" => is_internal = true,
                _ => {}
            }
        }

        // Validation rules
        if func_name == "__init__" && !is_constructor {
            return Err(SemanticError::MissingDecorator {
                decorator: "constructor".to_string(),
                target: func_name.to_string(),
            });
        }

        if is_constructor && func_name != "__init__" {
            return Err(SemanticError::InvalidDecorator {
                decorator: "constructor".to_string(),
                target: format!("function {} (only __init__ can be constructor)", func_name),
            });
        }

        if is_view && is_external && !decorators.contains(&"view".to_string()) {
            return Err(SemanticError::InvalidDecorator {
                decorator: "external+view".to_string(),
                target: "Use either @view or @external, not both".to_string(),
            });
        }

        if is_payable && !is_external {
            return Err(SemanticError::InvalidDecorator {
                decorator: "payable".to_string(),
                target: "@payable only valid with @external".to_string(),
            });
        }

        Ok(ValidatedDecorators {
            is_constructor,
            is_external,
            is_view,
            is_payable,
            is_internal,
        })
    }

    pub fn validate_mapping_key(&self, key_type: &str) -> SemanticResult<()> {
        let valid_keys = ["address", "uint256", "uint8", "bytes32", "bool"];
        if !valid_keys.contains(&key_type) {
            return Err(SemanticError::TypeMismatch {
                expected: "primitive type (address, uint256, bytes32, bool)".to_string(),
                found: key_type.to_string(),
            });
        }
        Ok(())
    }

    pub fn validate_return_in_view(&self, is_view: bool, has_return: bool) -> SemanticResult<()> {
        if is_view && !has_return {
            return Err(SemanticError::ViewFunctionModifiesState(
                "View functions must return a value".to_string()
            ));
        }
        Ok(())
    }

    pub fn get_errors(&self) -> &[SemanticError] {
        &self.errors
    }
}

#[derive(Debug)]
pub struct ValidatedDecorators {
    pub is_constructor: bool,
    pub is_external: bool,
    pub is_view: bool,
    pub is_payable: bool,
    pub is_internal: bool,
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
