use json_peek::util;

fn main() {
	let content = r#"
	{
		"foo": 1,
		"bar": 2
	}
	"#;
	let result = util::from_str(content);

	match result {
		Ok(v) => println!("{:#?}", v),
		Err(e) => println!("{}", e),
	}
}
