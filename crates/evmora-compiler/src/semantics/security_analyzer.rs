use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct SecurityWarning {
    pub warning_type: String,
    pub severity: Severity,
    pub location: String,
    pub message: String,
    pub remediation: String,
}

pub struct SecurityAnalyzer {
    warnings: Vec<SecurityWarning>,
    external_calls_found: HashSet<String>,
    state_modifications_found: HashSet<String>,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
            external_calls_found: HashSet::new(),
            state_modifications_found: HashSet::new(),
        }
    }

    pub fn analyze_function(
        &mut self,
        func_name: &str,
        is_view: bool,
        has_auth_check: bool,
        modifies_state: bool,
        has_external_call: bool,
        state_modified_after_call: bool,
    ) {
        // Reentrancy check
        if has_external_call && state_modified_after_call {
            self.warnings.push(SecurityWarning {
                warning_type: "REENTRANCY".to_string(),
                severity: Severity::Critical,
                location: func_name.to_string(),
                message: "State modification after external call - potential reentrancy vulnerability".to_string(),
                remediation: "Follow Checks-Effects-Interactions pattern: update state before external calls".to_string(),
            });
        }

        // Access control check
        if modifies_state && !has_auth_check && func_name != "__init__" {
            self.warnings.push(SecurityWarning {
                warning_type: "ACCESS_CONTROL".to_string(),
                severity: Severity::High,
                location: func_name.to_string(),
                message: "State-modifying function lacks access control check".to_string(),
                remediation: "Add require(msg.sender == owner) or similar authorization".to_string(),
            });
        }

        // View function check
        if is_view && modifies_state {
            self.warnings.push(SecurityWarning {
                warning_type: "VIEW_VIOLATION".to_string(),
                severity: Severity::High,
                location: func_name.to_string(),
                message: "View function modifies state".to_string(),
                remediation: "Remove @view decorator or remove state modifications".to_string(),
            });
        }
    }

    pub fn check_unchecked_arithmetic(&mut self, location: &str, operation: &str) {
        self.warnings.push(SecurityWarning {
            warning_type: "INTEGER_OVERFLOW".to_string(),
            severity: Severity::Medium,
            location: location.to_string(),
            message: format!("Unchecked arithmetic operation: {}", operation),
            remediation: "Use safe_add, safe_sub, safe_mul, safe_div for checked arithmetic".to_string(),
        });
    }

    pub fn check_unbounded_loop(&mut self, location: &str) {
        self.warnings.push(SecurityWarning {
            warning_type: "GAS_OPTIMIZATION".to_string(),
            severity: Severity::Medium,
            location: location.to_string(),
            message: "Potentially unbounded loop detected".to_string(),
            remediation: "Add upper bound to loop iterations to prevent gas exhaustion".to_string(),
        });
    }

    pub fn check_tx_origin_usage(&mut self, location: &str) {
        self.warnings.push(SecurityWarning {
            warning_type: "TX_ORIGIN".to_string(),
            severity: Severity::High,
            location: location.to_string(),
            message: "Use of tx.origin for authorization".to_string(),
            remediation: "Use msg.sender instead of tx.origin to prevent phishing attacks".to_string(),
        });
    }

    pub fn get_warnings(&self) -> &[SecurityWarning] {
        &self.warnings
    }

    pub fn print_warnings(&self) {
        for warning in &self.warnings {
            let severity_str = match warning.severity {
                Severity::Critical => "🔴 CRITICAL",
                Severity::High => "🟠 HIGH",
                Severity::Medium => "🟡 MEDIUM",
                Severity::Low => "🟢 LOW",
            };

            eprintln!("\n{} [{}] {}", severity_str, warning.warning_type, warning.location);
            eprintln!("  {}", warning.message);
            eprintln!("  💡 {}", warning.remediation);
        }
    }

    pub fn has_critical_warnings(&self) -> bool {
        self.warnings.iter().any(|w| w.severity == Severity::Critical)
    }
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
