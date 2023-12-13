use crate::widget::{
    BackButton, Button, Checkbox, Column, Empty, ErrorText, Form, FormButton, FormSection,
    GroupedRowTable, Image, LastErrorText, NavButton, Scroll, Text, Textfield,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "typ")]
pub enum Widget {
    #[serde(rename = "back_button")]
    BackButton(BackButton),
    #[serde(rename = "button")]
    Button(Button),
    #[serde(rename = "checkbox")]
    Checkbox(Checkbox),
    #[serde(rename = "column")]
    Column(Column),
    #[serde(rename = "empty")]
    Empty(Empty),
    #[serde(rename = "error_text")]
    ErrorText(ErrorText),
    #[serde(rename = "form")]
    Form(Form),
    #[serde(rename = "form_button")]
    FormButton(FormButton),
    #[serde(rename = "form_section")]
    FormSection(FormSection),
    #[serde(rename = "grouped_row_table")]
    GroupedRowTable(GroupedRowTable),
    #[serde(rename = "image")]
    Image(Image),
    #[serde(rename = "last_error_text")]
    LastErrorText(LastErrorText),
    #[serde(rename = "nav_button")]
    NavButton(NavButton),
    #[serde(rename = "scroll")]
    Scroll(Scroll),
    #[serde(rename = "text")]
    Text(Text),
    #[serde(rename = "textfield")]
    Textfield(Textfield),
}

impl<A: Into<Widget>> From<Option<A>> for Widget {
    fn from(value: Option<A>) -> Self {
        match value {
            Some(widget) => widget.into(),
            None => Widget::Empty(Empty {}),
        }
    }
}

impl From<BackButton> for Widget {
    fn from(src: BackButton) -> Self {
        Widget::BackButton(src)
    }
}

impl From<Button> for Widget {
    fn from(src: Button) -> Self {
        Widget::Button(src)
    }
}

impl From<Checkbox> for Widget {
    fn from(src: Checkbox) -> Self {
        Widget::Checkbox(src)
    }
}

impl From<Column> for Widget {
    fn from(src: Column) -> Self {
        Widget::Column(src)
    }
}

impl From<Empty> for Widget {
    fn from(src: Empty) -> Self {
        Widget::Empty(src)
    }
}

impl From<ErrorText> for Widget {
    fn from(src: ErrorText) -> Self {
        Widget::ErrorText(src)
    }
}

impl From<Form> for Widget {
    fn from(src: Form) -> Self {
        Widget::Form(src)
    }
}

impl From<FormButton> for Widget {
    fn from(src: FormButton) -> Self {
        Widget::FormButton(src)
    }
}

impl From<FormSection> for Widget {
    fn from(src: FormSection) -> Self {
        Widget::FormSection(src)
    }
}

impl From<GroupedRowTable> for Widget {
    fn from(src: GroupedRowTable) -> Self {
        Widget::GroupedRowTable(src)
    }
}

impl From<Image> for Widget {
    fn from(src: Image) -> Self {
        Widget::Image(src)
    }
}

impl From<LastErrorText> for Widget {
    fn from(src: LastErrorText) -> Self {
        Widget::LastErrorText(src)
    }
}

impl From<NavButton> for Widget {
    fn from(src: NavButton) -> Self {
        Widget::NavButton(src)
    }
}

impl From<Scroll> for Widget {
    fn from(src: Scroll) -> Self {
        Widget::Scroll(src)
    }
}

impl From<Text> for Widget {
    fn from(src: Text) -> Self {
        Widget::Text(src)
    }
}

impl From<Textfield> for Widget {
    fn from(src: Textfield) -> Self {
        Widget::Textfield(src)
    }
}

impl From<BackButton> for Option<Widget> {
    fn from(src: BackButton) -> Self {
        Some(Widget::BackButton(src))
    }
}

impl From<Button> for Option<Widget> {
    fn from(src: Button) -> Self {
        Some(Widget::Button(src))
    }
}

impl From<Checkbox> for Option<Widget> {
    fn from(src: Checkbox) -> Self {
        Some(Widget::Checkbox(src))
    }
}

impl From<Column> for Option<Widget> {
    fn from(src: Column) -> Self {
        Some(Widget::Column(src))
    }
}

impl From<Empty> for Option<Widget> {
    fn from(src: Empty) -> Self {
        Some(Widget::Empty(src))
    }
}

impl From<ErrorText> for Option<Widget> {
    fn from(src: ErrorText) -> Self {
        Some(Widget::ErrorText(src))
    }
}

impl From<Form> for Option<Widget> {
    fn from(src: Form) -> Self {
        Some(Widget::Form(src))
    }
}

impl From<FormButton> for Option<Widget> {
    fn from(src: FormButton) -> Self {
        Some(Widget::FormButton(src))
    }
}

impl From<FormSection> for Option<Widget> {
    fn from(src: FormSection) -> Self {
        Some(Widget::FormSection(src))
    }
}

impl From<GroupedRowTable> for Option<Widget> {
    fn from(src: GroupedRowTable) -> Self {
        Some(Widget::GroupedRowTable(src))
    }
}

impl From<Image> for Option<Widget> {
    fn from(src: Image) -> Self {
        Some(Widget::Image(src))
    }
}

impl From<LastErrorText> for Option<Widget> {
    fn from(src: LastErrorText) -> Self {
        Some(Widget::LastErrorText(src))
    }
}

impl From<NavButton> for Option<Widget> {
    fn from(src: NavButton) -> Self {
        Some(Widget::NavButton(src))
    }
}

impl From<Scroll> for Option<Widget> {
    fn from(src: Scroll) -> Self {
        Some(Widget::Scroll(src))
    }
}

impl From<Text> for Option<Widget> {
    fn from(src: Text) -> Self {
        Some(Widget::Text(src))
    }
}

impl From<Textfield> for Option<Widget> {
    fn from(src: Textfield) -> Self {
        Some(Widget::Textfield(src))
    }
}
