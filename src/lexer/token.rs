use crate::util::Span;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Eq)]
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

	pub const fn new_string(span: Span, source: &'a str) -> Token<'a> {
		Token::new(span, source, TokenKind::String)
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

	pub const fn trim(&self, offset: usize) -> Token<'a> {
		let span = self.span.trim(offset);
		Token::new(span, self.source, self.kind)
	}

	pub const fn test(value: &'a str, kind: TokenKind) -> Token<'a> {
		let span = Span::new(0, value.len() - 1);
		Token::new(span, value, kind)
	}

	pub const fn test_symbol(value: &'a str) -> Token<'a> {
		Token::test(value, TokenKind::Symbol)
	}

	pub const fn test_string(value: &'a str) -> Token<'a> {
		Token::test(value, TokenKind::String)
	}

	pub const fn test_number(value: &'a str) -> Token<'a> {
		Token::test(value, TokenKind::Number)
	}

	pub const fn test_identifier(value: &'a str) -> Token<'a> {
		Token::test(value, TokenKind::Identifier)
	}

	pub const fn test_unknown(value: &'a str) -> Token<'a> {
		Token::test(value, TokenKind::Unknown)
	}
}

impl fmt::Debug for Token<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} ({:?})", self.value(), self.span.range())
	}
}

impl fmt::Display for Token<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value())
	}
}

impl PartialEq for Token<'_> {
	fn eq(&self, other: &Token) -> bool {
		self.value() == other.value() && self.kind == other.kind
	}
}

impl<'a> PartialEq<char> for Token<'a> {
	fn eq(&self, other: &char) -> bool {
		self.value().chars().next().as_ref() == Some(other)
	}
}

impl Into<f64> for Token<'_> {
	fn into(self) -> f64 {
		let value = self.value();
		f64::from_str(value).unwrap_or_default()
	}
}

impl Into<String> for Token<'_> {
	fn into(self) -> String {
		self.value().to_owned()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	String,
	Number,
	Symbol,
	Identifier,
	Unknown
}