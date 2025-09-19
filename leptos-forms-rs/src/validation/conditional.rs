//! Conditional validation module - Conditional validation logic
//!
//! This module provides conditional validation capabilities including
//! ConditionalValidator, FieldCondition, and ConditionalRule for implementing
//! field dependencies and conditional validation rules.

use super::rules::FieldValidator;
use crate::core::types::FieldValue;
use crate::core::Form;

/// Conditional validation engine for field dependencies
#[derive(Default)]
pub struct ConditionalValidator {
    rules: Vec<ConditionalRule>,
}

impl ConditionalValidator {
    /// Create a new conditional validator
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a conditional rule
    pub fn add_rule(&mut self, rule: ConditionalRule) {
        self.rules.push(rule);
    }

    /// Validate conditional fields
    pub fn validate_conditional_fields<T: Form>(
        &self,
        form: &T,
        field_name: &str,
        field_value: &FieldValue,
    ) -> Result<(), String> {
        // Find rules that apply to this field
        for rule in &self.rules {
            if rule.target_field == field_name {
                if let Some(condition) = &rule.condition {
                    // Check if the condition is met
                    let condition_met = self.evaluate_condition(form, condition)?;

                    if condition_met {
                        // Apply the validation rule
                        for validator in &rule.validators {
                            validator(field_value)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Evaluate a field condition
    #[allow(clippy::only_used_in_recursion)]
    fn evaluate_condition<T: Form>(
        &self,
        form: &T,
        condition: &FieldCondition,
    ) -> Result<bool, String> {
        match condition {
            FieldCondition::Equals(field, value) => {
                let field_value = form.get_field_value(field);
                Ok(field_value == *value)
            }
            FieldCondition::NotEquals(field, value) => {
                let field_value = form.get_field_value(field);
                Ok(field_value != *value)
            }
            FieldCondition::Contains(field, value) => {
                if let FieldValue::String(field_str) = form.get_field_value(field) {
                    Ok(field_str.contains(value))
                } else {
                    Ok(false)
                }
            }
            FieldCondition::IsEmpty(field) => {
                let field_value = form.get_field_value(field);
                Ok(field_value.is_empty())
            }
            FieldCondition::IsNotEmpty(field) => {
                let field_value = form.get_field_value(field);
                Ok(!field_value.is_empty())
            }
            FieldCondition::And(conditions) => {
                for condition in conditions {
                    if !self.evaluate_condition(form, condition)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            FieldCondition::Or(conditions) => {
                for condition in conditions {
                    if self.evaluate_condition(form, condition)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }

    /// Get all rules for a specific field
    pub fn get_rules_for_field(&self, field_name: &str) -> Vec<&ConditionalRule> {
        self.rules
            .iter()
            .filter(|rule| rule.target_field == field_name)
            .collect()
    }

    /// Check if a field has conditional rules
    pub fn has_rules_for_field(&self, field_name: &str) -> bool {
        self.rules
            .iter()
            .any(|rule| rule.target_field == field_name)
    }

    /// Get all conditional rules
    pub fn get_all_rules(&self) -> &[ConditionalRule] {
        &self.rules
    }

    /// Clear all conditional rules
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    /// Remove rules for a specific field
    pub fn remove_rules_for_field(&mut self, field_name: &str) {
        self.rules.retain(|rule| rule.target_field != field_name);
    }
}

/// A conditional validation rule
pub struct ConditionalRule {
    pub target_field: String,
    pub condition: Option<FieldCondition>,
    pub validators: Vec<FieldValidator>,
    pub error_message: Option<String>,
}

impl std::fmt::Debug for ConditionalRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConditionalRule")
            .field("target_field", &self.target_field)
            .field("condition", &self.condition)
            .field(
                "validators",
                &format!("{} validators", self.validators.len()),
            )
            .field("error_message", &self.error_message)
            .finish()
    }
}

impl ConditionalRule {
    /// Create a new conditional rule
    pub fn new(target_field: String) -> Self {
        Self {
            target_field,
            condition: None,
            validators: Vec::new(),
            error_message: None,
        }
    }

    /// Set the condition for this rule
    pub fn when(mut self, condition: FieldCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Add a validator to this rule
    pub fn validate_with(mut self, validator: FieldValidator) -> Self {
        self.validators.push(validator);
        self
    }

    /// Set the error message for this rule
    pub fn with_error_message(mut self, message: String) -> Self {
        self.error_message = Some(message);
        self
    }

    /// Get the target field name
    pub fn target_field(&self) -> &str {
        &self.target_field
    }

    /// Get the condition for this rule
    pub fn condition(&self) -> Option<&FieldCondition> {
        self.condition.as_ref()
    }

    /// Get the validators for this rule
    pub fn validators(&self) -> &[FieldValidator] {
        &self.validators
    }

    /// Get the error message for this rule
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_ref().map(|s| s.as_str())
    }

    /// Check if this rule has a condition
    pub fn has_condition(&self) -> bool {
        self.condition.is_some()
    }

    /// Check if this rule has validators
    pub fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }
}

/// Field conditions for conditional validation
#[derive(Debug, Clone)]
pub enum FieldCondition {
    Equals(String, FieldValue),
    NotEquals(String, FieldValue),
    Contains(String, String),
    IsEmpty(String),
    IsNotEmpty(String),
    And(Vec<FieldCondition>),
    Or(Vec<FieldCondition>),
}

impl FieldCondition {
    /// Create an equals condition
    pub fn equals(field: &str, value: FieldValue) -> Self {
        Self::Equals(field.to_string(), value)
    }

    /// Create a not equals condition
    pub fn not_equals(field: &str, value: FieldValue) -> Self {
        Self::NotEquals(field.to_string(), value)
    }

    /// Create a contains condition
    pub fn contains(field: &str, value: &str) -> Self {
        Self::Contains(field.to_string(), value.to_string())
    }

    /// Create an is empty condition
    pub fn is_empty(field: &str) -> Self {
        Self::IsEmpty(field.to_string())
    }

    /// Create an is not empty condition
    pub fn is_not_empty(field: &str) -> Self {
        Self::IsNotEmpty(field.to_string())
    }

    /// Create an AND condition
    pub fn and(conditions: Vec<FieldCondition>) -> Self {
        Self::And(conditions)
    }

    /// Create an OR condition
    pub fn or(conditions: Vec<FieldCondition>) -> Self {
        Self::Or(conditions)
    }

    /// Get the field name for this condition
    pub fn field_name(&self) -> Option<&str> {
        match self {
            FieldCondition::Equals(field, _) => Some(field),
            FieldCondition::NotEquals(field, _) => Some(field),
            FieldCondition::Contains(field, _) => Some(field),
            FieldCondition::IsEmpty(field) => Some(field),
            FieldCondition::IsNotEmpty(field) => Some(field),
            FieldCondition::And(_) => None,
            FieldCondition::Or(_) => None,
        }
    }

    /// Check if this condition is a logical operator
    pub fn is_logical_operator(&self) -> bool {
        matches!(self, FieldCondition::And(_) | FieldCondition::Or(_))
    }

    /// Get the logical operator type if this is a logical operator
    pub fn logical_operator_type(&self) -> Option<LogicalOperator> {
        match self {
            FieldCondition::And(_) => Some(LogicalOperator::And),
            FieldCondition::Or(_) => Some(LogicalOperator::Or),
            _ => None,
        }
    }
}

/// Logical operators for field conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl LogicalOperator {
    /// Get the string representation of this operator
    pub fn as_str(&self) -> &'static str {
        match self {
            LogicalOperator::And => "AND",
            LogicalOperator::Or => "OR",
        }
    }
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
