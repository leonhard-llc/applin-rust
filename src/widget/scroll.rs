use crate::widget::Widget;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Scroll {
    pub widget: Box<Widget>,
}

pub fn scroll(widget: impl Into<Widget>) -> Scroll {
    Scroll {
        widget: Box::new(widget.into()),
    }
}
