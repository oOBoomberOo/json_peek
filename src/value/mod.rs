use crate::{
	lexer::{Token, TokenKind},
	util::Span,
};
use std::collections::HashMap;

mod item;
pub use item::*;

#[derive(Debug, Clone)]
pub enum Value {
	Object(Object),
	Array(Array),
	Literal(Literal),
	Number(Number),
	Bool(Bool),
	Null(Null),
}

impl Value {
	pub fn null() -> Value {
		Self::new_null(Span::new(0, 0))
	}

	pub fn is_object(&self) -> bool {
		match self {
			Self::Object(_) => true,
			_ => false,
		}
	}

	pub fn is_array(&self) -> bool {
		match self {
			Value::Array(_) => true,
			_ => false,
		}
	}

	pub fn is_string(&self) -> bool {
		match self {
			Value::Literal(_) => true,
			_ => false,
		}
	}

	pub fn is_number(&self) -> bool {
		match self {
			Value::Number(_) => true,
			_ => false,
		}
	}

	pub fn is_null(&self) -> bool {
		match self {
			Value::Null(_) => true,
			_ => false,
		}
	}

	pub fn is_bool(&self) -> bool {
		match self {
			Value::Bool(_) => true,
			_ => false,
		}
	}

	pub fn span(&self) -> Span {
		match self {
			Self::Object(item) => item.span(),
			Self::Array(item) => item.span(),
			Self::Literal(item) => item.span(),
			Self::Number(item) => item.span(),
			Self::Null(item) => item.span(),
			Self::Bool(item) => item.span(),
		}
	}

	/// Transform `Value` into `Some(Value)` of itself.
	/// This is helpful for testing since most of the operation return an `Option<Value>`
	pub const fn some(self) -> Option<Self> {
		Some(self)
	}

	pub fn new_object(span: Span, value: HashMap<Literal, Value>) -> Value {
		let item = Item::new(span, value);
		Value::Object(item)
	}

	pub fn new_array(span: Span, value: Vec<Value>) -> Value {
		let item = Item::new(span, value);
		Value::Array(item)
	}

	pub fn new_string(span: Span, value: impl Into<String>) -> Value {
		let value = value.into();
		let item = Item::new(span, value);
		Value::Literal(item)
	}

	pub fn new_number(span: Span, value: impl Into<String>) -> Value {
		let value = value.into();
		let item = Item::new(span, value);
		Value::Number(item)
	}

	pub fn new_null(span: Span) -> Value {
		let item = Item::new(span, ());
		Value::Null(item)
	}

	pub fn new_bool(span: Span, value: bool) -> Value {
		let item = Item::new(span, value);
		Value::Bool(item)
	}

	pub fn test_object(value: HashMap<Literal, Value>) -> Value {
		Value::new_object(Span::test(), value)
	}

	pub fn test_array(value: Vec<Value>) -> Value {
		Value::new_array(Span::test(), value)
	}

	pub fn test_string(value: &str) -> Value {
		Value::new_string(Span::test(), value)
	}

	pub fn test_number(value: &str) -> Value {
		Value::new_number(Span::test(), value)
	}

	pub fn test_null() -> Value {
		Value::new_null(Span::test())
	}

	pub fn test_bool(value: bool) -> Value {
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

impl From<&str> for Value {
	fn from(key: &str) -> Self {
		let span = Span::new(0, key.len() - 1);
		Value::new_string(span, key)
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Value) -> bool {
		match (self, other) {
			(Self::Object(left), Self::Object(right)) => left == right,
			(Self::Array(left), Self::Array(right)) => left == right,
			(Self::Literal(left), Self::Literal(right)) => left == right,
			(Self::Number(left), Self::Number(right)) => left == right,
			(Self::Bool(left), Self::Bool(right)) => left == right,
			(Self::Null(left), Self::Null(right)) => left == right,
			_ => false,
		}
	}
}

impl PartialEq<str> for Value {
	fn eq(&self, other: &str) -> bool {
		match self {
			Self::Literal(item) |
			Self::Number(item) => item.value == other,
			_ => false
		}
	}
}

impl PartialEq<String> for Value {
	fn eq(&self, other: &String) -> bool {
		self.eq(other.as_str())
	}
}

impl PartialEq<&str> for Value {
	fn eq(&self, other: &&str) -> bool {
		self.eq(*other)
	}
}

impl Eq for Value {}

pub trait Indexable<'a, I=usize> {
	type Output;

	fn index(&'a self, index: I) -> Self::Output;
}

pub trait Keyable<'a, K> {
	type Output;
	fn get(&'a self, key: K) -> Self::Output;
}

impl<'a> Indexable<'a> for Value {
	type Output = Option<Value>;

	fn index(&'a self, index: usize) -> Self::Output {
		if let Self::Array(item) = self {
			item.value.get(index).cloned()
		}
		else {
			None
		}
	}
}

impl<'a, K> Keyable<'a, K> for Value where K: Into<Literal> {
	type Output = Option<Value>;

	fn get(&'a self, key: K) -> Self::Output {
		let key = key.into();
		if let Self::Object(item) = self {
			item.value.get(&key).cloned()
		}
		else {
			None
		}
	}
}

impl<'a> Indexable<'a> for Option<Value> {
	type Output = Option<Value>;

	fn index(&self, idx: usize) -> Self::Output {
		self.as_ref().and_then(|x| x.index(idx))
	}
}

impl<'a, K> Keyable<'a, K> for Option<Value> where K: Into<Literal> {
	type Output = Option<Value>;

	fn get(&'a self, key: K) -> Self::Output {
		self.as_ref().and_then(|x| x.get(key))
	}
}

#[doc(hidden)]
pub mod prelude {
	pub use super::{Value, Indexable, Keyable};
}

#[cfg(test)]
mod tests {
	use crate::util;
	use super::*;

	#[test]
	fn get_span() {
		let value = Value::new_string(Span::new(5, 18), "Hello, World!");
		assert_eq!(value.span(), Span::new(5, 18));
	}

	#[test]
	fn get_key() {
		let sample_data = Value::test_object({
			let mut map = HashMap::new();
			map.insert(Literal::new_literal("a"), Value::test_number("1"));
			map.insert(Literal::new_literal("b"), Value::test_number("2"));
			map.insert(Literal::new_literal("c"), Value::test_number("3"));
			map
		});

		assert_eq!(sample_data.get("a"), Value::test_number("1").some());
	}

	#[test]
	fn get_index() {
		let sample_data = Value::test_array(vec![
			Value::test_number("3"),
			Value::test_number("2"),
			Value::test_number("1"),
		]);

		assert_eq!(sample_data.index(0), Value::test_number("3").some());
	}


	#[test]
	fn getter() {
		let content = r#"
		{
		    "display": {
		        "title": "Installed Datapacks",
		        "description": "",
		        "icon": {
		            "item": "minecraft:knowledge_book"
		        },
		        "background": "minecraft:textures/block/gray_concrete.png",
		        "show_toast": false,
		        "announce_to_chat": false
		    },
		    "criteria": {
		        "trigger": {
		             "trigger": "minecraft:tick"
		        }
		    }
		}
		"#;

		let parse_result = util::from_str(content).expect("Invalid json");
		let title = parse_result.get("display").get("title").expect("Title doesn't exist");

		assert_eq!(title, "Installed Datapacks");
	}
}
