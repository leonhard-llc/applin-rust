use serde::{Deserialize, Serialize};

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
