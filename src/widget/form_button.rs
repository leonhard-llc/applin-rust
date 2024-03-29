use crate::action::Action;
use crate::option::HAlignment;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FormButton {
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub align: HAlignment,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_destructive: bool,
    #[serde(default)]
    pub text: String,
}

impl FormButton {
    #[must_use]
    pub fn new(text: impl Into<String>, actions: impl Into<Vec<Action>>) -> Self {
        Self {
            actions: actions.into(),
            align: HAlignment::default(),
            id: String::new(),
            is_destructive: bool::default(),
            text: text.into(),
        }
    }

    /// Appends `action`.
    #[must_use]
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Appends `actions`.
    #[must_use]
    pub fn with_actions(mut self, actions: impl IntoIterator<Item = Action>) -> Self {
        self.actions.extend(actions);
        self
    }

    #[must_use]
    pub fn with_align(mut self, x: HAlignment) -> Self {
        self.align = x;
        self
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_is_destructive(mut self, value: bool) -> Self {
        self.is_destructive = value;
        self
    }
}

pub fn form_button(text: impl Into<String>, actions: impl Into<Vec<Action>>) -> FormButton {
    FormButton::new(text, actions)
}
