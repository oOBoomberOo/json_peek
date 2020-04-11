#![deny(unsafe_code)]
#![deny(clippy::all)]
#![feature(const_fn)]
#![feature(type_alias_impl_trait)]
#![feature(const_saturating_int_methods)]

//! `json-peek` is an amature json parser library design for my specific need.
//! Don't expect this to be efficient or perfect.
//!
//! This library will parse json file while also maintain positional information which work really well with [codespan-reporting](https://docs.rs/codespan-reporting/)
//!
//! ```
//! # use json_peek::util;
//! # use json_peek::value::prelude::*;
//! let content = r#"
//! {
//!     "display": {
//!         "title": "Installed Datapacks",
//!         "description": "",
//!         "icon": {
//!             "item": "minecraft:knowledge_book"
//!         },
//!         "background": "minecraft:textures/block/gray_concrete.png",
//!         "show_toast": false,
//!         "announce_to_chat": false
//!     },
//!     "criteria": {
//!         "trigger": {
//!              "trigger": "minecraft:tick"
//!         }
//!     }
//! }
//! "#;
//!
//! let parse_result = util::from_str(content).expect("Invalid json");
//! let title = parse_result.get("display").get("title").expect("Title doesn't exist");
//!
//! assert_eq!(title, "Installed Datapacks");
//! ```

/// Lexer module which parse string into usable [Token](lexer/struct.Token.html)
pub mod lexer;
/// Parser module which handling interpreting [Token](lexer/struct.Token.html) into JSON AST
pub mod parser;
/// Utility module
pub mod util;
pub mod value;

pub use value::{Value, Indexable, Keyable};
pub use parser::Parser;
pub use lexer::{Lexer, Token};