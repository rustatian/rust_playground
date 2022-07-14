#[link(name = "X11")]
extern {
    fn XOpenDisplay(_: usize) -> usize;
}

fn main() {
    println!("{:?}", unsafe { XOpenDisplay(0) })
}
