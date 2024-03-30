use core::fmt;

pub enum Kind {
    String,
    Array,
}

impl Kind {
    pub fn as_str(&self) -> &str {
        match self {
            Kind::String => "+",
            Kind::Array => "*",
        }
    }
}
