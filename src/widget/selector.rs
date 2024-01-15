use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Selector {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub initial_string: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub initial_string1: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub initial_string2: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options1: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options2: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_delay_ms: Option<u32>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub validated: bool,
    pub var_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub var_name1: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub var_name2: String,
}

impl Selector {
    /// # Panics
    /// Panics when `var_name` is empty.
    #[must_use]
    pub fn new(
        var_name: impl Into<String>,
        options: impl Into<Vec<String>>,
        opt_initial: Option<impl Into<String>>,
    ) -> Self {
        let var_name = var_name.into();
        assert!(!var_name.is_empty());
        Self {
            error: String::new(),
            id: String::new(),
            initial_string: opt_initial.map_or(String::new(), Into::into),
            initial_string1: String::new(),
            initial_string2: String::new(),
            label: String::new(),
            options: options.into(),
            options1: vec![],
            options2: vec![],
            poll_delay_ms: None,
            validated: false,
            var_name,
            var_name1: String::new(),
            var_name2: String::new(),
        }
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_opt_error(mut self, x: Option<impl Into<String>>) -> Self {
        self.error = x.map_or(String::new(), Into::into);
        self
    }

    #[must_use]
    pub fn with_options1(
        mut self,
        var_name1: impl Into<String>,
        options1: impl Into<Vec<String>>,
        opt_initial1: Option<impl Into<String>>,
    ) -> Self {
        self.initial_string1 = opt_initial1.map_or(String::new(), Into::into);
        self.options1 = options1.into();
        self.var_name1 = var_name1.into();
        self
    }

    #[must_use]
    pub fn with_options2(
        mut self,
        var_name2: impl Into<String>,
        options2: impl Into<Vec<String>>,
        opt_initial2: Option<impl Into<String>>,
    ) -> Self {
        self.initial_string2 = opt_initial2.map_or(String::new(), Into::into);
        self.options2 = options2.into();
        self.var_name2 = var_name2.into();
        self
    }

    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
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
pub fn selector(
    var_name: impl Into<String>,
    options: impl Into<Vec<String>>,
    opt_initial: Option<impl Into<String>>,
) -> Selector {
    Selector::new(var_name, options, opt_initial)
}
