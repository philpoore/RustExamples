use std::collections::HashMap;


fn section(title: &str) {
    println!("\n{}\n{}", title, "-".repeat(20))
}

fn main() {
    println!("Simple rust programming examples");

    // Expressions
    section("Expressions");
    let a = 15;
    let b = 45;
    println!("a = {}, b = {}, a * b = {}", a, b, a + b);
    println!("a = {}, b = {}, a * b = {}", a, b, a - b);
    println!("a = {}, b = {}, a * b = {}", a, b, a * b);
    println!("a = {}, b = {}, a * b = {}", a, b, a / b);

    // Strings
    section("Strings");
    let hello_world = "Hello world";
    println!("{}", hello_world);

    let hello = "Hello";
    let world = "world";
    let combined_hello = format!("{} {}", hello, world);
    println!("{}", combined_hello);

    // Mutable
    section("Mutable");
    let mut a = 20;
    println!("a = {}", a);
    a = 40;
    println!("a = {}", a);

    let mut hello = "hello";
    println!("Before: {}", hello);
    hello = "other";
    println!("After: {}", hello);


    // Loops
    section("Loops");

    // Basic loop
    let mut count = 0;
    loop {
        if count == 5 {
            println!("All done looping");
            break;
        }

        println!("count is {}", count);

        count += 1;
    }

    // Alternative loop using while
    let mut count2 = 0;
    while count2 < 5 {
        println!("count2 is {}", count);
        count2 += 1;
    }
    println!("All done looping2");

    // Loop using for in range
    for count3 in 0..5 {
        println!("count3 is {}", count3);
    }
    println!("All done looping3");

    // Vectors
    section("Vectors");
    let mut a = vec![];
    a.push("one");
    a.push("two");
    a.push("three");
    println!("a has length {}", a.len());
    println!("a is now : {:?}", a);
    
    for item in a {
        println!("item = {}", item);
    }

    // Functions
    section("Functions");
    let add = |a: u8, b: u8| -> u8 {
        a + b
    };
    println!("add(34, 59) = {}", add(34, 59));

    let min_max = |v : Vec<i64>| -> (i64, i64) {
        let mut min = i64::MAX;
        let mut max = i64::MIN;
        for item in v {
            if item < min { min = item; }
            if item > max { max = item; }
        }
        (min, max)
    };
    let some_numbers = vec![1,6,3,1,4,7,8,234,4,23,23,3,4,2,-12,-3];
    println!("someNumbers = {:?}", some_numbers);
    println!("minMax = {:?}", min_max(some_numbers));

    
    section("HashMap");
    let mut contacts = HashMap::new();
    contacts.insert("alice", "000-000-111");
    contacts.insert("bob", "000-000-222");
    contacts.insert("clare", "000-000-333");

    println!("I have {} contacts", contacts.len());
    if contacts.contains_key("bob") {
        println!("I know bob! His number is {}", contacts.get("bob").unwrap());
    }
    println!("My contacts: {:?}", contacts);
    for contact in contacts {
        println!("{} = {}", contact.0, contact.1);
    }


    section("Structs");
    #[derive(Debug)]
    struct User {
        id: u64,
        name: String,
        likes: Vec<String>,
    };

    let alice = User{
        id: 1,
        name: "Alice".to_string(),
        likes: vec![
            "archary".to_string(),
            "avacardos".to_string()
        ]
    };

    println!("Alice = {:?}", alice);
}
