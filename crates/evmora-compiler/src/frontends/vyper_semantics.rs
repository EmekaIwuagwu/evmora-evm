// Vyper-specific semantic analysis
use crate::semantics::SemanticAnalyzer;
use crate::semantics::backend::Backend;
use anyhow::Result;

pub struct VyperSemantics;

impl VyperSemantics {
    pub fn analyze(source: &str, backend: Backend) -> Result<()> {
        let quorlin_like = Self::translate_to_quorlin_syntax(source);
        
        let mut analyzer = SemanticAnalyzer::for_backend(backend);
        match analyzer.analyze(&quorlin_like) {
            Ok(_) => {
                analyzer.print_warnings();
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Vyper semantic error: {}", e)),
        }
    }
    
    fn translate_to_quorlin_syntax(vyper: &str) -> String {
        let mut output = String::new();
        output.push_str("contract VyperContract {\n");
        
        for line in vyper.lines() {
            let line = line.trim();
            
            // State variables: balance: public(uint256) -> self.balance: uint256;
            if line.contains(':') && (line.contains("public") || line.contains("HashMap")) {
                if let Some(translated) = Self::translate_state_var(line) {
                    output.push_str(&translated);
                    output.push('\n');
                    continue;
                }
            }
            
            // Functions: @external -> @external
            if line.starts_with('@') {
                output.push_str("    ");
                output.push_str(line);
                output.push('\n');
                continue;
            }
            
            if line.starts_with("def ") {
                if let Some(translated) = Self::translate_function(line) {
                    output.push_str(&translated);
                    output.push('\n');
                    continue;
                }
            }
            
            output.push_str("    ");
            output.push_str(line);
            output.push('\n');
        }
        
        output.push_str("}\n");
        output
    }
    
    fn translate_state_var(line: &str) -> Option<String> {
        // balance: public(uint256) -> self.balance: uint256;
        // balances: HashMap[address, uint256] -> self.balances: mapping[address => uint256];
        
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            return None;
        }
        
        let name = parts[0].trim();
        let type_part = parts[1].trim();
        
        if type_part.contains("HashMap") {
            // balances: HashMap[address, uint256]
            if let Some(start) = type_part.find('[') {
                if let Some(end) = type_part.find(']') {
                    let inner = &type_part[start+1..end];
                    let inner = inner.replace(',', " =>");
                    return Some(format!("    self.{}: mapping[{}];", name, inner));
                }
            }
        } else if type_part.contains("public") {
            // balance: public(uint256)
            if let Some(start) = type_part.find('(') {
                if let Some(end) = type_part.find(')') {
                    let inner = &type_part[start+1..end];
                    return Some(format!("    self.{}: {};", name, inner));
                }
            }
        } else {
            // Simple type
            return Some(format!("    self.{}: {};", name, type_part));
        }
        
        None
    }
    
    fn translate_function(line: &str) -> Option<String> {
        // def transfer(to: address, amount: uint256): -> fn transfer(to: address, amount: uint256) {
        
        let line = line.trim();
        let def_start = line.find("def ")? + 4;
        let paren_start = line.find('(')?;
        let name = line[def_start..paren_start].trim();
        
        let paren_end = line.find(')')?;
        let params = &line[paren_start+1..paren_end];
        
        // Check for return type -> RetType:
        let return_type = if line.contains("->") {
            let arrow = line.find("->")? + 2;
            let colon = line[arrow..].find(':').map(|i| arrow + i).or(Some(line.len()))?;
            let ret = line[arrow..colon].trim();
            format!(" -> {}", ret)
        } else {
            String::new()
        };
        
        let fn_name = if name == "__init__" { "__init__" } else { name };
        
        Some(format!("    fn {}({}){} {{", fn_name, params, return_type))
    }
}
