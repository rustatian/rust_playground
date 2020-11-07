fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    // word here is invalid

    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}