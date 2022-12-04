use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 5);

    let team_name = String::from("Yellow");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);

    let mut map = HashMap::new();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}", field_name);

    //Check if key exists in hash map. If it exists, the existing value should remain. If it does not exist, add the key and the value currently passed.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    //Update value based on old value
    let text = "hello world wonderful world";

    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", text_map);

    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }
}
