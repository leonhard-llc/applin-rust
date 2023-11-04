use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Text {
    pub text: String,
}

pub fn text(s: impl Into<String>) -> Text {
    Text { text: s.into() }
}
