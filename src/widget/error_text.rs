use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ErrorText {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default)]
    pub text: String,
}
impl ErrorText {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

/// The frontend replaces `${INTERACTIVE_ERROR_DETAILS}` with the text of the last error.
pub fn error_text(s: impl Into<String>) -> ErrorText {
    ErrorText {
        id: String::new(),
        text: s.into(),
    }
}
