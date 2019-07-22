use crate::mutex::mutex_f;

mod mutex;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();
    let events = numbers.filter(|x| *x % 2 == 0);
    let events_square = events.clone().map(|x| x * x);

    mutex_f();
}
