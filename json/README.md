# JSON Examples

Examples of using JSON in rust using the [serde_json](https://github.com/serde-rs/json) library.

## Example

```rust
    #[derive(Serialize, Deserialize, Debug)]
    struct Person {
        name: String,
        age: u16,
        likes: Vec<String>,
    }
    
    let data = r#"
    {
        "name": "alice",
        "age": 22,
        "likes": [
            "food",
            "dancing"
        ]
    }
    "#;

    let alice : Person = serde_json::from_str(data)?;
```