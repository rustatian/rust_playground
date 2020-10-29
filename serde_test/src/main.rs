use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    id: i64,
    some_field: u64,
    some_field2: String,
}

impl Move {
    fn new(id: i64) -> Self {
        Move {
            id,
            some_field: 0,
            some_field2: "".to_string(),
        }
    }
}


fn main() {
    let a = Move::new(10);
    let serializer = serde_json::to_string(&a).unwrap();
    
}
