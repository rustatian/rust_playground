fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_len(s1);
    println!("The length of {} is {}.", s2, len);
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
