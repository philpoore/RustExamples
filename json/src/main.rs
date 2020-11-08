use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json;


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u16,
    likes: Vec<String>,
}


fn main() -> Result<()> {
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
    println!("alice = {:?}", alice);

    let data_people = r#"
    [
        {
            "name": "alice",
            "age": 22,
            "likes": [
                "food",
                "dancing"
            ]
        }, {
            "name": "bob",
            "age": 23,
            "likes": [
                "flowers",
                "haikus"
            ]
        }, {
            "name": "clare",
            "age": 24,
            "likes": [
                "mountains",
                "dragons"
            ]
        }
    ]
    "#;

    let people : Vec<Person> = serde_json::from_str(data_people)?;

    for person in people {
        println!("{:?}", person);
    }

    let dave = Person{
        name: "Date".to_string(),
        age: 25,
        likes: vec![
            "strawberries".to_string(),
            "watermelon".to_string()
        ]
    };

    let dave_json = serde_json::to_string(&dave)?;
    println!("dave json = {}", dave_json);

    Ok(())
}
