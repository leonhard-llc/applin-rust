use crate::util::csprng_random_positive_nonzero_i64;
use crate::util::escape_and_elide;
use core::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// TODONT: Do not derive `Hash`, `Ord`, or `PartialOrd`.  They would let
//         data structure operations leak `secret` via timing.
#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub struct Secret(i64);
impl Secret {
    #[must_use]
    pub fn new_random() -> Self {
        Self(csprng_random_positive_nonzero_i64())
    }

    #[must_use]
    pub fn inner(self) -> i64 {
        self.0
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}
impl TryFrom<i64> for Secret {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 1 {
            return Err(format!("invalid Secret: {value}"));
        }
        Ok(Self(value))
    }
}
impl TryFrom<&str> for Secret {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let err = || format!("invalid Secret: {}", escape_and_elide(s.as_bytes(), 20));
        let value: i64 = s.parse().map_err(|_e| err())?;
        if value < 1 {
            return Err(err());
        }
        Ok(Self(value))
    }
}
impl FromStr for Secret {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Secret(...)")
    }
}
