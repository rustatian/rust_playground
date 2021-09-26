use std::io::Write;

fn main() {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut Write = &mut buf;
}

trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_set(&self, x: i32, y: i32) -> bool;
}
