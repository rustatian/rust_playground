fn main() {
    let v1 = vec![1, 2, 3];
    let c: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(c, vec![2, 3, 4]);
}
