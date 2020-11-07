fn main() {
    // put you code here to launch it
    font_of_house::hosting::add_to_waitlist();
}


mod font_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}
    }
}