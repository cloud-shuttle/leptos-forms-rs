//! Contract monitoring for detecting API violations

use crate::*;
use std::collections::HashMap;

/// Contract monitoring system
pub struct ContractMonitor {
    baseline_schema: ApiSchema,
    current_schema: ApiSchema,
    violation_threshold: ViolationThreshold,
}

/// Violation threshold configuration
#[derive(Debug, Clone)]
pub struct ViolationThreshold {
    pub max_breaking_changes: usize,
    pub max_signature_changes: usize,
    pub max_removed_methods: usize,
    pub allow_new_features: bool,
    pub allow_deprecations: bool,
}

impl Default for ViolationThreshold {
    fn default() -> Self {
        Self {
            max_breaking_changes: 0,
            max_signature_changes: 0,
            max_removed_methods: 0,
            allow_new_features: true,
            allow_deprecations: true,
        }
    }
}

/// Contract violation types
#[derive(Debug, Clone)]
pub enum ContractViolation {
    BreakingChange(BreakingChange),
    MissingMethod(String),
    SignatureMismatch(String, String, String), // method, expected, actual
    TypeMismatch(String, String, String),      // type, expected, actual
    RemovedField(String, String),              // type, field
    RemovedVariant(String, String),            // type, variant
    StabilityDowngrade(String, ApiStability, ApiStability), // api, from, to
}

/// Contract monitoring results
#[derive(Debug, Clone)]
pub struct MonitoringResults {
    pub violations: Vec<ContractViolation>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub severity: ViolationSeverity,
}

/// Violation severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ViolationSeverity {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl ContractMonitor {
    pub fn new(baseline: ApiSchema, current: ApiSchema) -> Self {
        Self {
            baseline_schema: baseline,
            current_schema: current,
            violation_threshold: ViolationThreshold::default(),
        }
    }

    pub fn with_threshold(mut self, threshold: ViolationThreshold) -> Self {
        self.violation_threshold = threshold;
        self
    }

    pub fn detect_violations(&self) -> MonitoringResults {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();

        // Check for breaking changes in traits
        for (trait_name, baseline_trait) in &self.baseline_schema.traits {
            if let Some(current_trait) = self.current_schema.traits.get(trait_name) {
                self.check_trait_violations(
                    trait_name,
                    current_trait,
                    baseline_trait,
                    &mut violations,
                    &mut warnings,
                );
            } else {
                violations.push(ContractViolation::BreakingChange(BreakingChange {
                    description: format!("Trait {} was removed", trait_name),
                    affected_api: trait_name.clone(),
                    migration_guide: format!("Update code that uses {} trait", trait_name),
                }));
            }
        }

        // Check for breaking changes in types
        for (type_name, baseline_type) in &self.baseline_schema.types {
            if let Some(current_type) = self.current_schema.types.get(type_name) {
                self.check_type_violations(
                    type_name,
                    current_type,
                    baseline_type,
                    &mut violations,
                    &mut warnings,
                );
            } else {
                violations.push(ContractViolation::BreakingChange(BreakingChange {
                    description: format!("Type {} was removed", type_name),
                    affected_api: type_name.clone(),
                    migration_guide: format!("Update code that uses {} type", type_name),
                }));
            }
        }

        // Check for new traits/types (warnings)
        for trait_name in self.current_schema.traits.keys() {
            if !self.baseline_schema.traits.contains_key(trait_name) {
                warnings.push(format!("New trait {} was added", trait_name));
            }
        }

        for type_name in self.current_schema.types.keys() {
            if !self.baseline_schema.types.contains_key(type_name) {
                warnings.push(format!("New type {} was added", type_name));
            }
        }

        // Generate recommendations
        self.generate_recommendations(&violations, &mut recommendations);

        // Determine severity
        let severity = self.determine_severity(&violations);

        MonitoringResults {
            violations,
            warnings,
            recommendations,
            severity,
        }
    }

    fn check_trait_violations(
        &self,
        trait_name: &str,
        current_trait: &TraitSchema,
        baseline_trait: &TraitSchema,
        violations: &mut Vec<ContractViolation>,
        warnings: &mut Vec<String>,
    ) {
        // Check for missing methods
        for method in &baseline_trait.required_methods {
            if !current_trait.required_methods.contains(method) {
                violations.push(ContractViolation::MissingMethod(format!(
                    "{}.{}",
                    trait_name, method
                )));
            }
        }

        // Check for signature changes
        for (method, expected_sig) in &baseline_trait.signatures {
            if let Some(actual_sig) = current_trait.signatures.get(method) {
                if expected_sig != actual_sig {
                    violations.push(ContractViolation::SignatureMismatch(
                        format!("{}.{}", trait_name, method),
                        expected_sig.clone(),
                        actual_sig.clone(),
                    ));
                }
            }
        }

        // Check for stability downgrades
        if self.is_stability_downgrade(&baseline_trait.stability, &current_trait.stability) {
            violations.push(ContractViolation::StabilityDowngrade(
                trait_name.to_string(),
                baseline_trait.stability.clone(),
                current_trait.stability.clone(),
            ));
        }
    }

    fn check_type_violations(
        &self,
        type_name: &str,
        current_type: &TypeSchema,
        baseline_type: &TypeSchema,
        violations: &mut Vec<ContractViolation>,
        warnings: &mut Vec<String>,
    ) {
        // Check for removed fields
        for field in &baseline_type.fields {
            if !current_type.fields.contains(field) {
                violations.push(ContractViolation::RemovedField(
                    type_name.to_string(),
                    field.clone(),
                ));
            }
        }

        // Check for removed methods
        for method in &baseline_type.methods {
            if !current_type.methods.contains(method) {
                violations.push(ContractViolation::MissingMethod(format!(
                    "{}.{}",
                    type_name, method
                )));
            }
        }

        // Check for stability downgrades
        if self.is_stability_downgrade(&baseline_type.stability, &current_type.stability) {
            violations.push(ContractViolation::StabilityDowngrade(
                type_name.to_string(),
                baseline_type.stability.clone(),
                current_type.stability.clone(),
            ));
        }
    }

    fn is_stability_downgrade(&self, from: &ApiStability, to: &ApiStability) -> bool {
        match (from, to) {
            (ApiStability::Stable, ApiStability::Experimental) => true,
            (ApiStability::Stable, ApiStability::Deprecated) => true,
            (ApiStability::Experimental, ApiStability::Deprecated) => true,
            _ => false,
        }
    }

    fn generate_recommendations(
        &self,
        violations: &[ContractViolation],
        recommendations: &mut Vec<String>,
    ) {
        let breaking_count = violations
            .iter()
            .filter(|v| matches!(v, ContractViolation::BreakingChange(_)))
            .count();
        let missing_methods = violations
            .iter()
            .filter(|v| matches!(v, ContractViolation::MissingMethod(_)))
            .count();
        let signature_changes = violations
            .iter()
            .filter(|v| matches!(v, ContractViolation::SignatureMismatch(_, _, _)))
            .count();

        if breaking_count > self.violation_threshold.max_breaking_changes {
            recommendations.push("Consider incrementing major version number".to_string());
        }

        if missing_methods > self.violation_threshold.max_removed_methods {
            recommendations.push("Consider deprecating methods before removing them".to_string());
        }

        if signature_changes > self.violation_threshold.max_signature_changes {
            recommendations
                .push("Consider using method overloading instead of signature changes".to_string());
        }

        if !violations.is_empty() {
            recommendations.push("Update migration documentation".to_string());
            recommendations.push("Notify consumers of breaking changes".to_string());
        }
    }

    fn determine_severity(&self, violations: &[ContractViolation]) -> ViolationSeverity {
        if violations.is_empty() {
            return ViolationSeverity::None;
        }

        let breaking_count = violations
            .iter()
            .filter(|v| matches!(v, ContractViolation::BreakingChange(_)))
            .count();
        let missing_methods = violations
            .iter()
            .filter(|v| matches!(v, ContractViolation::MissingMethod(_)))
            .count();
        let signature_changes = violations
            .iter()
            .filter(|v| matches!(v, ContractViolation::SignatureMismatch(_, _, _)))
            .count();

        if breaking_count > 5 || missing_methods > 3 || signature_changes > 5 {
            ViolationSeverity::Critical
        } else if breaking_count > 2 || missing_methods > 1 || signature_changes > 2 {
            ViolationSeverity::High
        } else if breaking_count > 0 || missing_methods > 0 || signature_changes > 0 {
            ViolationSeverity::Medium
        } else {
            ViolationSeverity::Low
        }
    }

    pub fn should_fail_build(&self) -> bool {
        let results = self.detect_violations();
        matches!(
            results.severity,
            ViolationSeverity::High | ViolationSeverity::Critical
        )
    }

    pub fn should_warn(&self) -> bool {
        let results = self.detect_violations();
        !matches!(results.severity, ViolationSeverity::None)
    }

    pub fn generate_report(&self) -> MonitoringReport {
        let results = self.detect_violations();

        MonitoringReport {
            baseline_version: self.baseline_schema.version.clone(),
            current_version: self.current_schema.version.clone(),
            results,
            timestamp: chrono::Utc::now(),
            threshold: self.violation_threshold.clone(),
        }
    }
}

/// Comprehensive monitoring report
#[derive(Debug, Clone, Serialize)]
pub struct MonitoringReport {
    pub baseline_version: String,
    pub current_version: String,
    pub results: MonitoringResults,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub threshold: ViolationThreshold,
}

impl MonitoringReport {
    pub fn is_compliant(&self) -> bool {
        matches!(
            self.results.severity,
            ViolationSeverity::None | ViolationSeverity::Low
        )
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Contract Monitoring Report\n\n");
        markdown.push_str(&format!(
            "**Baseline Version**: {}\n",
            self.baseline_version
        ));
        markdown.push_str(&format!("**Current Version**: {}\n", self.current_version));
        markdown.push_str(&format!(
            "**Timestamp**: {}\n",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        markdown.push_str(&format!("**Severity**: {:?}\n", self.results.severity));
        markdown.push_str(&format!("**Compliant**: {}\n\n", self.is_compliant()));

        if !self.results.violations.is_empty() {
            markdown.push_str("## Violations\n\n");
            for violation in &self.results.violations {
                match violation {
                    ContractViolation::BreakingChange(change) => {
                        markdown
                            .push_str(&format!("- **Breaking Change**: {}\n", change.description));
                        markdown
                            .push_str(&format!("  - **Affected API**: {}\n", change.affected_api));
                        markdown.push_str(&format!(
                            "  - **Migration Guide**: {}\n",
                            change.migration_guide
                        ));
                    }
                    ContractViolation::MissingMethod(method) => {
                        markdown.push_str(&format!("- **Missing Method**: {}\n", method));
                    }
                    ContractViolation::SignatureMismatch(method, expected, actual) => {
                        markdown.push_str(&format!("- **Signature Mismatch**: {}\n", method));
                        markdown.push_str(&format!("  - **Expected**: {}\n", expected));
                        markdown.push_str(&format!("  - **Actual**: {}\n", actual));
                    }
                    ContractViolation::TypeMismatch(type_name, expected, actual) => {
                        markdown.push_str(&format!("- **Type Mismatch**: {}\n", type_name));
                        markdown.push_str(&format!("  - **Expected**: {}\n", expected));
                        markdown.push_str(&format!("  - **Actual**: {}\n", actual));
                    }
                    ContractViolation::RemovedField(type_name, field) => {
                        markdown
                            .push_str(&format!("- **Removed Field**: {}.{}\n", type_name, field));
                    }
                    ContractViolation::RemovedVariant(type_name, variant) => {
                        markdown.push_str(&format!(
                            "- **Removed Variant**: {}.{}\n",
                            type_name, variant
                        ));
                    }
                    ContractViolation::StabilityDowngrade(api, from, to) => {
                        markdown.push_str(&format!("- **Stability Downgrade**: {}\n", api));
                        markdown.push_str(&format!("  - **From**: {:?}\n", from));
                        markdown.push_str(&format!("  - **To**: {:?}\n", to));
                    }
                }
                markdown.push_str("\n");
            }
        }

        if !self.results.warnings.is_empty() {
            markdown.push_str("## Warnings\n\n");
            for warning in &self.results.warnings {
                markdown.push_str(&format!("- {}\n", warning));
            }
            markdown.push_str("\n");
        }

        if !self.results.recommendations.is_empty() {
            markdown.push_str("## Recommendations\n\n");
            for recommendation in &self.results.recommendations {
                markdown.push_str(&format!("- {}\n", recommendation));
            }
            markdown.push_str("\n");
        }

        markdown
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_monitor_creation() {
        let baseline = generate_current_schema();
        let current = generate_current_schema();

        let monitor = ContractMonitor::new(baseline, current);
        let violations = monitor.detect_violations();

        assert!(violations.violations.is_empty());
        assert_eq!(violations.severity, ViolationSeverity::None);
    }

    #[test]
    fn test_contract_monitor_breaking_changes() {
        let baseline = generate_current_schema();
        let mut current = generate_current_schema();

        // Simulate a breaking change by removing a required method
        if let Some(form_trait) = current.traits.get_mut("Form") {
            form_trait.required_methods.retain(|m| m != "validate");
        }

        let monitor = ContractMonitor::new(baseline, current);
        let violations = monitor.detect_violations();

        assert!(!violations.violations.is_empty());
        assert!(violations.severity > ViolationSeverity::None);
    }

    #[test]
    fn test_violation_threshold() {
        let threshold = ViolationThreshold {
            max_breaking_changes: 1,
            max_signature_changes: 2,
            max_removed_methods: 1,
            allow_new_features: true,
            allow_deprecations: true,
        };

        assert_eq!(threshold.max_breaking_changes, 1);
        assert_eq!(threshold.max_signature_changes, 2);
        assert_eq!(threshold.max_removed_methods, 1);
        assert!(threshold.allow_new_features);
        assert!(threshold.allow_deprecations);
    }

    #[test]
    fn test_monitoring_report() {
        let baseline = generate_current_schema();
        let current = generate_current_schema();

        let monitor = ContractMonitor::new(baseline, current);
        let report = monitor.generate_report();

        assert!(report.is_compliant());
        assert_eq!(report.baseline_version, "1.1.0");
        assert_eq!(report.current_version, "1.1.0");

        let markdown = report.to_markdown();
        assert!(markdown.contains("# Contract Monitoring Report"));
        assert!(markdown.contains("**Baseline Version**: 1.1.0"));
        assert!(markdown.contains("**Current Version**: 1.1.0"));
    }
}
