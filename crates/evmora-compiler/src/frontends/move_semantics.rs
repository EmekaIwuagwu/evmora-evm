// Move-specific semantic analysis
use crate::semantics::SemanticAnalyzer;
use crate::semantics::backend::Backend;
use anyhow::Result;

pub struct MoveSemantics;

impl MoveSemantics {
    pub fn analyze(source: &str, backend: Backend) -> Result<()> {
        let quorlin_like = Self::translate_to_quorlin_syntax(source);
        
        let mut analyzer = SemanticAnalyzer::for_backend(backend);
        match analyzer.analyze(&quorlin_like) {
            Ok(_) => {
                analyzer.print_warnings();
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Move semantic error: {}", e)),
        }
    }
    
    fn translate_to_quorlin_syntax(move_source: &str) -> String {
        let mut output = String::new();
        
        for line in move_source.lines() {
            let line = line.trim();
            
            // module -> contract
            if line.starts_with("module ") {
                let name = line.strip_prefix("module ")
                    .and_then(|s| s.split_whitespace().next())
                    .unwrap_or("Unknown");
                output.push_str(&format!("contract {} {{\n", name));
                continue;
            }
            
            // struct resources -> state variables
            if line.starts_with("struct ") && line.contains("has key") {
                output.push_str("    // Resource state\n");
                continue;
            }
            
            // public fun -> @external fn
            if line.starts_with("public fun ") || line.starts_with("public entry fun ") {
                if let Some(translated) = Self::translate_function(line) {
                    output.push_str(&translated);
                    output.push('\n');
                    continue;
                }
            }
            
            output.push_str(line);
            output.push('\n');
        }
        
        output
    }
    
    fn translate_function(line: &str) -> Option<String> {
        // public fun transfer(from: &signer, to: address, amount: u64)
        // -> @external fn transfer(from: address, to: address, amount: uint256)
        
        let is_entry = line.contains("entry");
        let fn_start = if line.contains("entry fun") {
            line.find("entry fun")? + 10
        } else {
            line.find("public fun")? + 11
        };
        
        let paren_start = line.find('(')?;
        let name = line[fn_start..paren_start].trim();
        
        let paren_end = line.find(')')?;
        let params_str = &line[paren_start+1..paren_end];
        let params = Self::translate_params(params_str);
        
        let decorator = if is_entry { "@external" } else { "@view" };
        
        Some(format!("    {}\n    fn {}({}) {{", decorator, name, params))
    }
    
    fn translate_params(params_str: &str) -> String {
        if params_str.trim().is_empty() {
            return String::new();
        }
        
        params_str.split(',')
            .map(|p| {
                let p = p.trim();
                // Remove &signer, &mut, etc.
                let p = p.replace("&signer", "address")
                         .replace("&mut ", "")
                         .replace("&", "");
                
                let parts: Vec<&str> = p.split(':').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim();
                    let type_ = parts[1].trim()
                        .replace("u64", "uint256")
                        .replace("u128", "uint256")
                        .replace("u256", "uint256");
                    format!("{}: {}", name, type_)
                } else {
                    p
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
