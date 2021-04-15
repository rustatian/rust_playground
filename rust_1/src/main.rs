static mut STASH: &i32 = &128;

fn main() {
    let r = &factorial(6);

    assert_eq!(r + &1009, 1129);
}

fn factorial(n: usize) -> usize {
    (1..n).fold(1, |a, b| a * b)
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
