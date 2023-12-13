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
//! # Cargo Geiger Safety Report
//! # Changelog
//! - v0.1.4 - Add `From<Option<Into<Widget>>>` for `Widget` for use with `error_text`.
//! - v0.1.3 - Add `on_user_error_poll` action.
//! - v0.1.2 - Bugfixes
//! - v0.1.1 - Update documentation.
//! - v0.1.0 - First published version
#![forbid(unsafe_code)]

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub use action::*;
pub use back_button::*;
pub use button::*;
pub use checkbox::*;
pub use column::*;
pub use empty::*;
pub use error_text::*;
pub use form::*;
pub use form_button::*;
pub use form_section::*;
pub use grouped_row_table::*;
pub use image::*;
pub use last_error_text::*;
pub use nav_button::*;
pub use page::*;
pub use real32::*;
pub use scroll::*;
pub use text::*;
pub use textfield::*;

mod action;
mod back_button;
mod button;
mod checkbox;
mod column;
mod empty;
mod error_text;
mod form;
mod form_button;
mod form_section;
mod grouped_row_table;
mod image;
mod last_error_text;
mod nav_button;
pub mod opt_widget_list;
mod page;
mod real32;
pub mod row_group_list;
pub mod row_list;
mod scroll;
mod text;
mod textfield;
pub mod widget;
pub mod widget_list;

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
pub fn client_error(msg: impl Into<String>) -> servlin::Response {
    servlin::Response::text(400, msg.into())
}

#[cfg(feature = "servlin")]
pub fn user_error(msg: impl Into<String>) -> servlin::Response {
    servlin::Response::unprocessable_entity_422(msg)
}

pub(crate) fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Allow {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "ascii")]
    Ascii,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "numbers")]
    Numbers,
    #[serde(rename = "tel")]
    Tel,
}

impl Default for Allow {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AutoCapitalize {
    #[serde(rename = "names")]
    Names,
    #[serde(rename = "sentences")]
    Sentences,
}

impl Default for AutoCapitalize {
    fn default() -> Self {
        Self::Sentences
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum HAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "end")]
    End,
}

impl HAlignment {
    /// This is useful for serde `skip_serializing_if` attributes.
    #[must_use]
    pub fn is_start(&self) -> bool {
        self == &Self::Start
    }
}

impl Default for HAlignment {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Disposition {
    /// Size the image to fill the space, preserving aspect ratio.
    #[serde(rename = "cover")]
    Cover,
    /// Size the image to just fit inside the space, preserving aspect ratio.
    #[serde(rename = "fit")]
    Fit,
    /// Stretch the image to fill the space.
    #[serde(rename = "stretch")]
    Stretch,
}
