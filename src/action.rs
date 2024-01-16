use crate::util::Real32;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ModalButton {
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    pub text: String,
}
impl ModalButton {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

pub fn modal_button(text: impl Into<String>, actions: impl Into<Vec<Action>>) -> ModalButton {
    ModalButton {
        actions: actions.into(),
        id: String::new(),
        text: text.into(),
    }
}

/// Add actions to button widgets and other widgets
/// to let the user initiate actions in the app.
///
/// When the user taps a widget, the frontend performs the action list.
/// It performs the actions in order.
/// If it encounters an error while performing an action, it stops.
///
/// For example, when performing the action list `[{"typ":"rpc","url":/save"}, {"typ":"pop"}]`,
/// if the RPC to `/save` returns an error, the frontend will not pop the page.
///
/// Do you need an action that's not here?
/// Please let us know: <https://www.applin.dev/docs/feature-requests.html>.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Action {
    pub typ: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub aspect_ratio: Option<Real32>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub buttons: Vec<ModalButton>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub message: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub on_user_error_poll: bool,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub page: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub string_value: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub title: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub url: String,
}
impl Action {
    pub fn new(typ: impl Display) -> Self {
        Action {
            typ: typ.to_string(),
            aspect_ratio: None,
            buttons: Vec::new(),
            id: String::new(),
            message: String::new(),
            on_user_error_poll: false,
            page: String::new(),
            string_value: String::new(),
            title: String::new(),
            url: String::new(),
        }
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

#[must_use]
pub fn choose_photo(upload_url: impl Display, aspect_ratio: Option<f32>) -> Action {
    let mut action = Action::new("choose_photo");
    action.aspect_ratio = aspect_ratio.map(Real32::new);
    action.url = upload_url.to_string();
    action
}

#[must_use]
pub fn copy_to_clipboard(text: impl Display) -> Action {
    let mut action = Action::new("copy_to_clipboard");
    action.string_value = text.to_string();
    action
}

/// Launch a URL on the user's device.
/// The URL can point to a webpage or another app.
///
/// Example URLs:
/// - `"https://www.example.com/support"`
/// - `"mailto:support@example.com?subject=Support&body=Error%20E123"`
///   (subject and body are
///   [percent-encoded](https://developer.mozilla.org/en-US/docs/Glossary/percent-encoding))
/// - `"tel:+12223334444`
/// - `"sms:+12223334444`
///
/// More info:
/// - <https://developer.apple.com/library/content/featuredarticles/iPhoneURLScheme_Reference/Introduction/Introduction.html>
/// - <https://developer.android.com/guide/components/intents-common.html>
///
/// Note: Simulator doesn't have email or phone apps so `mailto` and `tel` links don't work.
///
/// # Example
/// ```
/// use applin::widget::button;
/// use applin::action::launch_url;
/// # fn f() -> applin::widget::Button {
/// button("Support", [launch_url("https://www.example.com/support")])
/// # }
/// ```
#[must_use]
pub fn launch_url(url: impl Display) -> Action {
    let mut action = Action::new("launch_url");
    action.url = url.to_string();
    action
}

#[must_use]
pub fn logout() -> Action {
    Action::new("logout")
}

/// Displays a modal dialog box, aka an "alert".
///
/// Use the string "Cancel" for the cancel button text.
/// Add `!` to the start of the button text to give the button "destructive" style.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use applin::widget::button;
/// use applin::action::{modal, poll, rpc};
/// # fn f() -> applin::widget::Button {
/// button("Delete", [modal(
///     "Delete Item?",
///     None,
///     vec![
///         ("!Delete".into(), vec![rpc("/delete?id=123", false), poll()]),
///         ("Cancel".into(), vec![]),
///     ]
/// )])
/// # }
/// ```
#[allow(clippy::implicit_hasher)]
#[must_use]
pub fn modal(
    title: impl Into<String>,
    message: Option<String>,
    buttons: impl Into<Vec<ModalButton>>,
) -> Action {
    let mut action = Action::new("modal");
    action.title = title.into();
    action.message = message.map(Into::into).unwrap_or_default();
    action.buttons = buttons.into();
    action
}

/// Use `poll` to update the page based on page data or backend data.
///
/// To execute the action,
/// the frontend re-fetches the page from the server and
/// then smoothly updates the displayed page to match the new version it received.
///
/// It shows the "Working" modal.
///
/// If the page has any variables, the frontend uses an HTTP POST
/// and includes the variables and their values in a JSON object request body.
///
/// ## Example
/// ```
/// use applin::widget::button;
/// use applin::action::{poll, rpc};
/// # fn f() -> applin::widget::Button {
/// button("Submit", [poll(), rpc("/form_submit", false)])
/// # }
/// ```
#[must_use]
pub fn poll() -> Action {
    Action::new("poll")
}

#[must_use]
pub fn pop() -> Action {
    Action::new("pop")
}

#[must_use]
pub fn push(page_key: impl Display) -> Action {
    let mut action = Action::new("push");
    action.page = page_key.to_string();
    action
}

#[must_use]
pub fn replace_all(page_key: impl Display) -> Action {
    let mut action = Action::new("replace_all");
    action.page = page_key.to_string();
    action
}

#[must_use]
pub fn rpc(url: impl Display, on_user_error_poll: bool) -> Action {
    let mut action = Action::new("rpc");
    action.url = url.to_string();
    action.on_user_error_poll = on_user_error_poll;
    action
}

#[must_use]
pub fn take_photo(upload_url: impl Display, aspect_ratio: Option<f32>) -> Action {
    let mut action = Action::new("take_photo");
    action.aspect_ratio = aspect_ratio.map(Real32::new);
    action.url = upload_url.to_string();
    action
}
