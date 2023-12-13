use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ErrorText {
    pub text: String,
}

/// The frontend replaces `${INTERACTIVE_ERROR_DETAILS}` with the text of the last error.
pub fn error_text(s: impl Into<String>) -> ErrorText {
    ErrorText { text: s.into() }
}
