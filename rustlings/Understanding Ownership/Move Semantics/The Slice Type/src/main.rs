fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s[..]);
    s.clear();
    // word here is invalid

    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    // println!("{}", word); error
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}