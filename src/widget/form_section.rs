use crate::widget::{Widget, WidgetList};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FormSection {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub widgets: Vec<Widget>,
}
impl FormSection {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

pub fn form_section(title: impl Into<String>, widgets: impl Into<WidgetList>) -> FormSection {
    FormSection {
        id: String::new(),
        title: title.into(),
        widgets: widgets.into().to_vec(),
    }
}
