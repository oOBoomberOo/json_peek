use crate::lexer::{Token, TokenKind};
use crate::util::Span;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod item;
use item::Item;

#[derive(Debug, Clone)]
pub enum Value {
	Object(Item<HashMap<Value, Value>>),
	Array(Item<Vec<Value>>),
	String(Item<String>),
	Number(Item<f64>),
	Null(Item<()>),
	Bool(Item<bool>),
}

impl Value {
	pub fn span(&self) -> Span {
		match self {
			Self::Object(item) => item.span(),
			Self::Array(item) => item.span(),
			Self::String(item) => item.span(),
			Self::Number(item) => item.span(),
			Self::Null(item) => item.span(),
			Self::Bool(item) => item.span(),
		}
	}

	pub const fn new_object(span: Span, value: HashMap<Value, Value>) -> Value {
		let item = Item::new(span, value);
		Value::Object(item)
	}

	pub const fn new_array(span: Span, value: Vec<Value>) -> Value {
		let item = Item::new(span, value);
		Value::Array(item)
	}

	pub fn new_string(span: Span, value: impl Into<String>) -> Value {
		let value = value.into();
		let item = Item::new(span, value);
		Value::String(item)
	}

	pub fn new_number(span: Span, value: impl Into<f64>) -> Value {
		let value = value.into();
		let item = Item::new(span, value);
		Value::Number(item)
	}

	pub const fn new_null(span: Span) -> Value {
		let item = Item::new(span, ());
		Value::Null(item)
	}

	pub const fn new_bool(span: Span, value: bool) -> Value {
		let item = Item::new(span, value);
		Value::Bool(item)
	}

	#[cfg(test)]
	pub const fn test_object(value: HashMap<Value, Value>) -> Value {
		Value::new_object(Span::test(), value)
	}

	#[cfg(test)]
	pub const fn test_array(value: Vec<Value>) -> Value {
		Value::new_array(Span::test(), value)
	}

	#[cfg(test)]
	pub fn test_string(value: &str) -> Value {
		Value::new_string(Span::test(), value)
	}

	#[cfg(test)]
	pub fn test_number(value: f64) -> Value {
		Value::new_number(Span::test(), value)
	}

	#[cfg(test)]
	pub const fn test_null() -> Value {
		Value::new_null(Span::test())
	}

	#[cfg(test)]
	pub const fn test_bool(value: bool) -> Value {
		Value::new_bool(Span::test(), value)
	}
}

impl<'a> From<Token<'a>> for Option<Value> {
	fn from(token: Token) -> Option<Value> {
		let span = token.span;
		match token.kind {
			TokenKind::Number => Some(Value::new_number(span, token)),
			TokenKind::String => Some(Value::new_string(span, token)),
			TokenKind::Identifier => match token.value() {
				"false" => Some(Value::new_bool(span, false)),
				"true" => Some(Value::new_bool(span, true)),
				"null" => Some(Value::new_null(span)),
				_ => None,
			},
			_ => None,
		}
	}
}

impl Hash for Value {
	fn hash<H: Hasher>(&self, state: &mut H) {
		if let Self::String(item) = self {
			item.hash(state);
		} else {
			panic!("Object key must be `Value::String`");
		}
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Value) -> bool {
		match (self, other) {
			(Self::Object(left), Self::Object(right)) => left == right,
			(Self::Array(left), Self::Array(right)) => left == right,
			(Self::String(left), Self::String(right)) => left == right,
			(Self::Number(left), Self::Number(right)) => left == right,
			(Self::Bool(left), Self::Bool(right)) => left == right,
			(Self::Null(left), Self::Null(right)) => left == right,
			_ => false,
		}
	}
}

impl Eq for Value {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_span() {
		let value = Value::new_string(Span::new(5, 18), "Hello, World!");
		assert_eq!(value.span(), Span::new(5, 18));
	}
}
