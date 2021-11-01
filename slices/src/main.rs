fn main() {
    let mut s = String::from("ivo belitts");

    let first_word = first_word(&s);

    s.clear();

    println!("{}", first_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
