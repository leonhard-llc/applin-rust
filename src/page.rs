#![allow(clippy::module_name_repetitions)]
use crate::widget::{empty, BackButton, Widget};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "typ")]
pub enum Page {
    #[serde(rename = "nav_page")]
    Nav(NavPage),
    #[serde(rename = "plain_page")]
    Plain(PlainPage),
}

impl From<NavPage> for Page {
    fn from(value: NavPage) -> Self {
        Page::Nav(value)
    }
}

impl From<PlainPage> for Page {
    fn from(value: PlainPage) -> Self {
        Page::Plain(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NavPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Widget>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ephemeral: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub poll_seconds: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<Widget>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub stream: bool,
    #[serde(default)]
    pub title: String,
    pub widget: Widget,
}

impl NavPage {
    #[must_use]
    pub fn new(title: impl Into<String>, widget: impl Into<Widget>) -> Self {
        Self {
            end: None,
            ephemeral: false,
            id: String::new(),
            poll_seconds: 0,
            start: None,
            stream: false,
            title: title.into(),
            widget: widget.into(),
        }
    }

    #[must_use]
    pub fn without_back(mut self) -> Self {
        self.start = Some(empty().into());
        self
    }

    #[must_use]
    pub fn with_end(mut self, widget: impl Into<Widget>) -> Self {
        self.end = Some(widget.into());
        self
    }

    #[must_use]
    pub fn with_ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_poll(mut self, seconds: u32) -> Self {
        self.poll_seconds = seconds;
        self
    }

    #[must_use]
    pub fn with_start(mut self, back_button: BackButton) -> Self {
        self.start = Some(back_button.into());
        self
    }

    #[must_use]
    pub fn with_empty_start(mut self) -> Self {
        self.start = Some(empty().into());
        self
    }

    #[must_use]
    pub fn with_stream(mut self) -> Self {
        self.stream = true;
        self
    }
}

pub fn nav_page(title: impl Into<String>, widget: impl Into<Widget>) -> NavPage {
    NavPage::new(title, widget)
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PlainPage {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ephemeral: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub poll_seconds: u32,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub stream: bool,
    #[serde(default)]
    pub title: String,
    pub widget: Widget,
}

impl PlainPage {
    #[must_use]
    pub fn new(title: impl Into<String>, widget: impl Into<Widget>) -> Self {
        Self {
            ephemeral: false,
            id: String::new(),
            poll_seconds: 0,
            stream: false,
            title: title.into(),
            widget: widget.into(),
        }
    }

    #[must_use]
    pub fn with_ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_poll(mut self, seconds: u32) -> Self {
        self.poll_seconds = seconds;
        self
    }

    #[must_use]
    pub fn with_stream(mut self) -> Self {
        self.stream = true;
        self
    }
}

pub fn plain_page(title: impl Into<String>, widget: impl Into<Widget>) -> PlainPage {
    PlainPage::new(title, widget)
}
