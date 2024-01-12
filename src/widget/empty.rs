use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Empty {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
}
impl Empty {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

#[must_use]
pub fn empty() -> Empty {
    Empty { id: String::new() }
}
