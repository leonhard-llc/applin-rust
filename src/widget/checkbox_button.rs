use crate::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CheckboxButton {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub initial_bool: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
}

impl CheckboxButton {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_text(mut self, label: impl Into<String>) -> Self {
        self.text = label.into();
        self
    }
}

pub fn checkbox_button(initial_bool: bool, actions: impl Into<Vec<Action>>) -> CheckboxButton {
    CheckboxButton {
        actions: actions.into(),
        initial_bool,
        id: String::new(),
        text: String::new(),
    }
}
