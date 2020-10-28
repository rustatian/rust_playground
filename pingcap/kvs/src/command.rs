pub struct Command {
    id: u64,
    command: String,
}

impl Command {
    fn new() -> Self {
        Command {
            id: 0,
            command: "".to_string(),
        }
    }
}