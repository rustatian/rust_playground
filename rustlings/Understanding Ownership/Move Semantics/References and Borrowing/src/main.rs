fn main() {
    let s1 = String::from("hello");
    let len = calculate_len(s1);
    println!("The length of {} is {}.", s1, len);
}

fn calculate_len(s: String) -> usize {
    let length = s.len();
    length
}
