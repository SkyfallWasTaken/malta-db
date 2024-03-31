use core::fmt;

use super::{kind::Kind, Resp};

pub struct Array<T: Resp>(pub Vec<T>);

impl<T: Resp> fmt::Display for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{kind}{length}\r\n{values}",
            kind = Kind::Array.as_str(),
            length = self.0.len(),
            values = self
                .0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::SimpleString;

    #[test]
    fn test_array() {
        let array = Array(vec![
            SimpleString::try_new("Hello, World!".to_string()).unwrap(),
            SimpleString::try_new("Goodbye, World!".to_string()).unwrap(),
        ]);

        assert_eq!(
            array.to_string(),
            "*2\r\n+Hello, World!\r\n+Goodbye, World!\r\n"
        );
    }

    #[test]
    fn test_empty_array() {
        let array = Array::<SimpleString>(vec![]);

        assert_eq!(array.to_string(), "*0\r\n");
    }
}
