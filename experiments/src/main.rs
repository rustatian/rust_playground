#[link(name = "")]
extern "C" {
    fn SetOpenOrClosed(a: i32) -> i32;
}

fn main() {
    unsafe {
        SetOpenOrClosed(1);
    }
}

