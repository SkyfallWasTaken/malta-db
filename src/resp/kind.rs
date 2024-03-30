pub enum Kind {
    SimpleString,
    BulkString,
    Array,
}

impl Kind {
    pub fn as_str(&self) -> &str {
        match self {
            Kind::SimpleString => "+",
            Kind::BulkString => "$",
            Kind::Array => "*",
        }
    }
}
