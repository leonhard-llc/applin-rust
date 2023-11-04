use serde::{Deserialize, Serialize};

use crate::Action;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BackButton {
    pub actions: Vec<Action>,
}

pub fn back_button(actions: impl Into<Vec<Action>>) -> BackButton {
    BackButton {
        actions: actions.into(),
    }
}
