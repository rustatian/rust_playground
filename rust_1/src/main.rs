static mut STASH: &i32 = &128;

fn main() {
    let r = &factorial(6);

    assert_eq!(r + &1009, 1129);
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn factorial(n: usize) -> usize {
    (1..n).fold(1, |a, b| a * b)
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
