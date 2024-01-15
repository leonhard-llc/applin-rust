use crate::action::Action;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Checkbox {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub initial_bool: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_delay_ms: Option<u32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub validated: bool,
    pub var_name: String,
}

impl Checkbox {
    /// # Panics
    /// Panics when `var_name` is empty.
    #[must_use]
    pub fn new(var_name: impl Into<String>) -> Self {
        let var_name = var_name.into();
        assert!(!var_name.is_empty());
        Self {
            actions: Vec::new(),
            id: String::new(),
            initial_bool: false,
            poll_delay_ms: None,
            text: String::new(),
            validated: false,
            var_name,
        }
    }

    /// Appends `actions`.
    #[must_use]
    pub fn with_actions(mut self, actions: impl IntoIterator<Item = Action>) -> Self {
        self.actions.extend(actions);
        self
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_initial_bool(mut self, checked: bool) -> Self {
        self.initial_bool = checked;
        self
    }

    /// Poll the page after the field is updated and `duration` has passed.
    #[must_use]
    pub fn with_poll_delay(mut self, duration: Duration) -> Self {
        self.poll_delay_ms = Some(duration.as_millis().try_into().unwrap_or(u32::MAX));
        self
    }

    #[must_use]
    pub fn with_text(mut self, label: impl Into<String>) -> Self {
        self.text = label.into();
        self
    }

    #[must_use]
    pub fn with_validated(mut self) -> Self {
        self.validated = true;
        self
    }
}

pub fn checkbox(var_name: impl Into<String>) -> Checkbox {
    Checkbox::new(var_name)
}
