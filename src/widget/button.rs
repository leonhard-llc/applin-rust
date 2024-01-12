use crate::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Button {
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default)]
    pub text: String,
}
impl Button {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

pub fn button(text: impl Into<String>, actions: impl Into<Vec<Action>>) -> Button {
    Button {
        actions: actions.into(),
        id: String::new(),
        text: text.into(),
    }
}
