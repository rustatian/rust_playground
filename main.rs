fn main() {
    let mut x = Some(2);

    match x.take() {
        Some(2) => { println!("2") }
        None => { println!("None") }
        _ => { println!("error") }
    };

    assert_eq!(x, Some(2));
}
