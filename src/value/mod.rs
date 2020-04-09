use crate::util::Span;
use std::collections::HashMap;

pub enum Value {
	Object { span: Span, value: HashMap<Value, Value> },
	Array { span: Span, value: Vec<Value> },
	String { span: Span, value: String },
	Number { span: Span, value: f64 },
	Null { span: Span },
	Bool { span: Span, value: bool }
}

impl Value {
	pub fn span(&self) -> Span {
		match self {
			Self::Object { span, .. }
			| Self::Array { span, .. }
			| Self::String { span, .. }
			| Self::Number { span, .. }
			| Self::Null { span }
			| Self::Bool { span, .. } => *span,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_span() {
		let value = Value::String { span: Span::new(5, 18), value: "Hello, World!".to_owned() };
		assert_eq!(value.span(), Span::new(5, 18));
	}
}