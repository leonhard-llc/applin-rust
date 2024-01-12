use crate::option::Disposition;
use crate::util::Real32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Image {
    pub aspect_ratio: Real32,
    pub disposition: Disposition,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    pub url: String,
}
impl Image {
    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
    }
}

/// # Panics
/// Panics when `aspect_ratio` is infinite, zero, negative, or NaN.
#[must_use]
pub fn image(
    disposition: Disposition,
    aspect_ratio_width_over_height: f32,
    url: impl Into<String>,
) -> Image {
    let aspect_ratio = Real32::new(aspect_ratio_width_over_height);
    let url = url.into();
    Image {
        aspect_ratio,
        disposition,
        id: String::new(),
        url,
    }
}
