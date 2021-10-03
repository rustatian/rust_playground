use std::io::Write;

fn main() {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut Write = &mut buf;

    <str as ToString>::to_string("foo"); // fully qualified method name
    let line = "foo foo";


    let words: Vec<String> = line.split_whitespace().map(ToString::to_string).collect();
}

trait Visible {
    fn hit_set(&self, x: i32, y: i32) -> bool;
}
