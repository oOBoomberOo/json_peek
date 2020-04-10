use json_parser::parser::Parser;

fn main() {
	let content = r#"
	{
		"foo": 1,
		"bar": 2
	}
	"#;

	let mut parser = Parser::new(content);
	let result = parser.parse_all();

	match result {
		Ok(v) => println!("{:#?}", v),
		Err(e) => println!("{}", e)
	}
}