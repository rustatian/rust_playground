use crate::threading::shared_state;
use crate::tokio::tokio_start;

mod mutex;
mod chapter1;
mod strings;
mod threading;
mod interior_mutability;
mod tokio;

#[allow(dead_code)]
fn main() {
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(..1).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);

    // A full range clears the vector
    v.drain(..);
    assert_eq!(v, &[]);
}


