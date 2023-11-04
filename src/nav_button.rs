use serde::{Deserialize, Serialize};

use crate::Action;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NavButton {
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_text: Option<String>,
    #[serde(default)]
    pub text: String,
}

impl NavButton {
    #[must_use]
    pub fn new(text: impl Into<String>, actions: impl Into<Vec<Action>>) -> Self {
        Self {
            actions: actions.into(),
            badge_text: None,
            photo_url: None,
            sub_text: None,
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
        self.actions.extend(actions.into_iter());
        self
    }

    #[must_use]
    pub fn with_badge_text(mut self, badge: impl Into<String>) -> Self {
        self.badge_text = Some(badge.into());
        self
    }

    #[must_use]
    pub fn with_photo_url(mut self, url: impl Into<String>) -> Self {
        self.photo_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn with_sub_text(mut self, sub_text: impl Into<String>) -> Self {
        self.sub_text = Some(sub_text.into());
        self
    }
}

#[must_use]
pub fn nav_button(text: impl Into<String>, actions: impl Into<Vec<Action>>) -> NavButton {
    NavButton::new(text, actions)
}
