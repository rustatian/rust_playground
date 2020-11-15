fn main() {
    let v1 = vec![1, 2, 3];
    let c: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(c, vec![2, 3, 4]);
}

struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}