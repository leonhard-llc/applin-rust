use crate::util::csprng_random_positive_nonzero_i64;
use crate::util::escape_and_elide;
use core::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Id(i64);
impl Id {
    #[must_use]
    pub fn new_random() -> Self {
        Self(csprng_random_positive_nonzero_i64())
    }

    #[must_use]
    pub fn inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for Id {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 1 {
            return Err(format!("invalid Id: {value}"));
        }
        Ok(Self(value))
    }
}
impl TryFrom<&str> for Id {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let err = || format!("invalid Id: {}", escape_and_elide(s.as_bytes(), 20));
        let value: i64 = s.parse().map_err(|_e| err())?;
        if value < 1 {
            return Err(err());
        }
        Ok(Self(value))
    }
}
impl FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Id({})", self.0)
    }
}
