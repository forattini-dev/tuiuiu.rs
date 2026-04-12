//! Form Hook
//!
//! A hook for managing form state, validation, and submission.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Form field value types.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Select(String),
    MultiSelect(Vec<String>),
}

impl Default for FieldValue {
    fn default() -> Self {
        FieldValue::Text(String::new())
    }
}

impl FieldValue {
    /// Get as text.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            FieldValue::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Get as number.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            FieldValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get as boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FieldValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as select value.
    pub fn as_select(&self) -> Option<&str> {
        match self {
            FieldValue::Select(s) => Some(s),
            _ => None,
        }
    }

    /// Get as multi-select values.
    pub fn as_multi_select(&self) -> Option<&[String]> {
        match self {
            FieldValue::MultiSelect(v) => Some(v),
            _ => None,
        }
    }
}

/// Validation result for a field.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

impl ValidationResult {
    /// Check if validation passed.
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Get error message if invalid.
    pub fn error(&self) -> Option<&str> {
        match self {
            ValidationResult::Invalid(msg) => Some(msg),
            _ => None,
        }
    }
}

/// Field validator function type.
pub type Validator = Box<dyn Fn(&FieldValue) -> ValidationResult + Send + Sync>;

/// Built-in validators.
pub mod validators {
    use super::*;

    /// Field is required (non-empty).
    pub fn required() -> Validator {
        Box::new(|value| {
            let is_empty = match value {
                FieldValue::Text(s) => s.trim().is_empty(),
                FieldValue::MultiSelect(v) => v.is_empty(),
                _ => false,
            };
            if is_empty {
                ValidationResult::Invalid("This field is required".to_string())
            } else {
                ValidationResult::Valid
            }
        })
    }

    /// Minimum length for text.
    pub fn min_length(min: usize) -> Validator {
        Box::new(move |value| {
            if let FieldValue::Text(s) = value {
                if s.len() < min {
                    return ValidationResult::Invalid(format!(
                        "Must be at least {} characters",
                        min
                    ));
                }
            }
            ValidationResult::Valid
        })
    }

    /// Maximum length for text.
    pub fn max_length(max: usize) -> Validator {
        Box::new(move |value| {
            if let FieldValue::Text(s) = value {
                if s.len() > max {
                    return ValidationResult::Invalid(format!(
                        "Must be at most {} characters",
                        max
                    ));
                }
            }
            ValidationResult::Valid
        })
    }

    /// Minimum value for numbers.
    pub fn min_value(min: f64) -> Validator {
        Box::new(move |value| {
            if let FieldValue::Number(n) = value {
                if *n < min {
                    return ValidationResult::Invalid(format!("Must be at least {}", min));
                }
            }
            ValidationResult::Valid
        })
    }

    /// Maximum value for numbers.
    pub fn max_value(max: f64) -> Validator {
        Box::new(move |value| {
            if let FieldValue::Number(n) = value {
                if *n > max {
                    return ValidationResult::Invalid(format!("Must be at most {}", max));
                }
            }
            ValidationResult::Valid
        })
    }

    /// Email format validation.
    pub fn email() -> Validator {
        Box::new(|value| {
            if let FieldValue::Text(s) = value {
                // Simple email regex check
                if !s.is_empty() && !s.contains('@') {
                    return ValidationResult::Invalid("Invalid email format".to_string());
                }
            }
            ValidationResult::Valid
        })
    }

    /// Pattern matching (simplified regex-like).
    pub fn pattern(pattern: &'static str, message: &'static str) -> Validator {
        Box::new(move |value| {
            if let FieldValue::Text(s) = value {
                // Simple pattern check - in real impl would use regex
                let matches = match pattern {
                    r"^\d+$" => s.chars().all(|c| c.is_ascii_digit()),
                    r"^[a-zA-Z]+$" => s.chars().all(|c| c.is_ascii_alphabetic()),
                    r"^[a-zA-Z0-9]+$" => s.chars().all(|c| c.is_ascii_alphanumeric()),
                    _ => true,
                };
                if !s.is_empty() && !matches {
                    return ValidationResult::Invalid(message.to_string());
                }
            }
            ValidationResult::Valid
        })
    }

    /// Custom validator with closure.
    pub fn custom<F>(f: F) -> Validator
    where
        F: Fn(&FieldValue) -> ValidationResult + Send + Sync + 'static,
    {
        Box::new(f)
    }
}

/// Form field definition.
pub struct FormField {
    /// Field name
    pub name: String,
    /// Initial value
    pub initial_value: FieldValue,
    /// Validators
    pub validators: Vec<Validator>,
}

impl FormField {
    /// Create a new field.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            initial_value: FieldValue::default(),
            validators: Vec::new(),
        }
    }

    /// Set initial value.
    pub fn initial(mut self, value: FieldValue) -> Self {
        self.initial_value = value;
        self
    }

    /// Add a validator.
    pub fn validate(mut self, validator: Validator) -> Self {
        self.validators.push(validator);
        self
    }

    /// Mark as required.
    pub fn required(self) -> Self {
        self.validate(validators::required())
    }
}

/// Form state.
#[derive(Debug, Clone)]
pub struct FormState {
    /// Field values
    values: HashMap<String, FieldValue>,
    /// Field errors
    errors: HashMap<String, String>,
    /// Touched fields
    touched: HashMap<String, bool>,
    /// Form is submitting
    submitting: bool,
    /// Form was submitted at least once
    submitted: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            errors: HashMap::new(),
            touched: HashMap::new(),
            submitting: false,
            submitted: false,
        }
    }
}

impl FormState {
    /// Get field value.
    pub fn get(&self, name: &str) -> Option<&FieldValue> {
        self.values.get(name)
    }

    /// Get field value as text.
    pub fn get_text(&self, name: &str) -> Option<&str> {
        self.values.get(name).and_then(|v| v.as_text())
    }

    /// Get field value as number.
    pub fn get_number(&self, name: &str) -> Option<f64> {
        self.values.get(name).and_then(|v| v.as_number())
    }

    /// Get field value as boolean.
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.values.get(name).and_then(|v| v.as_bool())
    }

    /// Get field error.
    pub fn error(&self, name: &str) -> Option<&str> {
        self.errors.get(name).map(|s| s.as_str())
    }

    /// Check if field was touched.
    pub fn is_touched(&self, name: &str) -> bool {
        self.touched.get(name).copied().unwrap_or(false)
    }

    /// Check if form is valid (no errors).
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if form is submitting.
    pub fn is_submitting(&self) -> bool {
        self.submitting
    }

    /// Check if form was submitted.
    pub fn was_submitted(&self) -> bool {
        self.submitted
    }

    /// Get all values.
    pub fn values(&self) -> &HashMap<String, FieldValue> {
        &self.values
    }

    /// Get all errors.
    pub fn errors(&self) -> &HashMap<String, String> {
        &self.errors
    }
}

/// Form handle for interacting with the form.
#[derive(Clone)]
pub struct FormHandle {
    state: Arc<Mutex<FormState>>,
    validators: Arc<HashMap<String, Vec<Validator>>>,
}

impl FormHandle {
    /// Get current state.
    pub fn state(&self) -> FormState {
        self.state.lock().unwrap().clone()
    }

    /// Set field value.
    pub fn set_value(&self, name: &str, value: FieldValue) {
        let mut state = self.state.lock().unwrap();
        state.values.insert(name.to_string(), value.clone());

        // Validate if touched or submitted
        if state.is_touched(name) || state.submitted {
            self.validate_field_internal(&mut state, name, &value);
        }
    }

    /// Set text value.
    pub fn set_text(&self, name: &str, value: impl Into<String>) {
        self.set_value(name, FieldValue::Text(value.into()));
    }

    /// Set number value.
    pub fn set_number(&self, name: &str, value: f64) {
        self.set_value(name, FieldValue::Number(value));
    }

    /// Set boolean value.
    pub fn set_bool(&self, name: &str, value: bool) {
        self.set_value(name, FieldValue::Boolean(value));
    }

    /// Mark field as touched.
    pub fn touch(&self, name: &str) {
        let mut state = self.state.lock().unwrap();
        state.touched.insert(name.to_string(), true);

        // Validate on touch
        if let Some(value) = state.values.get(name).cloned() {
            self.validate_field_internal(&mut state, name, &value);
        }
    }

    /// Validate single field.
    fn validate_field_internal(&self, state: &mut FormState, name: &str, value: &FieldValue) {
        if let Some(validators) = self.validators.get(name) {
            for validator in validators {
                match validator(value) {
                    ValidationResult::Valid => {
                        state.errors.remove(name);
                    }
                    ValidationResult::Invalid(msg) => {
                        state.errors.insert(name.to_string(), msg);
                        return;
                    }
                }
            }
        }
    }

    /// Validate all fields.
    pub fn validate(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        state.errors.clear();

        for (name, validators) in self.validators.iter() {
            if let Some(value) = state.values.get(name).cloned() {
                for validator in validators {
                    if let ValidationResult::Invalid(msg) = validator(&value) {
                        state.errors.insert(name.clone(), msg);
                        break;
                    }
                }
            }
        }

        state.errors.is_empty()
    }

    /// Submit the form.
    pub fn submit<F>(&self, on_submit: F) -> bool
    where
        F: FnOnce(&HashMap<String, FieldValue>),
    {
        // Mark as submitted
        {
            let mut state = self.state.lock().unwrap();
            state.submitted = true;
        }

        // Validate
        if !self.validate() {
            return false;
        }

        // Get values and call handler
        let values = {
            let mut state = self.state.lock().unwrap();
            state.submitting = true;
            state.values.clone()
        };

        on_submit(&values);

        // Mark as not submitting
        {
            let mut state = self.state.lock().unwrap();
            state.submitting = false;
        }

        true
    }

    /// Reset form to initial values.
    pub fn reset(&self, initial_values: &HashMap<String, FieldValue>) {
        let mut state = self.state.lock().unwrap();
        state.values = initial_values.clone();
        state.errors.clear();
        state.touched.clear();
        state.submitted = false;
        state.submitting = false;
    }

    /// Set error manually.
    pub fn set_error(&self, name: &str, error: impl Into<String>) {
        let mut state = self.state.lock().unwrap();
        state.errors.insert(name.to_string(), error.into());
    }

    /// Clear error.
    pub fn clear_error(&self, name: &str) {
        let mut state = self.state.lock().unwrap();
        state.errors.remove(name);
    }

    /// Clear all errors.
    pub fn clear_errors(&self) {
        let mut state = self.state.lock().unwrap();
        state.errors.clear();
    }
}

/// Create a form hook with field definitions.
pub fn use_form(fields: Vec<FormField>) -> FormHandle {
    let mut initial_values = HashMap::new();
    let mut validators_map = HashMap::new();

    for field in fields {
        initial_values.insert(field.name.clone(), field.initial_value);
        if !field.validators.is_empty() {
            validators_map.insert(field.name, field.validators);
        }
    }

    let state = Arc::new(Mutex::new(FormState {
        values: initial_values,
        ..Default::default()
    }));

    FormHandle {
        state,
        validators: Arc::new(validators_map),
    }
}

/// Create an empty form.
pub fn create_form() -> FormHandle {
    use_form(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_basic() {
        let form = use_form(vec![
            FormField::new("name")
                .initial(FieldValue::Text("".to_string()))
                .required(),
            FormField::new("email")
                .initial(FieldValue::Text("".to_string()))
                .validate(validators::email()),
        ]);

        // Set values
        form.set_text("name", "John");
        form.set_text("email", "john@example.com");

        // Validate
        assert!(form.validate());
        assert!(form.state().is_valid());
    }

    #[test]
    fn test_form_validation_error() {
        let form = use_form(vec![FormField::new("name").required()]);

        // Empty required field
        form.set_text("name", "");

        // Validate
        assert!(!form.validate());
        assert!(form.state().error("name").is_some());
    }
}
