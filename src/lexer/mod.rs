use crate::util::Span;
use std::iter::Peekable;
use std::str::CharIndices;

mod token;
pub use token::{Token, TokenKind};

/// Shorthand for Lexer to use
pub type TokenStream<'a> = Peekable<CharIndices<'a>>;

/// A Token Lexer which take string and convert it into a list of [Token](struct.Token.html)
/// 
/// ```
/// # use json_peek::lexer::{Lexer};
/// let content = r#"{"foo": 1}"#;
/// let lexer = Lexer::new(content);
/// 
/// let tokens = lexer.lex();
/// ```
/// 
/// You are probably getting tired of bad documentation like this so here's megumin:
/// ![](https://i.redd.it/nz2ecwpyp3k01.jpg)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lexer<'a> {
	source: &'a str,
}

impl<'a> Lexer<'a> {
	/// Create Lexer to `source`
	///
	/// ```
	/// # use json_peek::lexer::{Lexer, Token};
	/// let content = r#"{ "foo": null, "bar": false }"#;
	/// let lexer = Lexer::new(content);
	/// let mut token = lexer.into_iter();
	///
	/// // Note: `Token::test_*()` function create a token without position information cause I'm too lazy to do it
	/// assert_eq!(token.next(), Some(Token::test_symbol("{")));
	/// assert_eq!(token.next(), Some(Token::test_string("foo")));
	/// assert_eq!(token.next(), Some(Token::test_symbol(":")));
	/// assert_eq!(token.next(), Some(Token::test_identifier("null")));
	/// assert_eq!(token.next(), Some(Token::test_symbol(",")));
	/// assert_eq!(token.next(), Some(Token::test_string("bar")));
	/// assert_eq!(token.next(), Some(Token::test_symbol(":")));
	/// assert_eq!(token.next(), Some(Token::test_identifier("false")));
	/// assert_eq!(token.next(), Some(Token::test_symbol("}")));
	/// assert_eq!(token.next(), None);
	/// ```
	pub const fn new(source: &'a str) -> Lexer<'a> {
		Lexer { source }
	}

	/// Easily create a `Vec` of [Token](struct.Token.html)
	pub fn lex(self) -> Vec<Token<'a>> {
		self.into_iter().collect()
	}
}

impl<'a> IntoIterator for Lexer<'a> {
	type Item = Token<'a>;
	type IntoIter = LexerIter<'a>;

	/// Convert Lexer into iterator of Token.  
	/// See: [LexerIter](struct.LexerIter.html)
	fn into_iter(self) -> Self::IntoIter {
		LexerIter::from(self.source)
	}
}

/// An iterator that generate [Token](struct.Token.html) from given `&str`
/// 
/// You shouldn't manually create this iterator but rather use [Lexer](struct.Lexer.html) instead.
#[derive(Debug, Clone)]
pub struct LexerIter<'a> {
	source: &'a str,
	stream: TokenStream<'a>,
	span: Span,
}

impl<'a> LexerIter<'a> {
	/// Manually create new LexerIter
	pub const fn new(source: &'a str, stream: TokenStream<'a>, span: Span) -> LexerIter<'a> {
		LexerIter {
			source,
			stream,
			span,
		}
	}

	/// Get the current scope that [LexerIter](struct.LexerIter.html) can see
	pub fn value(&self) -> &str {
		&self.source[self.span.range()]
	}

	/// Shorthand for `self.previous_token() == Some(char)`
	pub fn previous_token_is(&self, token: char) -> bool {
		self.previous_token() == Some(token)
	}

	/// Get a `char` before the current token.
	///
	/// Can be `None` if this is the beginning of iterator or this is emoji, I really need to make this compatible with UTF-8 /shrug
	///
	/// ```
	/// # use json_peek::lexer::Lexer;
	/// let mut lexer = Lexer::new("177013").into_iter();
	/// lexer.next();
	/// assert_eq!(lexer.previous_token(), Some('1'));
	/// ```
	pub fn previous_token(&self) -> Option<char> {
		let span = self.span.end_point() - Span::new(1, 1);
		let token = &self.source.get(span.range())?;
		token.chars().next()
	}

	/// Continue lexing until `predicate` return `false`, will *include* the last item with the result
	///
	/// **NOT** an inverse of [lex_while](struct.LexerIter.html#method.lex_while)
	pub fn lex_until(&mut self, predicate: impl Fn(char, &mut LexerIter) -> bool) {
		while let Some(token) = self.stream.peek() {
			let &(index, token) = token;
			self.span.end = index;
			self.stream.next();

			if !predicate(token, self) {
				break;
			}
		}
	}

	/// Continue lexing while `predicate` return `true`, will *exclude* the last item from the result
	///
	/// **NOT** an inverse of [lex_until](struct.LexerIter.html#method.lex_until)
	pub fn lex_while(&mut self, predicate: impl Fn(char, &mut LexerIter) -> bool) {
		while let Some(token) = self.stream.peek() {
			let &(index, token) = token;
			if !predicate(token, self) {
				break;
			}
			self.span.end = index;
			self.stream.next();
		}
	}

	/// Lex string literal
	fn lex_string(&mut self) -> Token<'a> {
		self.lex_until(|token, iter| {
			!token.is_quote() || iter.previous_token_is('\\') || iter.span.is_point()
		});
		Token::new_string(self.span, self.source).trim(1)
	}

	/// Lex identifier literal
	/// 
	/// Can be represent in regex form as `[\d\w_]+`
	fn lex_identifier(&mut self) -> Token<'a> {
		self.lex_while(|x, _| x.is_identifier());
		Token::new_identifier(self.span, self.source)
	}

	/// Lex number literal
	/// 
	/// Can be represent in regex form as `[\d\-.]+`
	fn lex_number(&mut self) -> Token<'a> {
		// NOTE: `is_number()` method will interpret more than `[0-9]` as number
		// Maybe don't use that?
		self.lex_while(|x, _| x.is_number());
		Token::new_number(self.span, self.source)
	}
}

impl<'a> Iterator for LexerIter<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let (index, token) = self.stream.next()?;
		self.span.start = index;
		self.span.end = index;

		let result = if token.is_quote() {
			self.lex_string()
		} else if token.is_number() {
			self.lex_number()
		} else if token.is_identifier() {
			self.lex_identifier()
		} else if token.is_symbol() {
			Token::new_symbol(self.span, self.source)
		} else if token.is_whitespace() {
			self.next()?
		} else {
			Token::new_unknown(self.span, self.source)
		};

		Some(result)
	}
}

impl<'a> From<&'a str> for LexerIter<'a> {
	fn from(source: &'a str) -> Self {
		let span = Span::default();
		let stream = source.char_indices().peekable();
		LexerIter::new(source, stream, span)
	}
}

trait ExtendedChar {
	fn is_number(&self) -> bool;
	fn is_symbol(&self) -> bool;
	fn is_quote(&self) -> bool;
	fn is_identifier(&self) -> bool;
}

impl ExtendedChar for char {
	fn is_number(&self) -> bool {
		self.is_numeric() || *self == '.' || *self == '-'
	}

	fn is_symbol(&self) -> bool {
		*self == '{'
			|| *self == '}'
			|| *self == '['
			|| *self == ']'
			|| *self == ','
			|| *self == ':'
			|| self.is_quote()
	}

	fn is_quote(&self) -> bool {
		*self == '"'
	}

	fn is_identifier(&self) -> bool {
		self.is_alphanumeric() || *self == '_'
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_lexing() {
		let content = r#"{"hello_world"}"#;

		let lexer = Lexer::new(content);
		let mut lexer = lexer.into_iter();

		assert_eq!(lexer.next(), Token::test_symbol("{").into());
		assert_eq!(lexer.next(), Token::test_string("hello_world").into());
		assert_eq!(lexer.next(), Token::test_symbol("}").into());
		assert_eq!(lexer.next(), None);
	}

	#[test]
	fn lexer_with_whitespace() {
		let content = r#"
		{
			"foo": {
				"bar": 42
			}
		}
		"#;

		let mut lexer = Lexer::new(content).into_iter();

		assert_eq!(lexer.next(), Token::test_symbol("{").into());
		assert_eq!(lexer.next(), Token::test_string("foo").into());
		assert_eq!(lexer.next(), Token::test_symbol(":").into());
		assert_eq!(lexer.next(), Token::test_symbol("{").into());
		assert_eq!(lexer.next(), Token::test_string("bar").into());
		assert_eq!(lexer.next(), Token::test_symbol(":").into());
		assert_eq!(lexer.next(), Token::test_number("42").into());
		assert_eq!(lexer.next(), Token::test_symbol("}").into());
		assert_eq!(lexer.next(), Token::test_symbol("}").into());
		assert_eq!(lexer.next(), None);
	}

	#[test]
	fn lexer_with_extra_comma() {
		let content = r#"
		{
			"foo": 1,
			"bar": 2,
		}
		"#;

		let mut lexer = Lexer::new(content).into_iter();

		assert_eq!(lexer.next(), Token::test_symbol("{").into());
		assert_eq!(lexer.next(), Token::test_string("foo").into());
		assert_eq!(lexer.next(), Token::test_symbol(":").into());
		assert_eq!(lexer.next(), Token::test_number("1").into());
		assert_eq!(lexer.next(), Token::test_symbol(",").into());
		assert_eq!(lexer.next(), Token::test_string("bar").into());
		assert_eq!(lexer.next(), Token::test_symbol(":").into());
		assert_eq!(lexer.next(), Token::test_number("2").into());
		assert_eq!(lexer.next(), Token::test_symbol(",").into());
		assert_eq!(lexer.next(), Token::test_symbol("}").into());
		assert_eq!(lexer.next(), None);
	}

	#[test]
	fn use_lex_function() {
		let content = r#"{
			"display": {
				"title": "Boomber",
				"description": "",
				"icon": {
					"item": "minecraft:player_head",
					"nbt": "{SkullOwner: 'Boomber'}"
				},
				"show_toast": false,
				"announce_to_chat": false
			},
			"parent": "global:root",
			"criteria": {
				"trigger": {
					"trigger": "minecraft:tick"
				}
			}
		}"#;

		let tokens = Lexer::new(content).lex();
		
		assert_eq!(tokens, vec![
			Token::test_symbol("{"),
			Token::test_string("display"),
			Token::test_symbol(":"),
			Token::test_symbol("{"),
			Token::test_string("title"),
			Token::test_symbol(":"),
			Token::test_string("Boomber"),
			Token::test_symbol(","),
			Token::test_string("description"),
			Token::test_symbol(":"),
			Token::test_string(""),
			Token::test_symbol(","),
			Token::test_string("icon"),
			Token::test_symbol(":"),
			Token::test_symbol("{"),
			Token::test_string("item"),
			Token::test_symbol(":"),
			Token::test_string("minecraft:player_head"),
			Token::test_symbol(","),
			Token::test_string("nbt"),
			Token::test_symbol(":"),
			Token::test_string("{SkullOwner: 'Boomber'}"),
			Token::test_symbol("}"),
			Token::test_symbol(","),
			Token::test_string("show_toast"),
			Token::test_symbol(":"),
			Token::test_identifier("false"),
			Token::test_symbol(","),
			Token::test_string("announce_to_chat"),
			Token::test_symbol(":"),
			Token::test_identifier("false"),
			Token::test_symbol("}"),
			Token::test_symbol(","),
			Token::test_string("parent"),
			Token::test_symbol(":"),
			Token::test_string("global:root"),
			Token::test_symbol(","),
			Token::test_string("criteria"),
			Token::test_symbol(":"),
			Token::test_symbol("{"),
			Token::test_string("trigger"),
			Token::test_symbol(":"),
			Token::test_symbol("{"),
			Token::test_string("trigger"),
			Token::test_symbol(":"),
			Token::test_string("minecraft:tick"),
			Token::test_symbol("}"),
			Token::test_symbol("}"),
			Token::test_symbol("}"),
		]);
	}
}
