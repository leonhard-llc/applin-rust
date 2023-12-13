use nanorand::{ChaCha, Rng};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Cryptographically-secure Pseudo-Random Number Generator
pub static CSPRNG: Lazy<Mutex<ChaCha<20>>> = Lazy::new(|| Mutex::new(ChaCha::new()));

#[must_use]
pub fn csprng_random_positive_nonzero_i64() -> i64 {
    CSPRNG
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .generate_range(1_i64..i64::MAX)
}
