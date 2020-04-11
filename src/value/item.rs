use super::Value;
use crate::lexer::Token;
use crate::util::Span;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

pub type Object = Item<HashMap<Literal, Value>>;
pub type Array = Item<Vec<Value>>;
pub type Number = Item<String>;
pub type Literal = Item<String>;
pub type Bool = Item<bool>;
pub type Null = Item<()>;

#[derive(Debug, Clone, Copy)]
pub struct Item<T> {
	pub span: Span,
	pub value: T,
}

impl<T> Item<T> {
	pub const fn new(span: Span, value: T) -> Item<T> {
		Item { span, value }
	}

	pub fn span(&self) -> Span {
		self.span
	}
}

impl Literal {
	pub fn new_literal(value: impl Into<String>) -> Self {
		let value = value.into();
		let span = Span::default();
		Item { span, value }
	}
}

impl Number {
	pub fn new_number(value: String) -> Self {
		let span = Span::default();
		Item { span, value }
	}
}

impl Array {
	pub fn new_array(value: Vec<Value>) -> Self {
		let span = Span::default();
		Item { span, value }
	}
}

impl Bool {
	pub fn new_bool(value: bool) -> Self {
		let span = Span::default();
		Item { span, value }
	}
}

impl Null {
	pub fn new_null() -> Self {
		let span = Span::default();
		Item { span, value: () }
	}
}

impl Object {
	pub fn new_object(value: HashMap<Literal, Value>) -> Self {
		let span = Span::default();
		Item { span, value }
	}
}

impl<T: PartialEq> PartialEq for Item<T> {
	fn eq(&self, other: &Item<T>) -> bool {
		self.value == other.value
	}
}

impl<T: PartialEq> Eq for Item<T> {}

impl<T: Hash> Hash for Item<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state);
	}
}

impl From<&str> for Literal {
	fn from(value: &str) -> Self {
		Literal::new_literal(value)
	}
}

impl From<Token<'_>> for Literal {
	fn from(token: Token) -> Self {
		Item::new(token.span, token.value().to_owned())
	}
}