use crate::widget::Widget;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Scroll {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    pub widget: Box<Widget>,
}
impl Scroll {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

pub fn scroll(widget: impl Into<Widget>) -> Scroll {
    Scroll {
        id: String::new(),
        widget: Box::new(widget.into()),
    }
}
