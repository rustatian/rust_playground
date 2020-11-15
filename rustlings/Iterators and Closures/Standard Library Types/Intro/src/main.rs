fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();


    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2_iter = v1.into_iter();
    for mut val in v2_iter {
        val = 4;
    }
    println!("{:?}", "fasd");
}
