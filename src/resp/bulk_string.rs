use core::fmt;
use std::str::FromStr;

use super::{kind::Kind, Error, Resp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkString(pub String);

impl Resp for BulkString {}

impl fmt::Display for BulkString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.0.len();
        write!(
            f,
            "{kind}{len}\r\n{value}\r\n",
            kind = Kind::BulkString.as_str(),
            len = len,
            value = self.0
        )
    }
}

impl FromStr for BulkString {
    type Err = Error<'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for BulkString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bulk_string_correctly_formatted() {
        let value = "Hello, World!".to_string();
        let bulk_string = BulkString(value.clone());
        assert_eq!(
            bulk_string.to_string(),
            format!("${len}\r\n{value}\r\n", len = value.len())
        );
    }

    #[test]
    fn empty_string_correctly_formatted() {
        let bulk_string = BulkString(String::new());
        assert_eq!(bulk_string.to_string(), format!("$0\r\n\r\n"));
    }
}
