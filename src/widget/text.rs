use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Text {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default)]
    pub text: String,
}
impl Text {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

pub fn text(s: impl Into<String>) -> Text {
    Text {
        id: String::new(),
        text: s.into(),
    }
}
