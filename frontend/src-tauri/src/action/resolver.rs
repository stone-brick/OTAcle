//! Action parameter resolver - handles placeholder substitution
//!
//! This module provides functionality to resolve placeholders like {{variable_name}}
//! in action parameters using values provided at runtime.

use crate::action::types::{Action, ParamValue, PlaceholderString};
use std::collections::HashMap;

/// Action resolver - resolves placeholders in action parameters
pub struct ActionResolver {
    params: HashMap<String, ParamValue>,
}

impl ActionResolver {
    /// Create a new ActionResolver with the given parameters
    pub fn new(params: HashMap<String, ParamValue>) -> Self {
        Self { params }
    }

    /// Check if there are any parameters to resolve
    pub fn has_params(&self) -> bool {
        !self.params.is_empty()
    }

    /// Extract placeholder names needed by an action (without checking if they exist)
    pub fn extract_needed_placeholders(action: &Action) -> Vec<String> {
        match action {
            Action::MouseMove(m) => PlaceholderString::new(&m.x)
                .extract_placeholders()
                .into_iter()
                .chain(PlaceholderString::new(&m.y).extract_placeholders())
                .collect(),
            Action::MouseClick(m) => PlaceholderString::new(&m.count).extract_placeholders(),
            Action::Text(t) => PlaceholderString::new(&t.content).extract_placeholders(),
            Action::Key(k) => PlaceholderString::new(&k.key).extract_placeholders(),
            Action::KeySequence(seq) => seq
                .keys
                .iter()
                .flat_map(|k| PlaceholderString::new(&k.key).extract_placeholders())
                .collect(),
            _ => vec![],
        }
    }

    /// Resolve placeholders in an action
    pub fn resolve_action(&self, action: &Action) -> Result<Action, String> {
        match action {
            Action::MouseMove(m) => {
                let x = self.resolve_string(&m.x)?;
                let y = self.resolve_string(&m.y)?;
                Ok(Action::MouseMove(crate::action::types::MouseMoveAction {
                    x,
                    y,
                    duration_ms: m.duration_ms,
                    backend: m.backend.clone(),
                }))
            }
            Action::MouseClick(m) => {
                let count = self.resolve_string(&m.count)?;
                Ok(Action::MouseClick(crate::action::types::MouseClickAction {
                    button: m.button.clone(),
                    count,
                    interval_ms: m.interval_ms,
                    hold_time_ms: m.hold_time_ms,
                    backend: m.backend.clone(),
                }))
            }
            Action::Text(t) => {
                let content = self.resolve_string(&t.content)?;
                Ok(Action::Text(crate::action::types::TextAction {
                    content,
                    backend: t.backend.clone(),
                }))
            }
            Action::Key(k) => {
                let key = self.resolve_string(&k.key)?;
                Ok(Action::Key(crate::action::types::KeyAction {
                    key,
                    hold_time_ms: k.hold_time_ms,
                    backend: k.backend.clone(),
                }))
            }
            // Other action types don't have placeholder fields
            other => Ok(other.clone()),
        }
    }

    /// Resolve a string field that may contain placeholders
    fn resolve_string(&self, value: &str) -> Result<String, String> {
        let ph = PlaceholderString::new(value);
        if ph.has_placeholder() {
            ph.resolve(&self.params)
        } else {
            Ok(value.to_string())
        }
    }
}
