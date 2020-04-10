use crate::lexer::{Lexer, Token, LexerIter};
use crate::value::Value;
use crate::util::Span;
use std::collections::HashMap;

pub struct Parser<'a> {
	inner: LexerIter<'a>
}

impl<'a> Parser<'a> {
	pub fn new(source: &'a str) -> Parser<'a> {
		let inner = Lexer::new(source).into_iter();
		Parser { inner }
	}

	pub fn parse(mut self) -> Option<Value> {
		self.next()
	}

	fn parse_object(&mut self, token: Token) -> Option<Value> {
		let mut list = HashMap::default();
		let mut last_token = token;

		while let Some(token) = self.inner.next() {
			println!("{:?}", token);
			let key = Option::<Value>::from(token)?;

			if let Some(token) = self.inner.next() {
				if token != ':' {
					// TODO: Cause some error
				}
			}

			let value = self.next()?;
			list.insert(key, value);

			let token = self.inner.next()?;
			last_token = token;

			if token == '}' {
				break;
			} else if token != ',' {
				// TODO: Cause some error
			}
		}

		let span = Span::from_span(token.span, last_token.span);
		Some(Value::new_object(span, list))
	}

	fn parse_array(&mut self, token: Token) -> Option<Value> {
		let mut list = Vec::default();
		
		list.push(self.next()?);
		let mut last_token = token;
		
		while let Some(token) = self.inner.next() {
			last_token = token;
			
			if token == ']' {
				break;
			} else if token != ',' {
				// TODO: Cause some error here
			}
			list.push(self.next()?);
		}
		
		let span = Span::from_span(token.span, last_token.span);
		Some(Value::new_array(span, list))
	}

	fn parse_other(&mut self, token: Token) -> Option<Value> {
		Option::<Value>::from(token)
	}
}

impl<'a> Iterator for Parser<'a> {
	type Item = Value;

	fn next(&mut self) -> Option<Self::Item> {
		let token: Token = self.inner.next()?;
		
		if token == '{' {
			self.parse_object(token)
		} else if token == '[' {
			self.parse_array(token)
		} else {
			self.parse_other(token)
		}
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

		let parser = Parser::new(content);
		let result = parser.parse().unwrap();

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

		let parser = Parser::new(content);
		let result = parser.parse().unwrap();

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