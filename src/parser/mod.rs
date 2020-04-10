use crate::lexer::{Lexer, Token, LexerIter};
use crate::value::Value;
use crate::util::Span;
use std::collections::HashMap;

mod error;
pub use error::ParseError;

pub type ParseResult<'a> = Result<Value, ParseError<'a>>;

pub struct Parser<'a> {
	inner: LexerIter<'a>,
	pos: Span
}

impl<'a> Parser<'a> {
	pub fn new(source: &'a str) -> Parser<'a> {
		let inner = Lexer::new(source).into_iter();
		let pos = Span::default();
		Parser { inner, pos }
	}

	pub fn parse_all(&'a mut self) -> ParseResult {
		self.parse()
	}

	pub fn parse(&mut self) -> ParseResult<'a> {
		let token: Token = self.inner.next().ok_or(ParseError::UnexpectedEndOfFile(self.pos))?;
		self.pos = token.span;
		
		if token == '{' {
			self.parse_object(token)
		} else if token == '[' {
			self.parse_array(token)
		} else {
			self.parse_other(token)
		}
	}

	fn parse_object(&mut self, token: Token) -> ParseResult<'a> {
		let mut list = HashMap::default();
		let mut last_token = token;

		while let Some(token) = self.inner.next() {
			let key = self.parse_other(token)?;

			if let Some(token) = self.inner.next() {
				if token != ':' {
					return Err(ParseError::UnexpectedToken(token, Token::test_symbol(":")));
				}
			}

			let value = self.parse()?;
			list.insert(key, value);

			let token = self.inner.next().ok_or(ParseError::UnexpectedEndOfFile(self.pos))?;
			last_token = token;

			if token == '}' {
				break;
			} else if token != ',' {
				return Err(ParseError::UnexpectedToken(token, Token::test_symbol(",")));
			}
		}

		let span = Span::from_span(token.span, last_token.span);
		Ok(Value::new_object(span, list))
	}

	fn parse_array(&mut self, token: Token) -> ParseResult<'a> {
		let mut list = Vec::default();
		
		list.push(self.parse()?);
		let mut last_token = token;
		
		while let Some(token) = self.inner.next() {
			last_token = token;
			
			if token == ']' {
				break;
			} else if token != ',' {
				return Err(ParseError::UnexpectedToken(token, Token::test_symbol(",")));
			}
			list.push(self.parse()?);
		}
		
		let span = Span::from_span(token.span, last_token.span);
		Ok(Value::new_array(span, list))
	}

	fn parse_other(&self, token: Token<'a>) -> ParseResult<'a> {
		Option::<Value>::from(token).ok_or(ParseError::InvalidToken(token))
	}
}

#[macro_export]
macro_rules! hashmap {
	($($x:expr => $y:expr)* ) => {
		{
			let mut m = HashMap::new();
			$(m.insert($x, $y);)*
			m
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn try_parse_stuff() {
		let content = r#"
		{
			"foo": 42,
			"bar": 0
		}
		"#;

		let mut parser = Parser::new(content);
		let result = parser.parse_all().unwrap();

		let map = hashmap! {
			Value::test_string("foo") => Value::test_number(42.0)
			Value::test_string("bar") => Value::test_number(0.0)
		};

		assert_eq!(result, Value::test_object(map));
	}


	#[test]
	fn try_parse_nested_stuff() {
		let content = r#"
		{
			"foo": 42,
			"bar": {
				"a": [1, 2,3 ],
				"b": false
			},
			"baz": null
		}
		"#;

		let mut parser = Parser::new(content);
		let result = parser.parse_all().unwrap();

		assert_eq!(result, Value::test_object(hashmap! {
			Value::test_string("foo") => Value::test_number(42.0)
			Value::test_string("bar") => Value::test_object(hashmap! {
				Value::test_string("a") => Value::test_array(vec![Value::test_number(1.0), Value::test_number(2.0), Value::test_number(3.0)])
				Value::test_string("b") => Value::test_bool(false)
			})
			Value::test_string("baz") => Value::test_null()
		}));
	}
}