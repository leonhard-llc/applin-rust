//! applin-rust
//! ======
//! [![crates.io version](https://img.shields.io/crates/v/applin.svg)](https://crates.io/crates/applin)
//! [![unsafe forbidden](https://raw.githubusercontent.com/leonhard-llc/applin-rust/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://github.com/leonhard-llc/applin-rust/workflows/CI/badge.svg)](https://github.com/leonhard-llc/applin-rust/actions)
//!
//! You can use the Applin™ Server-Driven UI framework to build a mobile app
//! with only server-side code.
//!
//! <https://www.applin.dev/>
//!
//! Use a provided boilerplate app for the frontend.
//! Use this library for the backend.
//!
//! Documentation: <https://www.applin.dev/docs/>
//!
//! Example and live demo: <https://github.com/leonhard-llc/applin-rust-demo>
//!
//! Related crate: [applin_headless](https://crates.io/crates/applin_headless)
//!
//! # Cargo Geiger Safety Report
//! # Changelog
//! - v0.2.9 2024-10-26
//!     - Add `ModalButton::new` and `ModalButton::with_actions`.
//!     - Upgrade to `servlin` v0.6.
//! - v0.2.8 - Make debug formatting more concise by using JSON, for better test failure messages.
//! - v0.2.7
//!     - Support `ApplinIos` 0.38.0.
//!     - Add `checkbox_button`.
//! - v0.2.6 - Add `RowList::push`, `RowList::new`, `OptWidgetList::push`, and `OptWidgetList::new`.
//! - v0.2.5 - Add `WidgetList::push` and `WidgetList::new`.
//! - v0.2.4
//!     - Support `ApplinIos` 0.36.0.
//!     - Add `reset_var` and `stop_actions` actions.
//! - v0.2.3 - Add `id` field to `Action` and `ModalButton`, for testing.
//! - v0.2.2 - Add `with_validated` to input widgets. Supports `ApplinIos` 0.33.0.
//! - v0.2.1
//!     - Add `id` fields for testing.
//!     - Make `Action` fields public.
//!     - Make `Real32` and enums `Copy`.
//! - v0.2.0
//!     - Support `ApplinIos` 0.32.0.
//!     - Remove `on_user_error_poll` action and make it a parameter of the `rpc` action.
//! - v0.1.7
//!     - Support `ApplinIos` 0.31.0.
//!     - Add `modal` action.
//!     - Add `aspect_ratio` to `choose_photo` and `take_photo` actions.
//! - v0.1.6
//!     - Support `ApplinIos` 0.28.0.
//!     - Add `logout` action.
//!     - Add `selector` widget.
//! - v0.1.5
//!     - Support `ApplinIos` 0.25.0.
//!     - Add `poll_delay_ms` to checkbox and textfield.
//!     - Replace checkbox `rpc` field with `actions`.
//! - v0.1.4
//!     - Add `From<Option<Into<Widget>>>` for `Widget` for use with `error_text`.
//!     - Add `SessionCookie`, `Id`, and `Secret`.
//!     - Organize into modules for easier discovery.
//! - v0.1.3 - Add `on_user_error_poll` action.
//! - v0.1.2 - Bugfixes
//! - v0.1.1 - Update documentation.
//! - v0.1.0 - First published version
#![forbid(unsafe_code)]

use crate::page::Page;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod action;
pub mod option;
pub mod page;
pub mod session;
pub mod util;
pub mod widget;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApplinResponse {
    pub page: Page,
}

/// # Errors
/// Returns an error when it fails to convert `page` to JSON.
#[cfg(feature = "servlin")]
pub fn applin_response(page: impl Into<Page>) -> Result<servlin::Response, servlin::Error> {
    let response = ApplinResponse { page: page.into() };
    Ok(servlin::Response::json(200, response)?
        .with_type(servlin::ContentType::Str("application/vnd.applin_response")))
}

#[cfg(feature = "servlin")]
pub fn client_error(msg: impl Into<String>) -> servlin::Error {
    servlin::Error::client_error(servlin::Response::text(400, msg.into()))
}

#[cfg(feature = "servlin")]
pub fn user_error(msg: impl Into<String>) -> servlin::Error {
    servlin::Error::client_error(servlin::Response::unprocessable_entity_422(msg))
}

pub(crate) fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}
