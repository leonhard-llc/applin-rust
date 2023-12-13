use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Empty;

#[must_use]
pub fn empty() -> Empty {
    Empty {}
}
