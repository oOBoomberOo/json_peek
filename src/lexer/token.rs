use crate::util::Span;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
	pub span: Span,
	source: &'a str,
	pub kind: TokenKind
}

impl<'a> Token<'a> {
	pub const fn new(span: Span, source: &'a str, kind: TokenKind) -> Token<'a> {
		Token { span, source, kind }
	}

	pub const fn new_number(span: Span, source: &'a str) -> Token<'a> {
		Token::new(span, source, TokenKind::Number)
	}

	pub const fn new_symbol(span: Span, source: &'a str) -> Token<'a> {
		Token::new(span, source, TokenKind::Symbol)
	}

	pub const fn new_identifier(span: Span, source: &'a str) -> Token<'a> {
		Token::new(span, source, TokenKind::Identifier)
	}

	pub const fn new_unknown(span: Span, source: &'a str) -> Token<'a> {
		Token::new(span, source, TokenKind::Unknown)
	}

	pub fn value(&self) -> &str {
		&self.source[self.span.range()]
	}
}

impl fmt::Debug for Token<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} ({:?})", self.value(), self.span.range())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	Number,
	Symbol,
	Identifier,
	Unknown
}