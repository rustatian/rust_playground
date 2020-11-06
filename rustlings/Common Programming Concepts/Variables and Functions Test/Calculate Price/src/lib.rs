use std::cmp::Ordering;

pub fn calculateprice(amount: i32) -> i32 {
    match 40.cmp(&amount) {
        Ordering::Greater => {
            amount * 2
        }
        Ordering::Equal => {
            amount * 2
        }
        Ordering::Less => {
            amount
        }
    }
}
