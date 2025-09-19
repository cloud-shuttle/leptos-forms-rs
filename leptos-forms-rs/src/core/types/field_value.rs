//! Field value module - Dynamic field value representation
//!
//! This module provides the FieldValue enum for representing dynamic field values,
//! including type conversion methods, validation helpers, and serialization support.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use super::file_data::FileData;

/// Dynamic field value representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
    Array(Vec<FieldValue>),
    Object(HashMap<String, FieldValue>),
    File(FileData),
    Null,
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldValue::String(s) => write!(f, "{}", s),
            FieldValue::Number(n) => write!(f, "{}", n),
            FieldValue::Integer(i) => write!(f, "{}", i),
            FieldValue::Boolean(b) => write!(f, "{}", b),
            FieldValue::Date(d) => write!(f, "{}", d),
            FieldValue::DateTime(dt) => write!(f, "{}", dt),
            FieldValue::Array(_) => write!(f, "[]"),
            FieldValue::Object(_) => write!(f, "{{}}"),
            FieldValue::File(_) => write!(f, "file"),
            FieldValue::Null => write!(f, "null"),
        }
    }
}

impl FieldValue {
    /// Get the value as a string reference
    pub fn as_string(&self) -> Option<&String> {
        match self {
            FieldValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get the value as a number (f64)
    pub fn as_number(&self) -> Option<f64> {
        match self {
            FieldValue::Number(n) => Some(*n),
            FieldValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Get the value as an integer (i64)
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            FieldValue::Integer(i) => Some(*i),
            FieldValue::Number(n) => Some(*n as i64),
            _ => None,
        }
    }

    /// Get the value as a boolean
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            FieldValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Get the value as a date
    pub fn as_date(&self) -> Option<NaiveDate> {
        match self {
            FieldValue::Date(d) => Some(*d),
            _ => None,
        }
    }

    /// Get the value as a datetime
    pub fn as_datetime(&self) -> Option<DateTime<Utc>> {
        match self {
            FieldValue::DateTime(dt) => Some(*dt),
            _ => None,
        }
    }

    /// Get the value as an array reference
    pub fn as_array(&self) -> Option<&Vec<FieldValue>> {
        match self {
            FieldValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Get the value as an object reference
    pub fn as_object(&self) -> Option<&HashMap<String, FieldValue>> {
        match self {
            FieldValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Get the value as a file data reference
    pub fn as_file(&self) -> Option<&FileData> {
        match self {
            FieldValue::File(file) => Some(file),
            _ => None,
        }
    }

    /// Check if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, FieldValue::Null)
    }

    /// Check if the value is empty
    pub fn is_empty(&self) -> bool {
        match self {
            FieldValue::String(s) => s.is_empty(),
            FieldValue::Array(arr) => arr.is_empty(),
            FieldValue::Object(obj) => obj.is_empty(),
            FieldValue::Null => true,
            _ => false,
        }
    }

    /// Check if the value is a string type
    pub fn is_string(&self) -> bool {
        matches!(self, FieldValue::String(_))
    }

    /// Check if the value is a number type
    pub fn is_number(&self) -> bool {
        matches!(self, FieldValue::Number(_) | FieldValue::Integer(_))
    }

    /// Check if the value is a boolean type
    pub fn is_boolean(&self) -> bool {
        matches!(self, FieldValue::Boolean(_))
    }

    /// Check if the value is a date type
    pub fn is_date(&self) -> bool {
        matches!(self, FieldValue::Date(_) | FieldValue::DateTime(_))
    }

    /// Check if the value is an array type
    pub fn is_array(&self) -> bool {
        matches!(self, FieldValue::Array(_))
    }

    /// Check if the value is an object type
    pub fn is_object(&self) -> bool {
        matches!(self, FieldValue::Object(_))
    }

    /// Check if the value is a file type
    pub fn is_file(&self) -> bool {
        matches!(self, FieldValue::File(_))
    }

    /// Get the type name of the value
    pub fn type_name(&self) -> &'static str {
        match self {
            FieldValue::String(_) => "string",
            FieldValue::Number(_) => "number",
            FieldValue::Integer(_) => "integer",
            FieldValue::Boolean(_) => "boolean",
            FieldValue::Date(_) => "date",
            FieldValue::DateTime(_) => "datetime",
            FieldValue::Array(_) => "array",
            FieldValue::Object(_) => "object",
            FieldValue::File(_) => "file",
            FieldValue::Null => "null",
        }
    }

    /// Convert to a string representation
    pub fn to_string(&self) -> String {
        match self {
            FieldValue::String(s) => s.clone(),
            FieldValue::Number(n) => n.to_string(),
            FieldValue::Integer(i) => i.to_string(),
            FieldValue::Boolean(b) => b.to_string(),
            FieldValue::Date(d) => d.to_string(),
            FieldValue::DateTime(dt) => dt.to_string(),
            FieldValue::Array(_) => "[]".to_string(),
            FieldValue::Object(_) => "{}".to_string(),
            FieldValue::File(file) => format!("file:{}", file.name),
            FieldValue::Null => "null".to_string(),
        }
    }

    /// Try to convert to a number, returning None if not possible
    pub fn to_number(&self) -> Option<f64> {
        match self {
            FieldValue::Number(n) => Some(*n),
            FieldValue::Integer(i) => Some(*i as f64),
            FieldValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Try to convert to a boolean, returning None if not possible
    pub fn to_boolean(&self) -> Option<bool> {
        match self {
            FieldValue::Boolean(b) => Some(*b),
            FieldValue::String(s) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Some(true),
                "false" | "0" | "no" | "off" => Some(false),
                _ => None,
            },
            FieldValue::Number(n) => Some(*n != 0.0),
            FieldValue::Integer(i) => Some(*i != 0),
            _ => None,
        }
    }

    /// Get the length of the value (for strings, arrays, objects)
    pub fn len(&self) -> Option<usize> {
        match self {
            FieldValue::String(s) => Some(s.len()),
            FieldValue::Array(arr) => Some(arr.len()),
            FieldValue::Object(obj) => Some(obj.len()),
            _ => None,
        }
    }

    /// Check if the value has a specific length
    pub fn has_length(&self, length: usize) -> bool {
        self.len().map_or(false, |len| len == length)
    }

    /// Check if the value is longer than a specific length
    pub fn is_longer_than(&self, length: usize) -> bool {
        self.len().map_or(false, |len| len > length)
    }

    /// Check if the value is shorter than a specific length
    pub fn is_shorter_than(&self, length: usize) -> bool {
        self.len().map_or(false, |len| len < length)
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        FieldValue::String(value)
    }
}

impl From<&str> for FieldValue {
    fn from(value: &str) -> Self {
        FieldValue::String(value.to_string())
    }
}

impl From<f64> for FieldValue {
    fn from(value: f64) -> Self {
        FieldValue::Number(value)
    }
}

impl From<i64> for FieldValue {
    fn from(value: i64) -> Self {
        FieldValue::Integer(value)
    }
}

impl From<i32> for FieldValue {
    fn from(value: i32) -> Self {
        FieldValue::Integer(value as i64)
    }
}

impl From<bool> for FieldValue {
    fn from(value: bool) -> Self {
        FieldValue::Boolean(value)
    }
}

impl From<NaiveDate> for FieldValue {
    fn from(value: NaiveDate) -> Self {
        FieldValue::Date(value)
    }
}

impl From<DateTime<Utc>> for FieldValue {
    fn from(value: DateTime<Utc>) -> Self {
        FieldValue::DateTime(value)
    }
}

impl From<Vec<FieldValue>> for FieldValue {
    fn from(value: Vec<FieldValue>) -> Self {
        FieldValue::Array(value)
    }
}

impl From<HashMap<String, FieldValue>> for FieldValue {
    fn from(value: HashMap<String, FieldValue>) -> Self {
        FieldValue::Object(value)
    }
}

impl From<FileData> for FieldValue {
    fn from(value: FileData) -> Self {
        FieldValue::File(value)
    }
}
