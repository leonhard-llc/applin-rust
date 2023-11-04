use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Action(pub String);

#[must_use]
pub fn choose_photo(upload_url: impl Display) -> Action {
    Action(format!("choose_photo:{upload_url}"))
}

#[must_use]
pub fn copy_to_clipboard(text: impl Display) -> Action {
    Action(format!("copy_to_clipboard:{text}"))
}

#[must_use]
pub fn launch_url(url: impl Display) -> Action {
    Action(format!("launch_url:{url}"))
}

#[must_use]
pub fn logout() -> Action {
    Action("logout".into())
}

#[must_use]
pub fn nothing() -> Action {
    Action("nothing".into())
}

#[must_use]
pub fn poll() -> Action {
    Action("poll".into())
}

#[must_use]
pub fn pop() -> Action {
    Action("pop".into())
}

#[must_use]
pub fn push(page_key: impl Display) -> Action {
    Action(format!("push:{page_key}"))
}

#[must_use]
pub fn replace_all(page_key: impl Display) -> Action {
    Action(format!("replace_all:{page_key}"))
}

#[must_use]
pub fn rpc(url: impl Display) -> Action {
    Action(format!("rpc:{url}"))
}

#[must_use]
pub fn take_photo(upload_url: impl Display) -> Action {
    Action(format!("take_photo:{upload_url}"))
}
