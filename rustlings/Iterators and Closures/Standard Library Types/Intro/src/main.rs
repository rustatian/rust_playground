fn main() {
    let v1 = vec![1, 2, 3];
    let v1_clone = v1.clone();
    let v1_iter = v1_clone.iter();

    let mut v2_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("{}", v2_iter.next().unwrap());
}
