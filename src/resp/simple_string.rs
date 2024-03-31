use core::fmt;
use core::str::FromStr;

use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;

use super::{kind::Kind, Error, Resp};

const BANNED_PATTERNS: [&str; 2] = ["\r", "\n"];

static BANNED_PATTERN_MATCHER: Lazy<AhoCorasick> =
    Lazy::new(|| AhoCorasick::new(BANNED_PATTERNS).unwrap());

/// Simple strings are encoded as a plus (`+`) character, followed by a string.
/// The string mustn't contain a CR (`\r`) or LF (`\n`) character and is terminated by CRLF (i.e., `\r\n`).
///
/// Simple strings transmit short, non-binary strings with minimal overhead.
/// For example, many Redis commands reply with just "OK" on success. The encoding of this
/// Simple String is the following 5 bytes:
///
/// ```text
/// +OK\r\n
/// ```
///
/// To send binary strings, use bulk strings instead.
pub struct SimpleString(String);

impl Resp for SimpleString {}

impl SimpleString {
    /// Attempts to create a new simple string.
    ///
    /// This function will return an error if the value contains banned patterns (`\r` or `\n`).
    ///
    /// ## Example
    /// ```rust
    /// use resp::SimpleString;
    ///
    /// let simple_string = SimpleString::try_new("Hello, World!".to_string()).unwrap();
    /// ```
    ///
    /// ## Errors
    /// This function will return `Error::BannedPatterns` if the value contains banned patterns.

    pub fn try_new<'a>(value: String) -> Result<Self, Error<'a>> {
        Self::is_valid(&value)?;
        Ok(Self(value))
    }

    /// Gets the value of the simple string, as a reference.
    ///
    /// ## Example
    /// ```rust
    /// use resp::SimpleString;
    ///
    /// let simple_string = SimpleString::try_new("Hello, World!".to_string()).unwrap();
    /// println!("{}", simple_string.value()); // "Hello, World!"
    /// ```
    pub const fn value(&self) -> &String {
        &self.0
    }

    /// Checks if the value is valid.
    pub fn is_valid<'a>(value: &String) -> Result<(), Error<'a>> {
        match (*BANNED_PATTERN_MATCHER).find(value) {
            Some(r#match) => Err(Error::BannedPatterns(BANNED_PATTERNS[r#match.pattern()])),
            None => Ok(()),
        }
    }
}

impl fmt::Display for SimpleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{kind}{value}\r\n",
            kind = Kind::SimpleString.as_str(),
            value = self.0
        )
    }
}

impl FromStr for SimpleString {
    type Err = Error<'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s.to_string())
    }
}

impl TryFrom<&str> for SimpleString {
    type Error = Error<'static>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_new(s.to_string())
    }
}

impl TryFrom<String> for SimpleString {
    type Error = Error<'static>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_new(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_string() {
        let value = "Hello, World!".to_string();
        let simple_string = SimpleString::try_new(value.clone()).unwrap();
        assert_eq!(simple_string.value(), &value);
    }

    #[test]
    fn simple_string_correctly_formatted() {
        let value = "Hello, World!".to_string();
        let simple_string = SimpleString::try_new(value.clone()).unwrap();
        assert_eq!(simple_string.to_string(), format!("+{}\r\n", value));
    }

    #[test]
    fn cr_banned() {
        let value = "Hello, World\r!".to_string();
        let result = SimpleString::try_new(value.clone());
        assert!(result.is_err());
    }

    #[test]
    fn lf_banned() {
        let value = "Hello, World\n!".to_string();
        let result = SimpleString::try_new(value.clone());
        assert!(result.is_err());
    }
}
