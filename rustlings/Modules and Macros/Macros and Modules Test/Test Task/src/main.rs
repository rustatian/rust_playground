mod my_macro {
    #[macro_export]
    macro_rules! my_macro {
        ($x:expr) => {
            "Hello world!";
        };
    }
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
