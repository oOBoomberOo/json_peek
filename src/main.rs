use json_parser::lexer::Lexer;

fn main() {
	let content = r#"
	{
		"foo": {
			"bar": 42
		}
	}
	"#;

	let lexer = Lexer::new(content);
	let result: Vec<_> = lexer.into_iter().collect();
	println!("{:#?}", result);
}