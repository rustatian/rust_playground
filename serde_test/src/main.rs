use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufReader};
use std::io;

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
            some_field2: "tupo_field".to_string(),
        }
    }
}


fn main() -> Result<(), io::Error> {
    let a = Move::new(10);
    // write
    {
        let serialized = serde_json::to_string(&a)?;

        let file = File::create("move.json");
        file.unwrap().write_all(serialized.as_bytes())?;
        println!("serialized = {}", serialized);
    }
    {
        let file = File::open("move.json")?;


        let reader = BufReader::new(file);
        let b: Move = serde_json::from_reader(reader)?;
        println!("{} - {}", b.id, b.some_field2);
    }

    Ok(())
}
