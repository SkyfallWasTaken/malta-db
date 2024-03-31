use core::fmt;

mod array;
mod bulk_string;
mod simple_string;

mod kind;

mod command;

/// Marker trait for RESP types.
pub trait Resp: fmt::Display {}

#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    #[error("the value contains banned patterns: {0}")]
    BannedPatterns(&'a str),
}

pub use array::Array;
pub use bulk_string::BulkString;
pub use simple_string::SimpleString;
