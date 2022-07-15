use std::time::Duration;
use chrono::{DateTime, Local};

#[link(name = "X11")]
extern {
    fn XOpenDisplay(_: usize) -> usize;
    fn XStoreName(display: usize, window: usize, name: *const u8) -> i32;
    fn XDefaultRootWindow(display: usize) -> usize;
    fn XCloseDisplay(display: usize) -> i32;
    fn XFlush(display: usize) -> i32;
}

fn main() {
    let disp = unsafe {
        XOpenDisplay(0)
    };
    let root = unsafe {
        XDefaultRootWindow(disp)
    };


    loop {
        let local: DateTime<Local> = Local::now();

        let string = format!("{}\0", local.format("%F %T.%3f"));


        unsafe {
            XStoreName(disp, root, string.as_ptr());
        }
        unsafe {
            XFlush(disp);
        }

        std::thread::sleep(Duration::from_nanos((1e0 / 144.) as u64));
    }
}
