use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Add actions to button widgets and other widgets
/// to let the user initiate actions in the app.
///
/// When the user taps a widget, the frontend performs the action list.
/// It performs the actions in order.
/// If it encounters an error while performing an action, it stops.
///
/// For example, when performing the action list `["rpc:/save", "pop"]`,
/// if the RPC to `/save` returns an error, the frontend will not pop the page.
///
/// Do you need an action that's not here?
/// Please let us know: <https://www.applin.dev/docs/feature-requests.html>.
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
    Action(format!("launch_url:{url}"))
}

#[must_use]
pub fn logout() -> Action {
    Action("logout".into())
}

#[must_use]
pub fn on_user_error_poll() -> Action {
    Action("on_user_error_poll".into())
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
/// button("Submit", [poll(), rpc("/form_submit")])
/// # }
/// ```
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
