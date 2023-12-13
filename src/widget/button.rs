use crate::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Button {
    pub actions: Vec<Action>,
    pub text: String,
}

pub fn button(text: impl Into<String>, actions: impl Into<Vec<Action>>) -> Button {
    Button {
        text: text.into(),
        actions: actions.into(),
    }
}
