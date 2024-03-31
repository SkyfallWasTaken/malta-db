use core::fmt;

use super::{Array, BulkString};

pub struct Command(pub Array<BulkString>);

impl Command {
    pub fn name(&self) -> Option<&String> {
        self.0 .0.first().map(|string| &string.0)
    }

    pub fn args(&self) -> &[BulkString] {
        &self.0 .0[1..]
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // As we are using the `Array` type, we can just use its `Display` implementation.
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_correctly_formatted() {
        let command = Command(Array(vec![
            "SET".into(),
            "my_key".into(),
            "my_value".into(),
        ]));

        assert_eq!(
            command.to_string(),
            "*3\r\n$3\r\nSET\r\n$6\r\nmy_key\r\n$8\r\nmy_value\r\n"
        );
    }

    #[test]
    fn command_name_works() {
        let command = Command(Array(vec!["PING".into()]));

        assert_eq!(command.name().unwrap().to_string(), "PING");
    }

    #[test]
    fn command_args_works() {
        let command = Command(Array(vec![
            "SET".into(),
            "my_key".into(),
            "my_value".into(),
        ]));

        assert_eq!(
            command.args(),
            &[BulkString("my_key".into()), BulkString("my_value".into())]
        );
    }
}
