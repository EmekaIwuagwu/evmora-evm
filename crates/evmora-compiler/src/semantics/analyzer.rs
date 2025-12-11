use super::symbol_table::{SymbolTable, FunctionSignature};
use super::type_checker::TypeChecker;
use super::security_analyzer::SecurityAnalyzer;
use super::validator::Validator;
use super::types::{Type, SemanticResult};
use super::ast::{Contract, SimpleParser};
use super::backend::{Backend, BackendValidator};
use std::collections::{HashMap, HashSet};

pub struct SemanticAnalyzer {
    symbols: SymbolTable,
    type_env: HashMap<String, Type>,
    current_function: Option<String>,
    initialized_vars: HashSet<String>,
    function_return_types: HashMap<String, Option<Type>>,
    security: SecurityAnalyzer,
    validator: Validator,
    backend: Backend,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self::for_backend(Backend::EVM)
    }

    pub fn for_backend(backend: Backend) -> Self {
        Self {
            symbols: SymbolTable::new(),
            type_env: HashMap::new(),
            current_function: None,
            initialized_vars: HashSet::new(),
            function_return_types: HashMap::new(),
            security: SecurityAnalyzer::new(),
            validator: Validator::new(),
            backend,
        }
    }

    /// Three-pass analysis:
    /// 1. Collect definitions
    /// 2. Type check and validate
    /// 3. Security analysis
    pub fn analyze(&mut self, source: &str) -> SemanticResult<AnalysisResult> {
        // Parse AST
        let contract = SimpleParser::parse_contract(source);
        
        // Backend-specific validation
        let backend_validator = BackendValidator::new(self.backend);
        backend_validator.validate_contract(&contract)?;
        
        // Pass 1: Collect definitions
        self.collect_definitions_from_ast(&contract)?;

        // Pass 2: Type check
        self.check_types_from_ast(&contract)?;

        // Pass 3: Security analysis
        self.analyze_security(source);

        Ok(AnalysisResult {
            type_env: self.type_env.clone(),
            warnings: self.security.get_warnings().to_vec(),
        })
    }

    fn collect_definitions_from_ast(&mut self, contract: &Contract) -> SemanticResult<()> {
        // Collect state variables
        for state_var in &contract.state_vars {
            self.symbols.define_state_variable(&state_var.name, state_var.type_.clone())?;
            self.type_env.insert(state_var.name.clone(), state_var.type_.clone());
        }

        // Collect functions
        for func in &contract.functions {
            let sig = FunctionSignature {
                name: func.name.clone(),
                params: func.params.clone(),
                return_type: func.return_type.clone(),
                is_view: func.decorators.contains(&"view".to_string()),
                is_external: func.decorators.contains(&"external".to_string()),
                is_payable: func.decorators.contains(&"payable".to_string()),
            };
            
            self.function_return_types.insert(func.name.clone(), func.return_type.clone());
            self.symbols.define_function(sig)?;
        }

        Ok(())
    }

    fn check_types_from_ast(&mut self, contract: &Contract) -> SemanticResult<()> {
        // Validate decorators and function signatures
        for func in &contract.functions {
            self.validator.validate_decorators(&func.name, &func.decorators)?;
        }
        Ok(())
    }

    fn analyze_security(&mut self, source: &str) {
        // Simple pattern-based security checks
        let has_external_call = source.contains(".call") || source.contains(".transfer");
        let has_state_change = source.contains("self.") && (source.contains("=") || source.contains("+="));
        
        // Check for state modification after external calls (simplified)
        if has_external_call && has_state_change {
            let lines: Vec<&str> = source.lines().collect();
            let mut call_line = 0;
            let mut state_mod_line = 0;

            for (i, line) in lines.iter().enumerate() {
                if line.contains(".call") || line.contains(".transfer") {
                    call_line = i;
                }
                if line.contains("self.") && (line.contains("=") || line.contains("+=")) {
                    state_mod_line = i;
                }
            }

            if state_mod_line > call_line && call_line > 0 {
                self.security.analyze_function(
                    "detected_function",
                    false,
                    false,
                    true,
                    true,
                    true,
                );
            }
        }

        // Check for unchecked arithmetic
        for line in source.lines() {
            if (line.contains('+') || line.contains('-') || line.contains('*') || line.contains('/'))
                && !line.contains("safe_") {
                if line.contains("self.") {
                    self.security.check_unchecked_arithmetic(line.trim(), line.trim());
                }
            }
        }
    }

    fn parse_type(&self, type_str: &str) -> Type {
        let type_str = type_str.trim();
        
        if type_str.starts_with("mapping[") {
            // mapping[K => V]
            if let Some(end) = type_str.rfind(']') {
                let inner = &type_str[8..end];
                if let Some(arrow) = inner.find("=>") {
                    let key = self.parse_type(&inner[..arrow].trim());
                    let value = self.parse_type(&inner[arrow+2..].trim());
                    return Type::Mapping(Box::new(key), Box::new(value));
                }
            }
        } else if type_str.starts_with("list[") {
            if let Some(end) = type_str.rfind(']') {
                let inner = &type_str[5..end];
                let elem_type = self.parse_type(inner);
                return Type::List(Box::new(elem_type));
            }
        } else if type_str.starts_with('(') && type_str.ends_with(')') {
            // Tuple
            let inner = &type_str[1..type_str.len()-1];
            let types: Vec<Type> = inner.split(',').map(|s| self.parse_type(s.trim())).collect();
            return Type::Tuple(types);
        } else if type_str.ends_with('?') {
            let inner = &type_str[..type_str.len()-1];
            return Type::Optional(Box::new(self.parse_type(inner)));
        }

        Type::Simple(type_str.to_string())
    }

    fn parse_function_signature(&self, line: &str) -> Option<FunctionSignature> {
        // Very simplified parser
        if !line.contains("fn ") {
            return None;
        }

        let fn_start = line.find("fn ")?;
        let paren_start = line.find('(')?;
        let name = line[fn_start+3..paren_start].trim().to_string();

        Some(FunctionSignature {
            name,
            params: Vec::new(),
            return_type: Some(Type::uint256()), // Simplified
            is_view: line.contains("@view"),
            is_external: line.contains("@external"),
            is_payable: line.contains("@payable"),
        })
    }

    pub fn get_security_warnings(&self) -> &[super::security_analyzer::SecurityWarning] {
        self.security.get_warnings()
    }

    pub fn print_warnings(&self) {
        self.security.print_warnings();
    }
}

pub struct AnalysisResult {
    pub type_env: HashMap<String, Type>,
    pub warnings: Vec<super::security_analyzer::SecurityWarning>,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_analysis() {
        let mut analyzer = SemanticAnalyzer::new();
        let source = r#"
            fn transfer(to: address, amount: uint256) {
                self.balance = self.balance - amount;
            }
        "#;

        let result = analyzer.analyze(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reentrancy_detection() {
        let mut analyzer = SemanticAnalyzer::new();
        let source = r#"
            fn withdraw() {
                to.call();
                self.balance = 0;
            }
        "#;

        let _ = analyzer.analyze(source);
        let warnings = analyzer.get_security_warnings();
        assert!(!warnings.is_empty());
    }
}
