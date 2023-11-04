use serde::{Deserialize, Serialize};
use crate::{Disposition, Real32};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Image {
    pub aspect_ratio: Real32,
    pub disposition: Disposition,
    pub url: String,
}

/// # Panics
/// Panics when `aspect_ratio` is infinite, zero, negative, or NaN.
#[must_use]
pub fn image(disposition: Disposition, aspect_ratio_width_over_height: f32, url: impl Into<String>) -> Image {
    let aspect_ratio = Real32::new(aspect_ratio_width_over_height);
    let url = url.into();
    Image { aspect_ratio, disposition, url }
}
