// Simplified AST for semantic analysis
use super::types::Type;

#[derive(Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub state_vars: Vec<StateVariable>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct StateVariable {
    pub name: String,
    pub type_: Type,
    pub initial_value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub decorators: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment { target: String, value: Expression },
    FunctionCall { name: String, args: Vec<Expression> },
    Return(Option<Expression>),
    Require(Expression),
    If { condition: Expression, then_block: Vec<Statement>, else_block: Vec<Statement> },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(String),
    Variable(String),
    BinaryOp { op: String, left: Box<Expression>, right: Box<Expression> },
    UnaryOp { op: String, operand: Box<Expression> },
    FunctionCall { name: String, args: Vec<Expression> },
    Index { object: Box<Expression>, index: Box<Expression> },
    Attribute { object: Box<Expression>, attr: String },
}

pub struct SimpleParser;

impl SimpleParser {
    pub fn parse_contract(source: &str) -> Contract {
        let mut state_vars = Vec::new();
        let mut functions = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        
        let contract_name = lines.iter()
            .find(|l| l.trim().starts_with("contract "))
            .and_then(|l| l.trim().strip_prefix("contract "))
            .and_then(|l| l.split_whitespace().next())
            .unwrap_or("Unknown")
            .to_string();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Parse state variables: self.var: Type
            if line.starts_with("self.") && line.contains(':') && !line.contains('(') {
                if let Some(var) = Self::parse_state_var(line) {
                    state_vars.push(var);
                }
            }
            
            // Parse functions
            if line.contains("fn ") {
                let func = Self::parse_function(&lines, &mut i);
                functions.push(func);
            }
            
            i += 1;
        }

        Contract { name: contract_name, state_vars, functions }
    }

    fn parse_state_var(line: &str) -> Option<StateVariable> {
        // self.name: Type = value
        let without_self = line.strip_prefix("self.")?;
        let (name_type, value) = if let Some(eq_pos) = without_self.find('=') {
            (&without_self[..eq_pos], Some(&without_self[eq_pos+1..]))
        } else {
            (without_self, None)
        };
        
        let parts: Vec<&str> = name_type.split(':').collect();
        if parts.len() != 2 { return None; }
        
        let name = parts[0].trim().to_string();
        let type_str = parts[1].trim().trim_end_matches(';');
        let type_ = Self::parse_type(type_str);
        
        Some(StateVariable {
            name,
            type_,
            initial_value: value.map(|v| Expression::Literal(v.trim().trim_end_matches(';').to_string())),
        })
    }

    fn parse_function(lines: &[&str], index: &mut usize) -> Function {
        let line = lines[*index].trim();
        
        // Collect decorators
        let mut decorators = Vec::new();
        let mut func_line = *index;
        
        while func_line > 0 {
            let prev = lines[func_line - 1].trim();
            if prev.starts_with('@') {
                decorators.insert(0, prev.trim_start_matches('@').to_string());
                func_line -= 1;
            } else {
                break;
            }
        }
        
        // Parse function signature
        let fn_start = line.find("fn ").unwrap_or(0) + 3;
        let paren_start = line.find('(').unwrap_or(line.len());
        let name = line[fn_start..paren_start].trim().to_string();
        
        let paren_end = line.find(')').unwrap_or(line.len());
        let params_str = &line[paren_start+1..paren_end];
        let params = Self::parse_params(params_str);
        
        let return_type = if line.contains("->") {
            let arrow_pos = line.find("->").unwrap() + 2;
            let ret_type_str = line[arrow_pos..].trim().trim_start_matches('{').trim();
            Some(Self::parse_type(ret_type_str))
        } else {
            None
        };
        
        // Parse function body (simplified)
        let body = Vec::new();
        
        Function { name, params, return_type, decorators, body }
    }

    fn parse_params(params_str: &str) -> Vec<(String, Type)> {
        if params_str.trim().is_empty() {
            return Vec::new();
        }
        
        params_str.split(',')
            .filter_map(|p| {
                let parts: Vec<&str> = p.split(':').collect();
                if parts.len() == 2 {
                    Some((parts[0].trim().to_string(), Self::parse_type(parts[1].trim())))
                } else {
                    None
                }
            })
            .collect()
    }

    fn parse_type(type_str: &str) -> Type {
        let type_str = type_str.trim();
        
        if type_str.starts_with("mapping[") {
            if let Some(end) = type_str.rfind(']') {
                let inner = &type_str[8..end];
                if let Some(arrow) = inner.find("=>") {
                    let key = Self::parse_type(inner[..arrow].trim());
                    let value = Self::parse_type(inner[arrow+2..].trim());
                    return Type::Mapping(Box::new(key), Box::new(value));
                }
            }
        } else if type_str.starts_with("list[") {
            if let Some(end) = type_str.rfind(']') {
                let inner = &type_str[5..end];
                return Type::List(Box::new(Self::parse_type(inner)));
            }
        }
        
        Type::Simple(type_str.to_string())
    }
}
