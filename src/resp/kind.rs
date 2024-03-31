pub enum Kind {
    SimpleString,
    BulkString,
    Array,
}

impl Kind {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::SimpleString => "+",
            Self::BulkString => "$",
            Self::Array => "*",
        }
    }
}
