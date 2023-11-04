use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LastErrorText;

pub fn last_error_text() -> LastErrorText {
    LastErrorText {}
}
