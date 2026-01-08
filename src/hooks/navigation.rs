//! Navigation Hook
//!
//! Linked-list style navigation for wizards, multi-step forms, and sequences.
//!
//! Features:
//! - Step-based navigation (next/previous/goto)
//! - History tracking with back/forward
//! - Step validation before proceeding
//! - Progress percentage calculation
//! - Custom step metadata

use std::collections::HashMap;

// =============================================================================
// Types
// =============================================================================

/// Navigation step with metadata.
#[derive(Debug, Clone)]
pub struct NavigationStep<T = ()> {
    /// Step ID (unique identifier)
    pub id: String,
    /// Step label for display
    pub label: String,
    /// Optional description
    pub description: Option<String>,
    /// Whether step is skippable
    pub skippable: bool,
    /// Custom metadata
    pub data: T,
}

impl<T: Default> NavigationStep<T> {
    /// Create a new navigation step.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            skippable: false,
            data: T::default(),
        }
    }
}

impl<T> NavigationStep<T> {
    /// Create a new navigation step with custom data.
    pub fn with_data(id: impl Into<String>, label: impl Into<String>, data: T) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            skippable: false,
            data,
        }
    }

    /// Set description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set skippable.
    pub fn skippable(mut self, skippable: bool) -> Self {
        self.skippable = skippable;
        self
    }
}

/// Navigation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationResult {
    /// Navigation successful
    Success,
    /// Cannot go back (at first step)
    AtStart,
    /// Cannot go forward (at last step)
    AtEnd,
    /// Step not found
    NotFound,
    /// Validation failed
    ValidationFailed,
}

/// Navigation state for wizard-style flows.
#[derive(Debug, Clone)]
pub struct NavigationState<T = ()> {
    /// All steps in order
    steps: Vec<NavigationStep<T>>,
    /// Current step index
    current_index: usize,
    /// History of visited step indices (for back/forward)
    history: Vec<usize>,
    /// Current position in history
    history_position: usize,
    /// Steps marked as complete
    completed: HashMap<String, bool>,
    /// Validation function (returns true if can proceed)
    validator: Option<fn(&NavigationStep<T>) -> bool>,
}

impl<T> Default for NavigationState<T> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            current_index: 0,
            history: vec![0],
            history_position: 0,
            completed: HashMap::new(),
            validator: None,
        }
    }
}

impl<T: Clone> NavigationState<T> {
    /// Create a new navigation state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create navigation with steps.
    pub fn with_steps<I>(steps: I) -> Self
    where
        I: IntoIterator<Item = NavigationStep<T>>,
    {
        let steps: Vec<_> = steps.into_iter().collect();
        let history = if steps.is_empty() { vec![] } else { vec![0] };
        Self {
            steps,
            current_index: 0,
            history,
            history_position: 0,
            completed: HashMap::new(),
            validator: None,
        }
    }

    /// Add a step.
    pub fn add_step(&mut self, step: NavigationStep<T>) {
        self.steps.push(step);
        if self.history.is_empty() {
            self.history.push(0);
        }
    }

    /// Set validator function.
    pub fn set_validator(&mut self, validator: fn(&NavigationStep<T>) -> bool) {
        self.validator = Some(validator);
    }

    /// Get current step.
    pub fn current(&self) -> Option<&NavigationStep<T>> {
        self.steps.get(self.current_index)
    }

    /// Get current step index.
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Get total number of steps.
    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }

    /// Get all steps.
    pub fn steps(&self) -> &[NavigationStep<T>] {
        &self.steps
    }

    /// Check if at first step.
    pub fn is_first(&self) -> bool {
        self.current_index == 0
    }

    /// Check if at last step.
    pub fn is_last(&self) -> bool {
        self.current_index >= self.steps.len().saturating_sub(1)
    }

    /// Get progress percentage (0.0 - 1.0).
    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.current_index as f64 / (self.steps.len() - 1).max(1) as f64
    }

    /// Get progress as percentage (0 - 100).
    pub fn progress_percent(&self) -> u8 {
        (self.progress() * 100.0) as u8
    }

    /// Go to next step.
    pub fn next(&mut self) -> NavigationResult {
        if self.is_last() {
            return NavigationResult::AtEnd;
        }

        // Validate current step before proceeding
        if let Some(validator) = self.validator {
            if let Some(current) = self.current() {
                if !validator(current) {
                    return NavigationResult::ValidationFailed;
                }
            }
        }

        self.current_index += 1;
        self.record_history();
        NavigationResult::Success
    }

    /// Go to previous step.
    pub fn previous(&mut self) -> NavigationResult {
        if self.is_first() {
            return NavigationResult::AtStart;
        }

        self.current_index -= 1;
        self.record_history();
        NavigationResult::Success
    }

    /// Go to specific step by index.
    pub fn goto(&mut self, index: usize) -> NavigationResult {
        if index >= self.steps.len() {
            return NavigationResult::NotFound;
        }

        self.current_index = index;
        self.record_history();
        NavigationResult::Success
    }

    /// Go to step by ID.
    pub fn goto_id(&mut self, id: &str) -> NavigationResult {
        if let Some(index) = self.steps.iter().position(|s| s.id == id) {
            self.goto(index)
        } else {
            NavigationResult::NotFound
        }
    }

    /// Go to first step.
    pub fn first(&mut self) -> NavigationResult {
        self.goto(0)
    }

    /// Go to last step.
    pub fn last(&mut self) -> NavigationResult {
        if self.steps.is_empty() {
            return NavigationResult::NotFound;
        }
        self.goto(self.steps.len() - 1)
    }

    /// Go back in history.
    pub fn back(&mut self) -> NavigationResult {
        if self.history_position == 0 {
            return NavigationResult::AtStart;
        }

        self.history_position -= 1;
        self.current_index = self.history[self.history_position];
        NavigationResult::Success
    }

    /// Go forward in history.
    pub fn forward(&mut self) -> NavigationResult {
        if self.history_position >= self.history.len() - 1 {
            return NavigationResult::AtEnd;
        }

        self.history_position += 1;
        self.current_index = self.history[self.history_position];
        NavigationResult::Success
    }

    /// Mark current step as complete.
    pub fn mark_complete(&mut self) {
        if let Some(step) = self.current() {
            self.completed.insert(step.id.clone(), true);
        }
    }

    /// Check if step is complete.
    pub fn is_complete(&self, id: &str) -> bool {
        self.completed.get(id).copied().unwrap_or(false)
    }

    /// Get number of completed steps.
    pub fn completed_count(&self) -> usize {
        self.completed.values().filter(|&&v| v).count()
    }

    /// Check if all steps are complete.
    pub fn all_complete(&self) -> bool {
        self.completed_count() == self.steps.len()
    }

    /// Reset navigation to first step.
    pub fn reset(&mut self) {
        self.current_index = 0;
        self.history = vec![0];
        self.history_position = 0;
        self.completed.clear();
    }

    /// Record current position in history.
    fn record_history(&mut self) {
        // Truncate any forward history
        self.history.truncate(self.history_position + 1);
        // Add current position
        self.history.push(self.current_index);
        self.history_position = self.history.len() - 1;
    }

    /// Can go back in history?
    pub fn can_back(&self) -> bool {
        self.history_position > 0
    }

    /// Can go forward in history?
    pub fn can_forward(&self) -> bool {
        self.history_position < self.history.len() - 1
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a new navigation state.
pub fn use_navigation<T: Clone>() -> NavigationState<T> {
    NavigationState::new()
}

/// Create navigation with steps.
pub fn create_navigation<T: Clone, I>(steps: I) -> NavigationState<T>
where
    I: IntoIterator<Item = NavigationStep<T>>,
{
    NavigationState::with_steps(steps)
}

/// Create simple string-based navigation steps.
pub fn simple_steps<I, S>(labels: I) -> Vec<NavigationStep<()>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    labels
        .into_iter()
        .enumerate()
        .map(|(i, label)| {
            let label_str: String = label.into();
            NavigationStep::new(format!("step_{}", i), label_str)
        })
        .collect()
}
