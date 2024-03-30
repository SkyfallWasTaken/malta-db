mod array;
mod simple_string;

mod kind;

#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    #[error("the value contains banned patterns: {0}")]
    BannedPatterns(&'a str),
}

pub use simple_string::SimpleString;
