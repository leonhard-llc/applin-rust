use serde::{Deserialize, Serialize};

use crate::widget::Widget;
use crate::widget_list::WidgetList;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FormSection {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub widgets: Vec<Widget>,
}

pub fn form_section(title: impl Into<String>, widgets: impl Into<WidgetList>) -> FormSection {
    FormSection {
        title: title.into(),
        widgets: widgets.into().to_vec(),
    }
}
