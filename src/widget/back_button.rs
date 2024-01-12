use crate::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BackButton {
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
}
impl BackButton {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

pub fn back_button(actions: impl Into<Vec<Action>>) -> BackButton {
    BackButton {
        actions: actions.into(),
        id: String::new(),
    }
}
