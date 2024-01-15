use crate::option::{Allow, AutoCapitalize};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Textfield {
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub allow: Allow,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_capitalize: Option<AutoCapitalize>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub initial_string: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub max_chars: u32,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub max_lines: u32,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub min_chars: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_delay_ms: Option<u32>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub validated: bool,
    pub var_name: String,
}

impl Textfield {
    /// # Panics
    /// Panics when `var_name` is empty.
    #[must_use]
    pub fn new(var_name: impl Into<String>) -> Self {
        let var_name = var_name.into();
        assert!(!var_name.is_empty());
        Self {
            allow: Allow::All,
            auto_capitalize: None,
            error: String::new(),
            id: String::new(),
            initial_string: String::new(),
            label: String::new(),
            max_chars: u32::MAX,
            max_lines: u32::MAX,
            min_chars: 0,
            poll_delay_ms: None,
            validated: false,
            var_name,
        }
    }

    #[must_use]
    pub fn with_allow(mut self, x: Allow) -> Self {
        self.allow = x;
        self
    }

    #[must_use]
    pub fn with_auto_capitalize(mut self, x: AutoCapitalize) -> Self {
        self.auto_capitalize = Some(x);
        self
    }

    #[must_use]
    pub fn with_error(mut self, x: impl Into<String>) -> Self {
        self.error = x.into();
        self
    }

    #[must_use]
    pub fn with_opt_error(mut self, x: Option<impl Into<String>>) -> Self {
        self.error = x.map_or(String::new(), Into::into);
        self
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_initial_string(mut self, x: impl Into<String>) -> Self {
        self.initial_string = x.into();
        self
    }

    #[must_use]
    pub fn with_opt_initial(mut self, opt_initial: Option<impl Into<String>>) -> Self {
        self.initial_string = opt_initial.map_or(String::new(), Into::into);
        self
    }

    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Ask the client to prevent the user from entering too many characters.
    /// Use `0` for no limit.
    #[must_use]
    pub fn with_max_chars(mut self, x: u32) -> Self {
        self.max_chars = x;
        self
    }

    /// Ask the client to prevent the user from entering too many newline characters.
    /// Use `0` for no limit.
    #[must_use]
    pub fn with_max_lines(mut self, x: u32) -> Self {
        self.max_lines = x;
        self
    }

    /// Show a warning when the user has not entered enough characters.
    /// Use 0 for no minimum.
    #[must_use]
    pub fn with_min_chars(mut self, x: u32) -> Self {
        self.min_chars = x;
        self
    }

    /// Poll the page after the field is updated and `duration` has passed.
    #[must_use]
    pub fn with_poll_delay(mut self, duration: Duration) -> Self {
        self.poll_delay_ms = Some(duration.as_millis().try_into().unwrap_or(u32::MAX));
        self
    }

    #[must_use]
    pub fn with_validated(mut self) -> Self {
        self.validated = true;
        self
    }
}

/// # Panics
/// Panics when `var_name` is empty.
#[must_use]
pub fn textfield(var_name: impl Into<String>) -> Textfield {
    Textfield::new(var_name)
}
