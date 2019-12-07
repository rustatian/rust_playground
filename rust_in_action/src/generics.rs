use std::ops::Add;

pub fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
