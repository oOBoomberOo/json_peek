use crate::parser::{ParseResult, Parser};
mod span;
pub use span::Span;

pub fn from_str(content: &str) -> ParseResult {
	let mut parser = Parser::new(content);
	parser.parse()
}
