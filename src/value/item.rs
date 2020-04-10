use crate::util::Span;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct Item<T> {
	span: Span,
	value: T
}

impl<T> Item<T> {
	pub const fn new(span: Span, value: T) -> Item<T> {
		Item { span, value }
	}

	pub fn span(&self) -> Span {
		self.span
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