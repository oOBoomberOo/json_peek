use crate::util::Span;
use std::iter::Peekable;
use std::str::CharIndices;

mod token;
use token::Token;

type TokenStream<'a> = Peekable<CharIndices<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lexer<'a> {
	source: &'a str,
}

impl<'a> Lexer<'a> {
	pub const fn new(source: &'a str) -> Lexer<'a> {
		Lexer { source }
	}
}

impl<'a> IntoIterator for Lexer<'a> {
	type Item = Token<'a>;
	type IntoIter = LexerIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		LexerIter::from(self.source)
	}
}

#[derive(Debug, Clone)]
pub struct LexerIter<'a> {
	source: &'a str,
	stream: TokenStream<'a>,
	span: Span,
}

impl<'a> LexerIter<'a> {
	pub const fn new(source: &'a str, stream: TokenStream<'a>, span: Span) -> LexerIter<'a> {
		LexerIter {
			source,
			stream,
			span,
		}
	}

	pub fn value(&self) -> &str {
		&self.source[self.span.range()]
	}

	fn previous_token_is(&self, token: char) -> bool {
		self.previous_token() == Some(token)
	}

	fn previous_token(&self) -> Option<char> {
		let span = self.span.end_point() - Span::new(1, 1);
		let token = &self.source.get(span.range())?;
		token.chars().next()
	}

	fn lex_until(&mut self, f: impl Fn(char, &mut LexerIter) -> bool) {
		while let Some(token) = self.stream.peek() {
			let &(index, token) = token;
			self.span.end = index;
			self.stream.next();

			if !f(token, self) {
				break;
			}
		}
	}

	fn lex_while(&mut self, f: impl Fn(char, &mut LexerIter) -> bool) {
		while let Some(token) = self.stream.peek() {
			let &(index, token) = token;
			if !f(token, self) {
				break;
			}
			self.span.end = index;
			self.stream.next();
		}
	}

	fn lex_string(&mut self) -> Token<'a> {
		self.lex_until(|token, iter| !token.is_quote() || iter.previous_token_is('\\') || iter.span.is_point());
		Token::new_string(self.span, self.source)
	}

	fn lex_identifier(&mut self) -> Token<'a> {
		self.lex_while(|x, _| x.is_identifier());
		Token::new_identifier(self.span, self.source)
	}

	fn lex_number(&mut self) -> Token<'a> {
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
		*self == '{' || *self == '}' || *self == '[' || *self == ']' || *self == ',' || *self == ':' || self.is_quote()
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

		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(0, 0), &content)));
		assert_eq!(lexer.next(), Some(Token::new_string(Span::new(1, 13), &content)));
		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(14, 14), &content)));
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

		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(3, 3), &content)));
		assert_eq!(lexer.next(), Some(Token::new_string(Span::new(8, 12), &content)));
		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(13, 13), &content)));
		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(15, 15), &content)));
		assert_eq!(lexer.next(), Some(Token::new_string(Span::new(21, 25), &content)));
		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(26, 26), &content)));
		assert_eq!(lexer.next(), Some(Token::new_number(Span::new(28, 29), &content)));
		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(34, 34), &content)));
		assert_eq!(lexer.next(), Some(Token::new_symbol(Span::new(38, 38), &content)));
		assert_eq!(lexer.next(), None);
	}
}
