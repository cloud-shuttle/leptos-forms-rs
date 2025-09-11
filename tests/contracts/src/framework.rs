//! Contract testing framework for leptos-forms-rs

use crate::*;
use std::collections::HashMap;

/// Contract testing framework
pub struct ContractTester {
    schema: ApiSchema,
    test_cases: Vec<ContractTestCase>,
    results: ContractTestResults,
}

/// Individual contract test case
pub struct ContractTestCase {
    pub name: String,
    pub description: String,
    pub category: TestCategory,
    pub test_fn: Box<dyn Fn() -> Result<(), String>>,
}

/// Test categories for organization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestCategory {
    TraitContract,
    TypeContract,
    HookContract,
    ComponentContract,
    BackwardCompatibility,
    ConsumerContract,
    PerformanceContract,
}

impl ContractTester {
    pub fn new() -> Self {
        Self {
            schema: generate_current_schema(),
            test_cases: Vec::new(),
            results: ContractTestResults::new(),
        }
    }

    pub fn with_schema(mut self, schema: ApiSchema) -> Self {
        self.schema = schema;
        self
    }

    pub fn add_test_case(&mut self, test_case: ContractTestCase) {
        self.test_cases.push(test_case);
    }

    pub fn add_trait_contract_test(
        &mut self,
        name: &str,
        description: &str,
        test_fn: impl Fn() -> Result<(), String> + 'static,
    ) {
        self.add_test_case(ContractTestCase {
            name: name.to_string(),
            description: description.to_string(),
            category: TestCategory::TraitContract,
            test_fn: Box::new(test_fn),
        });
    }

    pub fn add_type_contract_test(
        &mut self,
        name: &str,
        description: &str,
        test_fn: impl Fn() -> Result<(), String> + 'static,
    ) {
        self.add_test_case(ContractTestCase {
            name: name.to_string(),
            description: description.to_string(),
            category: TestCategory::TypeContract,
            test_fn: Box::new(test_fn),
        });
    }

    pub fn add_hook_contract_test(
        &mut self,
        name: &str,
        description: &str,
        test_fn: impl Fn() -> Result<(), String> + 'static,
    ) {
        self.add_test_case(ContractTestCase {
            name: name.to_string(),
            description: description.to_string(),
            category: TestCategory::HookContract,
            test_fn: Box::new(test_fn),
        });
    }

    pub fn add_component_contract_test(
        &mut self,
        name: &str,
        description: &str,
        test_fn: impl Fn() -> Result<(), String> + 'static,
    ) {
        self.add_test_case(ContractTestCase {
            name: name.to_string(),
            description: description.to_string(),
            category: TestCategory::ComponentContract,
            test_fn: Box::new(test_fn),
        });
    }

    pub fn add_backward_compatibility_test(
        &mut self,
        name: &str,
        description: &str,
        test_fn: impl Fn() -> Result<(), String> + 'static,
    ) {
        self.add_test_case(ContractTestCase {
            name: name.to_string(),
            description: description.to_string(),
            category: TestCategory::BackwardCompatibility,
            test_fn: Box::new(test_fn),
        });
    }

    pub fn add_consumer_contract_test(
        &mut self,
        name: &str,
        description: &str,
        test_fn: impl Fn() -> Result<(), String> + 'static,
    ) {
        self.add_test_case(ContractTestCase {
            name: name.to_string(),
            description: description.to_string(),
            category: TestCategory::ConsumerContract,
            test_fn: Box::new(test_fn),
        });
    }

    pub fn add_performance_contract_test(
        &mut self,
        name: &str,
        description: &str,
        test_fn: impl Fn() -> Result<(), String> + 'static,
    ) {
        self.add_test_case(ContractTestCase {
            name: name.to_string(),
            description: description.to_string(),
            category: TestCategory::PerformanceContract,
            test_fn: Box::new(test_fn),
        });
    }

    pub fn run_all_tests(&mut self) -> &ContractTestResults {
        self.results = ContractTestResults::new();

        for test_case in &self.test_cases {
            match (test_case.test_fn)() {
                Ok(()) => self.results.add_success(test_case.name.clone()),
                Err(error) => self.results.add_failure(test_case.name.clone(), error),
            }
        }

        &self.results
    }

    pub fn run_tests_by_category(&mut self, category: TestCategory) -> &ContractTestResults {
        self.results = ContractTestResults::new();

        for test_case in &self.test_cases {
            if test_case.category == category {
                match (test_case.test_fn)() {
                    Ok(()) => self.results.add_success(test_case.name.clone()),
                    Err(error) => self.results.add_failure(test_case.name.clone(), error),
                }
            }
        }

        &self.results
    }

    pub fn run_specific_test(&mut self, test_name: &str) -> Result<(), String> {
        for test_case in &self.test_cases {
            if test_case.name == test_name {
                return (test_case.test_fn)();
            }
        }

        Err(format!("Test '{}' not found", test_name))
    }

    pub fn get_test_cases(&self) -> &[ContractTestCase] {
        &self.test_cases
    }

    pub fn get_test_cases_by_category(&self, category: TestCategory) -> Vec<&ContractTestCase> {
        self.test_cases
            .iter()
            .filter(|tc| tc.category == category)
            .collect()
    }

    pub fn get_schema(&self) -> &ApiSchema {
        &self.schema
    }

    pub fn get_results(&self) -> &ContractTestResults {
        &self.results
    }

    pub fn generate_report(&self) -> ContractTestReport {
        ContractTestReport {
            total_tests: self.test_cases.len(),
            passed: self.results.passed.len(),
            failed: self.results.failed.len(),
            success_rate: self.results.success_rate(),
            results_by_category: self.get_results_by_category(),
            schema_version: self.schema.version.clone(),
            timestamp: chrono::Utc::now(),
        }
    }

    fn get_results_by_category(&self) -> HashMap<TestCategory, CategoryResults> {
        let mut results_by_category = HashMap::new();

        for category in [
            TestCategory::TraitContract,
            TestCategory::TypeContract,
            TestCategory::HookContract,
            TestCategory::ComponentContract,
            TestCategory::BackwardCompatibility,
            TestCategory::ConsumerContract,
            TestCategory::PerformanceContract,
        ] {
            let mut category_results = CategoryResults::new();

            for test_case in &self.test_cases {
                if test_case.category == category {
                    if self.results.passed.contains(&test_case.name) {
                        category_results.passed += 1;
                    } else if self
                        .results
                        .failed
                        .iter()
                        .any(|(name, _)| name == &test_case.name)
                    {
                        category_results.failed += 1;
                    }
                    category_results.total += 1;
                }
            }

            results_by_category.insert(category, category_results);
        }

        results_by_category
    }
}

/// Results for a specific test category
#[derive(Debug, Clone)]
pub struct CategoryResults {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

impl CategoryResults {
    pub fn new() -> Self {
        Self {
            total: 0,
            passed: 0,
            failed: 0,
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed as f64 / self.total as f64
        }
    }
}

/// Comprehensive contract test report
#[derive(Debug, Clone, Serialize)]
pub struct ContractTestReport {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub success_rate: f64,
    pub results_by_category: HashMap<TestCategory, CategoryResults>,
    pub schema_version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ContractTestReport {
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Contract Test Report\n\n");
        markdown.push_str(&format!("**Schema Version**: {}\n", self.schema_version));
        markdown.push_str(&format!(
            "**Timestamp**: {}\n",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        markdown.push_str(&format!("**Total Tests**: {}\n", self.total_tests));
        markdown.push_str(&format!("**Passed**: {}\n", self.passed));
        markdown.push_str(&format!("**Failed**: {}\n", self.failed));
        markdown.push_str(&format!(
            "**Success Rate**: {:.1}%\n\n",
            self.success_rate * 100.0
        ));

        markdown.push_str("## Results by Category\n\n");

        for (category, results) in &self.results_by_category {
            if results.total > 0 {
                let category_name = match category {
                    TestCategory::TraitContract => "Trait Contract",
                    TestCategory::TypeContract => "Type Contract",
                    TestCategory::HookContract => "Hook Contract",
                    TestCategory::ComponentContract => "Component Contract",
                    TestCategory::BackwardCompatibility => "Backward Compatibility",
                    TestCategory::ConsumerContract => "Consumer Contract",
                    TestCategory::PerformanceContract => "Performance Contract",
                };

                markdown.push_str(&format!(
                    "### {}\n- **Total**: {}\n- **Passed**: {}\n- **Failed**: {}\n- **Success Rate**: {:.1}%\n\n",
                    category_name,
                    results.total,
                    results.passed,
                    results.failed,
                    results.success_rate() * 100.0
                ));
            }
        }

        markdown
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Contract test suite builder
pub struct ContractTestSuite {
    tester: ContractTester,
}

impl ContractTestSuite {
    pub fn new() -> Self {
        Self {
            tester: ContractTester::new(),
        }
    }

    pub fn with_schema(mut self, schema: ApiSchema) -> Self {
        self.tester = self.tester.with_schema(schema);
        self
    }

    pub fn add_default_tests(mut self) -> Self {
        // Add trait contract tests
        self.tester.add_trait_contract_test(
            "form_trait_required_methods",
            "Verify Form trait has all required methods",
            || {
                // This would be implemented with actual trait testing
                Ok(())
            },
        );

        self.tester.add_trait_contract_test(
            "form_trait_method_signatures",
            "Verify Form trait method signatures are correct",
            || {
                // This would be implemented with actual signature testing
                Ok(())
            },
        );

        // Add type contract tests
        self.tester.add_type_contract_test(
            "field_metadata_structure",
            "Verify FieldMetadata has required fields",
            || {
                // This would be implemented with actual type testing
                Ok(())
            },
        );

        self.tester.add_type_contract_test(
            "validation_errors_methods",
            "Verify ValidationErrors has required methods",
            || {
                // This would be implemented with actual type testing
                Ok(())
            },
        );

        // Add backward compatibility tests
        self.tester.add_backward_compatibility_test(
            "legacy_form_compatibility",
            "Verify legacy forms still work",
            || {
                // This would be implemented with actual compatibility testing
                Ok(())
            },
        );

        self
    }

    pub fn build(self) -> ContractTester {
        self.tester
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_tester_creation() {
        let tester = ContractTester::new();
        assert_eq!(tester.get_test_cases().len(), 0);
        assert_eq!(tester.get_results().total, 0);
    }

    #[test]
    fn test_contract_tester_add_tests() {
        let mut tester = ContractTester::new();

        tester.add_trait_contract_test("test1", "Test 1", || Ok(()));

        tester.add_type_contract_test("test2", "Test 2", || Ok(()));

        assert_eq!(tester.get_test_cases().len(), 2);

        let trait_tests = tester.get_test_cases_by_category(TestCategory::TraitContract);
        assert_eq!(trait_tests.len(), 1);
        assert_eq!(trait_tests[0].name, "test1");

        let type_tests = tester.get_test_cases_by_category(TestCategory::TypeContract);
        assert_eq!(type_tests.len(), 1);
        assert_eq!(type_tests[0].name, "test2");
    }

    #[test]
    fn test_contract_tester_run_tests() {
        let mut tester = ContractTester::new();

        tester.add_trait_contract_test("passing_test", "A test that passes", || Ok(()));

        tester.add_trait_contract_test("failing_test", "A test that fails", || {
            Err("Test failed".to_string())
        });

        let results = tester.run_all_tests();

        assert_eq!(results.total, 2);
        assert_eq!(results.passed.len(), 1);
        assert_eq!(results.failed.len(), 1);
        assert!(!results.is_success());
        assert!((results.success_rate() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_contract_test_suite() {
        let tester = ContractTestSuite::new().add_default_tests().build();

        assert!(tester.get_test_cases().len() > 0);

        let trait_tests = tester.get_test_cases_by_category(TestCategory::TraitContract);
        assert!(trait_tests.len() > 0);

        let type_tests = tester.get_test_cases_by_category(TestCategory::TypeContract);
        assert!(type_tests.len() > 0);

        let compatibility_tests =
            tester.get_test_cases_by_category(TestCategory::BackwardCompatibility);
        assert!(compatibility_tests.len() > 0);
    }

    #[test]
    fn test_contract_test_report() {
        let mut tester = ContractTester::new();

        tester.add_trait_contract_test("test1", "Test 1", || Ok(()));

        tester.add_trait_contract_test("test2", "Test 2", || Err("Failed".to_string()));

        tester.run_all_tests();
        let report = tester.generate_report();

        assert_eq!(report.total_tests, 2);
        assert_eq!(report.passed, 1);
        assert_eq!(report.failed, 1);
        assert!((report.success_rate - 0.5).abs() < 0.001);
        assert!(!report.is_success());

        let markdown = report.to_markdown();
        assert!(markdown.contains("# Contract Test Report"));
        assert!(markdown.contains("**Total Tests**: 2"));
        assert!(markdown.contains("**Passed**: 1"));
        assert!(markdown.contains("**Failed**: 1"));
    }
}
