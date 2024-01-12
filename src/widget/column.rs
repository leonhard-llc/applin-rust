use crate::option::HAlignment;
use crate::widget::{Widget, WidgetList};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Column {
    #[serde(default, skip_serializing_if = "HAlignment::is_start")]
    pub align: HAlignment,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub spacing: u16,
    #[serde(default)]
    pub widgets: Vec<Widget>,
}

impl Column {
    /// Makes a `column` widget with horizontal alignment `start` and spacing `0`.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(widgets: impl Into<WidgetList>) -> Self {
        Self {
            align: HAlignment::Start,
            id: String::new(),
            spacing: u16::default(),
            widgets: widgets.into().to_vec(),
        }
    }

    #[must_use]
    pub fn with_alignment(mut self, align: HAlignment) -> Self {
        self.align = align;
        self
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn with_spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Appends `widget`.
    #[must_use]
    pub fn with_widget(mut self, widget: impl Into<Widget>) -> Self {
        self.widgets.push(widget.into());
        self
    }

    /// Appends `widgets`.
    #[must_use]
    pub fn with_widgets(mut self, widgets: impl Into<WidgetList>) -> Self {
        self.widgets.extend(widgets.into().0);
        self
    }
}

#[must_use]
pub fn column(widgets: impl Into<WidgetList>) -> Column {
    Column::new(widgets)
}
