mod random;
mod real32;
pub(crate) use random::*;
pub use real32::*;

/// Convert a byte slice into a string.
/// Includes printable ASCII characters as-is.
/// Converts non-printable or non-ASCII characters to strings like "\n" and "\x19".
///
/// Uses
/// [`core::ascii::escape_default`](https://doc.rust-lang.org/core/ascii/fn.escape_default.html)
/// internally to escape each byte.
///
/// This function is useful for comparing byte slices in tests.
///
/// # Example
/// ```
/// use applin::util::escape_ascii;
/// assert_eq!("abc", escape_ascii(b"abc"));
/// assert_eq!("abc\\n", escape_ascii(b"abc\n"));
/// assert_eq!(
///     "Euro sign: \\xe2\\x82\\xac",
///     escape_ascii("Euro sign: \u{20AC}".as_bytes())
/// );
/// assert_eq!("\\x01\\x02\\x03", escape_ascii(&[1, 2, 3]));
/// ```
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn escape_ascii(input: &[u8]) -> String {
    let mut result = String::new();
    for byte in input {
        for ascii_byte in core::ascii::escape_default(*byte) {
            result.push_str(core::str::from_utf8(&[ascii_byte]).unwrap());
        }
    }
    result
}

/// Convert a byte slice into a string of limited length.
/// Includes printable ASCII characters as-is.
/// Converts non-printable or non-ASCII characters to strings like "\n" and "\x19".
///
/// Uses
/// [`core::ascii::escape_default`](https://doc.rust-lang.org/core/ascii/fn.escape_default.html)
/// internally to escape each byte.
///
/// This function is useful for printing byte slices to logs.
///
/// # Example
/// ```
/// use applin::util::escape_and_elide;
/// assert_eq!("abc", escape_and_elide(b"abc", 5));
/// assert_eq!("ab...", escape_and_elide(b"abcdefg", 5));
/// assert_eq!("abc\\n", escape_and_elide(b"abc\n", 5));
/// assert_eq!(
///     "Euro sign: \\xe2\\x82\\xac",
///     escape_and_elide("Euro sign: \u{20AC}".as_bytes(), 20)
/// );
/// assert_eq!("\\x01\\x02\\x03", escape_and_elide(&[1, 2, 3], 10));
/// ```
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn escape_and_elide(input: &[u8], max_len: usize) -> String {
    if input.len() > max_len {
        escape_ascii(&input[..max_len]) + "..."
    } else {
        escape_ascii(input)
    }
}
