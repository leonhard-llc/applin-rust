use serde::{Deserialize, Serialize};
use crate::Action;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Checkbox {
    pub actions: Vec<Action>,
    pub var_name: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub initial_bool: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpc: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
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
            var_name,
            initial_bool: false,
            rpc: None,
            text: String::new(),
        }
    }

    #[must_use]
    pub fn with_text(mut self, label: impl Into<String>) -> Self {
        self.text = label.into();
        self
    }

    #[must_use]
    pub fn with_rpc(mut self, rpc: impl Into<String>) -> Self {
        self.rpc = Some(rpc.into());
        self
    }

    #[must_use]
    pub fn with_initial_bool(mut self, checked: bool) -> Self {
        self.initial_bool = checked;
        self
    }
}

pub fn checkbox(var_name: impl Into<String>) -> Checkbox {
    Checkbox::new(var_name)
}
