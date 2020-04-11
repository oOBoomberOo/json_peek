use crate::util::Span;
use std::fmt;
use std::str::FromStr;

/// A single point of string that represent a useful symbol for [Parser](../parser/struct.Parser.html) to use
#[derive(Clone, Copy, Eq)]
pub struct Token<'a> {
	pub span: Span,
	source: &'a str,
	pub kind: TokenKind,
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

	/// Get a string reprsentation of what `Token` is pointing to
	pub fn value(&self) -> &str {
		&self.source.get(self.span.range()).unwrap_or_default()
	}

	/// See [Span::trim()](../util/struct.Span.html#method.trim)
	/// 
	/// ```
	/// # use json_peek::lexer::Token;
	/// let token = Token::test_identifier("[hello_world!]");
	/// 
	/// assert_eq!(token.trim(1), Token::test_identifier("hello_world!"));
	/// ```
	/// ```
	/// # use json_peek::lexer::Token;
	/// let token = Token::test_identifier("()");
	/// 
	/// assert_eq!(token.trim(1), Token::test_identifier(""));
	/// ```
	pub fn trim(&self, offset: usize) -> Token<'a> {
		let span = self.span.trim(offset);
		Token::new(span, self.source, self.kind)
	}

	/// Create "test" Token which is use inside a unit test to easily create a mock Token
	pub const fn test(value: &'a str, kind: TokenKind) -> Token<'a> {
		let span = Span::new(0, value.len().saturating_sub(1));
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
		write!(f, "{:?} ({:?})", self.value(), self.span)
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

/// Represent each type of [Token](struct.Token.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	/// Token is made surrounded by quote (`"`)
	String,
	/// Token is made entirely out of number and related symbol
	Number,
	/// Token is a symbol, usually is one character in length
	Symbol,
	/// Token is made up of numbers, alphabet and underscore (`_`) but isn't surrounded by quote ('"')
	Identifier,
	/// Lexer wasn't able to determine the type of this Token
	Unknown,
}
