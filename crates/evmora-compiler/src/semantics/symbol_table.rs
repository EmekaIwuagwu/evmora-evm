use super::types::{Type, SemanticError, SemanticResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub is_view: bool,
    pub is_external: bool,
    pub is_payable: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable(Type),
    Function(FunctionSignature),
    StateVariable(Type),
    Parameter(Type),
}

#[derive(Debug)]
struct Scope {
    symbols: HashMap<String, SymbolKind>,
}

impl Scope {
    fn new() -> Self {
        Self { symbols: HashMap::new() }
    }
}

pub struct SymbolTable {
    scopes: Vec<Scope>,
    global_scope: Scope,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
            global_scope: Scope::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define_variable(&mut self, name: &str, type_: Type) -> SemanticResult<()> {
        let current = self.scopes.last_mut().unwrap();
        if current.symbols.contains_key(name) {
            return Err(SemanticError::DuplicateDefinition(name.to_string()));
        }
        current.symbols.insert(name.to_string(), SymbolKind::Variable(type_));
        Ok(())
    }

    pub fn define_state_variable(&mut self, name: &str, type_: Type) -> SemanticResult<()> {
        if self.global_scope.symbols.contains_key(name) {
            return Err(SemanticError::DuplicateDefinition(name.to_string()));
        }
        self.global_scope.symbols.insert(name.to_string(), SymbolKind::StateVariable(type_));
        Ok(())
    }

    pub fn define_function(&mut self, sig: FunctionSignature) -> SemanticResult<()> {
        let name = sig.name.clone();
        if self.global_scope.symbols.contains_key(&name) {
            return Err(SemanticError::DuplicateDefinition(name));
        }
        self.global_scope.symbols.insert(name, SymbolKind::Function(sig));
        Ok(())
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Type> {
        // Search from innermost scope outward
        for scope in self.scopes.iter().rev() {
            if let Some(SymbolKind::Variable(t)) | Some(SymbolKind::Parameter(t)) = scope.symbols.get(name) {
                return Some(t);
            }
        }
        // Check global scope for state variables
        if let Some(SymbolKind::StateVariable(t)) = self.global_scope.symbols.get(name) {
            return Some(t);
        }
        None
    }

    pub fn lookup_function(&self, name: &str) -> Option<&FunctionSignature> {
        if let Some(SymbolKind::Function(sig)) = self.global_scope.symbols.get(name) {
            return Some(sig);
        }
        None
    }

    pub fn define_parameter(&mut self, name: &str, type_: Type) -> SemanticResult<()> {
        let current = self.scopes.last_mut().unwrap();
        if current.symbols.contains_key(name) {
            return Err(SemanticError::DuplicateDefinition(name.to_string()));
        }
        current.symbols.insert(name.to_string(), SymbolKind::Parameter(type_));
        Ok(())
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
