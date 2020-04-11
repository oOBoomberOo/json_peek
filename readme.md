# Json-Peek

Json Peek is an amature JSON parser written for my specific need. Don't expect this to be efficient or perfect.

This library is designed to parse JSON while also keeping track of position information which can then be use inside [codespan-reporting](https://github.com/brendanzab/codespan) crate.

```rust
use json_peek::util;
use json_peek::value::prelude::*;
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

println!("Title is located at: {}", title.span);
```
