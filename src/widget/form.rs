use crate::widget::{Widget, WidgetList};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Form {
    #[serde(default)]
    pub widgets: Vec<Widget>,
}

impl Form {
    /// Makes a `form` widget.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(widgets: impl Into<WidgetList>) -> Self {
        Self {
            widgets: widgets.into().to_vec(),
        }
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
pub fn form(widgets: impl Into<WidgetList>) -> Form {
    Form::new(widgets)
}
