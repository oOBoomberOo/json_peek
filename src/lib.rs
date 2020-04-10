#![deny(unsafe_code)]
#![deny(clippy::all)]

pub mod lexer;
pub mod parser;
pub mod util;
mod value;

pub use value::Value;
