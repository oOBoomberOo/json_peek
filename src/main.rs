use json_parser::parser::Parser;

fn main() {
	let content = r#"
	{
		"foo": 1,
		"bar": 2
	}
	"#;

	let parser = Parser::new(content);
	let result = parser.parse();
	println!("{:#?}", result);
}