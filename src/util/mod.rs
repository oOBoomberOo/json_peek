use crate::parser::{ParseResult, Parser};
mod span;
pub use span::Span;

/// Parse string to JSON tree
///
/// ```
/// # use json_peek::value::Value;
/// # use json_peek::util;
/// let content = r#"[1, 2, 3]"#;
///
/// let result = util::from_str(content).expect("Invalid JSON");
/// assert_eq!(result, Value::test_array(
///     vec![
///         Value::test_number("1"),
///         Value::test_number("2"),
///         Value::test_number("3")
///     ])
/// );
/// ```
pub fn from_str(content: &str) -> ParseResult {
	let mut parser = Parser::new(content);
	parser.parse()
}

#[cfg(test)]
mod tests {
	#[test]
	fn from_str() {
		use crate::value::Value;
		let content = r#"["megumin", "kazuma", "aqua"]"#;

		let result = super::from_str(content).expect("Invalid JSON");
		assert_eq!(
			result,
			Value::test_array(vec![
				Value::test_string("megumin"),
				Value::test_string("kazuma"),
				Value::test_string("aqua"),
			])
		);
	}
}
