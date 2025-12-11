// Solidity-specific semantic analysis
use crate::semantics::SemanticAnalyzer;
use crate::semantics::backend::Backend;
use anyhow::Result;

pub struct SoliditySemantics;

impl SoliditySemantics {
    pub fn analyze(source: &str, backend: Backend) -> Result<()> {
        // Adapt Solidity syntax to our analyzer
        let quorlin_like = Self::translate_to_quorlin_syntax(source);
        
        let mut analyzer = SemanticAnalyzer::for_backend(backend);
        match analyzer.analyze(&quorlin_like) {
            Ok(_) => {
                analyzer.print_warnings();
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Solidity semantic error: {}", e)),
        }
    }
    
    fn translate_to_quorlin_syntax(solidity: &str) -> String {
        let mut output = String::new();
        
        for line in solidity.lines() {
            let line = line.trim();
            
            // Contract definition: contract Name { -> contract Name {
            if line.starts_with("contract ") {
                output.push_str(line);
                output.push('\n');
                continue;
            }
            
            // State variables: uint256 public balance; -> self.balance: uint256;
            if Self::is_state_variable(line) {
                if let Some(translated) = Self::translate_state_var(line) {
                    output.push_str(&translated);
                    output.push('\n');
                    continue;
                }
            }
            
            // Function: function transfer(...) public { -> @external fn transfer(...) {
            if line.starts_with("function ") {
                if let Some(translated) = Self::translate_function(line) {
                    output.push_str(&translated);
                    output.push('\n');
                    continue;
                }
            }
            
            // Pass through everything else
            output.push_str(line);
            output.push('\n');
        }
        
        output
    }
    
    fn is_state_variable(line: &str) -> bool {
        // Simple heuristic: has type and ends with ; but not inside function
        (line.contains("uint") || line.contains("address") || line.contains("bool") || line.contains("mapping"))
            && line.ends_with(';')
            && !line.contains('(')
    }
    
    fn translate_state_var(line: &str) -> Option<String> {
        // uint256 public balance; -> self.balance: uint256;
        // mapping(address => uint256) balances; -> self.balances: mapping[address => uint256];
        
        let line = line.trim_end_matches(';').trim();
        
        if line.starts_with("mapping") {
            // mapping(address => uint256) public balances
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(name) = parts.last() {
                // Extract mapping type
                if let Some(start) = line.find('(') {
                    if let Some(end) = line.find(')') {
                        let inner = &line[start+1..end];
                        let inner = inner.replace("=>", "=>");
                        return Some(format!("    self.{}: mapping[{}];", name, inner));
                    }
                }
            }
        } else {
            // Regular variable: uint256 public balance
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let type_ = parts[0];
                let name = parts.iter().last()?;
                return Some(format!("    self.{}: {};", name, type_));
            }
        }
        
        None
    }
    
    fn translate_function(line: &str) -> Option<String> {
        // function transfer(address to, uint256 amount) public { 
        // -> @external fn transfer(to: address, amount: uint256) {
        
        let line = line.trim();
        let is_public = line.contains("public");
        let is_view = line.contains("view") || line.contains("pure");
        let is_payable = line.contains("payable");
        
        // Extract function name
        let fn_start = line.find("function ")? + 9;
        let paren_start = line.find('(')?;
        let name = line[fn_start..paren_start].trim();
        
        // Extract parameters
        let paren_end = line.find(')')?;
        let params_str = &line[paren_start+1..paren_end];
        let params = Self::translate_params(params_str);
        
        // Extract return type
        let return_type = if line.contains("returns") {
            if let Some(ret_start) = line.find("returns") {
                let ret_start = ret_start + 7;
                if let Some(paren) = line[ret_start..].find('(') {
                    if let Some(paren_end) = line[ret_start..].find(')') {
                        let ret = line[ret_start+paren+1..ret_start+paren_end].trim();
                        format!(" -> {}", ret)
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        // Build output
        let mut decorators = Vec::new();
        if name == "constructor" {
            decorators.push("@constructor");
        } else if is_view {
            decorators.push("@view");
        } else if is_public || is_payable {
            decorators.push("@external");
        }
        if is_payable {
            decorators.push("@payable");
        }
        
        let decorator_str = if decorators.is_empty() {
            String::new()
        } else {
            format!("    {}\n", decorators.join("\n    "))
        };
        
        let fn_name = if name == "constructor" { "__init__" } else { name };
        
        Some(format!("{}    fn {}({}){} {{", decorator_str, fn_name, params, return_type))
    }
    
    fn translate_params(params_str: &str) -> String {
        if params_str.trim().is_empty() {
            return String::new();
        }
        
        params_str.split(',')
            .map(|p| {
                let parts: Vec<&str> = p.trim().split_whitespace().collect();
                if parts.len() == 2 {
                    format!("{}: {}", parts[1], parts[0])
                } else {
                    p.trim().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
