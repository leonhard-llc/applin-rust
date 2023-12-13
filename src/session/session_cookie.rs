use crate::session::{Id, Secret};
use crate::util::escape_and_elide;
use core::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[cfg(feature = "servlin")]
const SESSION_COOKIE_NAME: &str = "APPLIN_SESSION";

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub struct SessionCookie {
    pub id: Id,
    pub secret: Secret,
}
impl SessionCookie {
    #[must_use]
    pub fn new(id: Id, secret: Secret) -> Self {
        Self { id, secret }
    }

    /// Gets and parses the session cookie, if present.
    ///
    /// # Errors
    /// Returns an error when the cookie is present but malformed.
    #[cfg(feature = "servlin")]
    pub fn from_req(req: &servlin::Request) -> Result<Option<Self>, servlin::Error> {
        let Some(cookie_string) = req.cookies.get(SESSION_COOKIE_NAME) else {
            return Ok(None);
        };
        match Self::try_from(cookie_string.as_str()) {
            Ok(cookie) => Ok(Some(cookie)),
            Err(e) => Err(servlin::Error::client_error(servlin::Response::text(
                400,
                format!("error parsing {SESSION_COOKIE_NAME:?} cookie: {e}"),
            ))),
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    #[cfg(feature = "servlin")]
    pub fn to_cookie(&self) -> servlin::Cookie {
        let duration = std::time::Duration::from_secs(100 * 365 * 24 * 3600);
        servlin::Cookie::new(
            SESSION_COOKIE_NAME,
            servlin::AsciiString::try_from(format!("{}-{}", self.id.inner(), self.secret.inner()))
                .unwrap(),
        )
        .with_secure(false) // So we can test at http://127.0.0.1/.
        .with_max_age(duration)
        .with_expires(std::time::SystemTime::now() + duration)
        .with_path("/")
    }
}
impl TryFrom<&str> for SessionCookie {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let err = || {
            format!(
                "invalid SessionCookie: {}",
                escape_and_elide(s.as_bytes(), 100)
            )
        };
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(err());
        }
        let id_string: i64 = parts[0].parse().map_err(|_| err())?;
        let secret_string: i64 = parts[1].parse().map_err(|_| err())?;
        let id = Id::try_from(id_string)?;
        let secret = Secret::try_from(secret_string)?;
        Ok(Self::new(id, secret))
    }
}
impl FromStr for SessionCookie {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
#[cfg(feature = "servlin")]
impl TryFrom<&servlin::Request> for SessionCookie {
    type Error = servlin::Error;

    fn try_from(req: &servlin::Request) -> Result<Self, Self::Error> {
        let cookie_string = req.cookies.get(SESSION_COOKIE_NAME).ok_or_else(|| {
            servlin::Error::client_error(servlin::Response::text(
                400,
                format!("request is missing {SESSION_COOKIE_NAME:?} cookie"),
            ))
        })?;
        Self::try_from(cookie_string.as_str()).map_err(|e| {
            servlin::Error::client_error(servlin::Response::text(
                400,
                format!("error parsing {SESSION_COOKIE_NAME:?} cookie: {e}"),
            ))
        })
    }
}
impl Debug for SessionCookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SessionCookie(id={},secret=...)", self.id.inner())
    }
}
#[allow(clippy::derived_hash_with_manual_eq)]
impl Hash for SessionCookie {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
        // TODONT: Do not hash `secret`.  This should prevent data structure operations
        // from leaking it via timing.
    }
}
