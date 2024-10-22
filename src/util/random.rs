use nanorand::{ChaCha, Rng};
use std::cell::OnceCell;
use std::sync::Mutex;

/// Cryptographically-secure Pseudo-Random Number Generator
pub static CSPRNG: Mutex<OnceCell<ChaCha<20>>> = Mutex::new(OnceCell::new());

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn csprng_random_positive_nonzero_i64() -> i64 {
    let mut guard = CSPRNG
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    guard.get_or_init(ChaCha::new);
    guard.get_mut().unwrap().generate_range(1_i64..i64::MAX)
}
