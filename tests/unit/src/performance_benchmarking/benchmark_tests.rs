//! Benchmark tests for performance benchmarking

use super::types::BenchmarkForm;
use leptos_forms_rs::Form;

#[test]
fn test_benchmark_form() {
    let form = BenchmarkForm::default();
    assert_eq!(form.field1, "");
    assert_eq!(form.field2, "");

    let result = form.validate();
    assert!(result.is_ok());
}
